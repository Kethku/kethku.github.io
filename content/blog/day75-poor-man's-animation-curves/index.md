+++
title = "Day75 - Poor Man's Animation Curves"
description = "Building a live reloadable interpolation curve system"
date = 2019-05-02

[extra]
project = "robot"
+++

![Todo](./todo.svg)

Today and yesterday I worked on a system for drawing and using animation curves.
Many modern engines have the ability to specify a curve which dictates some type
of interpolation between values over time. Frequently this system is accompanied
by an editor which allows fine grain manipulation of the curve and live updates
as you edit it.

Unfortunately I don't have the time or desire to build such a system from
scratch, so instead I built a poor-man's version of it which uses animation
curves drawn as pixels in an image and live reloads the curve when the image is
edited. To get this working, I built a mechanism for watching and reloading
textures at runtime in Monogame and a simple interpolation curve system which
turns an image with a line drawn in it into functions which give a middle value
depending on that curve and a given progress.

## Reloadable Resources

Monogame, the graphics library I use to build games and demos in C#, loads
textures via a content pipeline which is run at compile time. This pipeline
enables the Monogame library to compile assets to whatever format is necessary
for a given platform and enables cross-platform support without changing any
game logic.

Unfortunately this makes it difficult to build live-reloading capabilities.
Luckily there is a way around this for certain platforms. Monogame provides a
function for loading images from a memory stream which bypasses the content
pipeline and parses the image format directly.

This combined with the existing infrastructure I built earlier this week for
running the Monogame in a debug mode on windows, means that I can build a system
for reloading assets from the content files directly and watch for changes in
those files to trigger a reload at runtime. Further, in the future I have some
ideas for integrating a scripting language such as Lua. So I decided to build a
general purpose `ReloadingResource` type which watches files and triggers some
abstract functions on reload depending on the currently built target. Then I
built an implementation of this type which loads textures and triggers a
callback whenever the texture is loaded.

{% code(lang="c#") %}
public abstract class ReloadingResource : IDisposable {
    ResourceReloader resourceReloader;

    // Name of the resource to use in the content directory
    protected string resourceName;
    // Path under the Shared/Content directory to watch
    protected string debugPath;

    FileSystemWatcher watcher;

    public ReloadingResource(ResourceReloader resourceReloader, string resourceName) {
#if DEBUG && !IOS
        this.resourceReloader = resourceReloader;
        this.resourceName = resourceName;

        watcher = new FileSystemWatcher();
        debugPath = $"../../../../Shared/Content/{resourceName}";

        watcher.Path = Path.GetDirectoryName(debugPath);
        watcher.Filter = Path.GetFileName(debugPath) + ".*";
        watcher.Changed += (_, __) => {
            resourceReloader.TriggerReload(resourceName);
        };
        watcher.EnableRaisingEvents = true;

        resourceReloader.Register(resourceName, this);
#endif
    }

    public abstract void DebugReload();
    public abstract void ProductionLoad(ContentManager content);

    public void Dispose() {
#if DEBUG && !IOS
        resourceReloader.Unregister(resourceName);
        watcher.Dispose();
#endif
    }
}
{% end %}

The reloading resource is given a resource name meant to refer to a given
resource file in the shared content directory. This name is assumed to be
without an extension as the basic `Content.Load` function expects the extension
to be stripped. Then if the current build is a DEBUG and not targeting iOS, a
file system watcher is built to notify when the given file is changed.

Unfortunately the file system watcher notifies of changes on it's own thread, so
I needed to implement a queue of resources to reload and then run down that
queue on the update thread. For this I took advantage of the Dependency
Injection system I built earlier to create a special purpose `ResourceReloader`
class which would be called on update and handle triggering the reloads.

{% code(lang="c#") %}
public class ResourceReloader : IContentLoadable, IUpdateable {
    Dictionary<string, ReloadingResource> reloadingResources = new Dictionary<string, ReloadingResource>();
    ConcurrentQueue<string> resourcesToReload = new ConcurrentQueue<string>();

    public void LoadContent(ContentManager content) {
        foreach (ReloadingResource resource in reloadingResources.Values) {
#if DEBUG && !IOS
            resource.DebugReload();
#else
            resource.ProductionLoad(content);
#endif
        }
    }

    public void Update(GameTime gameTime) {
        while (resourcesToReload.TryDequeue(out var resourcePath)) {
            reloadingResources[resourcePath].DebugReload();
        }
    }

    public void TriggerReload(string resourcePath) => resourcesToReload.Enqueue(resourcePath);

    public void Register(string resourcePath, ReloadingResource resource) => reloadingResources[resourcePath] = resource;
    public void Unregister(string resourcePath) => reloadingResources.Remove(resourcePath);
}
{% end %}

It is worth noting here that I use conditional compilation heavily here to
ensure that reload logic is only used for debug builds. Also since the resource
queue may be modified on multiple threads, I decided to use a ConcurrentQueue to
prevent any resource contention.

Lastly for the reloadable resources I created the `ReloadingTexture` class which
just inherits from `ReloadingResource` and takes a callback function which gets
called with the updated texture on reload.

{% code(lang="c#") %}
public class ReloadingTexture : ReloadingResource {
    Action<Texture2D> loadAction;
    GraphicsDevice graphicsDevice;

    public Texture2D LoadedTexture { get; private set; }

    public ReloadingTexture(GraphicsDevice graphicsDevice, ResourceReloader resourceReloader, string textureName, Action<Texture2D> loadAction = null) 
        : base(resourceReloader, textureName) {
        this.graphicsDevice = graphicsDevice;
        this.loadAction = loadAction;
    }

    public override void DebugReload() {
        using (FileStream textureStream = new FileStream(debugPath + ".png", FileMode.Open)) {
            LoadedTexture = Texture2D.FromStream(graphicsDevice, textureStream);
            loadAction?.Invoke(LoadedTexture);
        }
    }

    public override void ProductionLoad(ContentManager content) {
        LoadedTexture = content.Load<Texture2D>(resourceName);
        loadAction?.Invoke(LoadedTexture);
    }
}
{% end %}

Here I rely upon the dependency injection for locating the `GraphicsDevice` and
`ResourceReloader` instances for me. These get injected in the factory function
and are used to trigger reloads and create new texture objects.

## Animation Curves

With hot-reloading out of the way I built a super simple animation curve system
which averages the y value of all pixels that are not white or transparent for
each vertical line in a given image. I then take those values and put them in an
array normalized from 0 to 1 and use the cached array to calculate an
interpolated value between a min and a max at a given progress.

In simpler terms this gives me a clean way to get an animation from point a to
point b via the route drawn in an image like this one:

![Example Animation Curve](ExampleAnimationCurve.png)

To do this I used the `GetData` function on `Texture2D` to access the pixel
values and calculate an array of doubles.

{% code(lang="c#") %}
private double[] BuildCurve(Texture2D texture) {
    var pixels = new Color[texture.Width * texture.Height];
    texture.GetData(pixels);
    Color GetPixel(int x, int y) => pixels[y * texture.Width + x];

    var curve = new double[texture.Width];

    for (var x = 0; x < texture.Width; x++) {
        var totalY = 0.0;
        var yCount = 0.0;

        for (var y = 0; y < texture.Height; y++) {
            var pixelColor = GetPixel(x, y);
            if (pixelColor.A != 0 && pixelColor != Color.White) {
                totalY += y;
                yCount++;
            }
        }
        curve[x] = totalY / yCount / texture.Height;
    }

    return curve;
}
{% end %}

I store arrays calculated in this way in a dictionary by name for future use at
startup. I use the above described "ReloadingTexture" to manage when and how to
load the textures and a simple factory injected into the animation factory to
fill in the constructor parameters.

{% code(lang="c#") %}
public void LoadContent(ContentManager content) {
    LoadCurve("Test", content);
}

private void LoadCurve(string textureName, ContentManager content) {
    var reloadingTexture = reloadingTextureFactory(textureName, texture => {
        animationCurves[textureName] = BuildCurve(texture);
    });
    animationCurveTextures.Add(reloadingTexture);
}
{% end %}

A then created a sample function which takes a curve name and a progress value
and returns a value from 0 to 1 representing the position on the curve at
progress.

{% code(lang="c#") %}
public double Sample(string curveName, double progress) {
    double[] curve = animationCurves[curveName];
    return curve[(int)(progress * curve.Length) % curve.Length];
}
{% end %}


Then pulling everything together I created some simple `Interpolate` functions
which take a named curve, a progress value, a from value, and a to value and
returns an interpolated result based on the animation curve. I created overloads
for integers, doubles, and vectors so that they are generally applicable
anywhere I could want to use the function.

{% code(lang="c#") %}
public int Interpolate(string curveName, double progress, int from, int to) {
    int diff = to - from;
    return (int)(Sample(curveName, progress) * diff) + from;
}

public double Interpolate(string curveName, double progress, double from, double to) {
    double diff = to - from;
    return Sample(curveName, progress) * diff + from;
}

public Vector2 Interpolate(string curveName, double progress, Vector2 from, Vector2 to) {
    Vector2 diff = to - from;
    return (float)Sample(curveName, progress) * diff + from;
}
{% end %}

To test all of this out I modified my `RoutineTicker` object to load the pixel
art blocks I built for my Tetris Attack game and to draw them in a row animated
by a given curve.

{% code(lang="c#") %}
public void LoadContent(ContentManager content) {
    font = content.Load<SpriteFont>("Gugi");

    actions.Add(content.Load<Texture2D>("Actions/Cloud"));
    actions.Add(content.Load<Texture2D>("Actions/Leaf"));
    actions.Add(content.Load<Texture2D>("Actions/Moon"));
    actions.Add(content.Load<Texture2D>("Actions/Rain"));
    actions.Add(content.Load<Texture2D>("Actions/Stick"));
    actions.Add(content.Load<Texture2D>("Actions/Sun"));
}

public void Draw(GameTime gameTime) {
    spriteBatch.DrawString(font, "Hello World!", new Vector2(100, 100), colors.Foreground);

    for (var i = 0; i < 5; i++) {
        int yPosition = animationManager.Interpolate("Test", gameTime.TotalGameTime.TotalSeconds + 3.1415 * i, 600, 300);
        spriteBatch.Draw(actions[i], new Rectangle(i * 100 + 40, yPosition, 80, 80), Color.White);
    }
}
{% end %}

I just interpolate using a Test animation curve and passing the current time in
as the progress value from 600 to 300. This value is then used as the y position
to draw each block and results in this gif:

![Animation Curves](AnimationCurves.gif)

I am hopeful that by simplifying this process I will be more likely to integrate
nifty animations into the UI for my game. I'm pretty pleased with the tradeoffs
made for this system as it was achievable in an afternoon and should get me most
of the way there without sacrificing too much in ergonomics. Time will tell.

Till tomorrow,  
Kaylee
