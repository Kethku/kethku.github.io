+++
title = "Day11 - Serializable Scene"
description = "Restructure repository and introduce json format for scenes"
date = 2023-04-06
+++

Today I took some time to do some prep work for testability
and restructure the repository to allow for demo apps and
debug tools. The idea is to introduce a scene struct which
contains all of the renderable primitives that we can
currently draw and make it serializable from json. Then we
can write tests that specify a scene in this json format as
well as an expected image. If the rendering changes, we can
either show them side by side or compute a diff image
highlighting the changes.

## Cargo Workspace

Step one was to restructure things so that the renderer is a
library crate rather than a binary executable and to
introduce a `scene_renderer` crate which depends on the
renderer. The plan is to make it so that the api exposed by
the renderer is sufficient to power both Neovide eventually
and whatever test executables there might be.

In the past, Neovide has been an exclusively single crate
project with some dependent crates in subfolders for things
like proc macros. This works for a while, but has draw backs
in terms of compile time and organization. Its hard to test
components individually when structure in this way and when
you make a change, often the entire crate needs to be
recompiled.

Cargo has a solution for this in the form of [workspaces](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html)
which let you define multiple crate with some shared
configuration and which all get compiled to the same target
directory.

With this migration completed, I now have the `shader` crate
which compiles rust code to spirv, the `renderer` crate
which exposes the `Renderer` struct in charge of managing
all of the shader and gpu specific code with a high level 2d
graphics interface, and the `scene_renderer` crate which
uses the `Renderer` to draw `scenes` from file. Eventually I
think its possible more sub crates will be introduced and or
this pattern will get ported to Neovide once the crate is
more stable.

## Scene Watcher

I introduced a simple `Scene` struct containing some details
about the window and fonts and a list of primitives. Right
now this is just the `Quads`, `Glyphs`, and `Texts` that I
support so far, but eventually this will likely include all
of the details we support such as layers with clipping
boundaries and blurred backgrounds as well as more complex
paths such as that of the cursor.

```rs
#[derive(Deserialize, Debug)]
pub struct Scene {
    #[serde(default = "default_background_color")]
    pub background_color: Vec4,
    #[serde(default = "default_font")]
    pub font_name: String,
    #[serde(default = "default_window_size")]
    pub window_size: Vec2,
    #[serde(default)]
    pub quads: Vec<Quad>,
    #[serde(default)]
    pub glyphs: Vec<Glyph>,
    #[serde(default)]
    pub texts: Vec<Text>,
}

#[derive(Deserialize, Debug)]
pub struct Quad {
    pub top_left: Vec2,
    pub size: Vec2,
    pub color: Vec4,
}

#[derive(Deserialize, Debug)]
pub struct Glyph {
    pub character: char,
    pub bottom_left: Vec2,
    pub size: f32,
    pub color: Vec4,
}

#[derive(Deserialize, Debug)]
pub struct Text {
    pub text: String,
    pub bottom_left: Vec2,
    pub size: f32,
    pub color: Vec4,
}
```

The only really interesting thing here is the utilization of
serde's default field constructors which specify a function
to fill if the json doesn't have the relevant field. With
this in place I added a `draw_scene` function to the `Renderer`
which takes one of these scene objects and calls the
relevant `add_*` function for each element.

```rs
pub fn draw_scene(&mut self, scene: &Scene, window: &Window) {
    window.set_inner_size(PhysicalSize::new(
        scene.window_size.x as u32,
        scene.window_size.y as u32,
    ));

    self.clear(scene.background_color);

    for quad in scene.quads.iter() {
        self.add_quad(quad.top_left, quad.size, quad.color);
    }

    let font = Font::from_name(&scene.font_name).unwrap();
    let font_ref = font.as_ref().unwrap();

    for glyph in scene.glyphs.iter() {
        self.add_glyph(
            font_ref,
            font_ref.charmap().map(glyph.character),
            glyph.bottom_left,
            glyph.size,
            glyph.color,
        );
    }

    for text in scene.texts.iter() {
        self.add_text(
            font_ref,
            &text.text,
            text.bottom_left,
            text.size,
            text.color,
        );
    }
}
```
From there I used the `notify` crate to watch a json file in
the root of the repository containing one of these
serialized scenes. When the file changes, I read from the
file and trigger a redraw of the window with the updated
scene contents.

```rs
let event_loop = EventLoop::new();

let scene: Arc<RwLock<Scene>> = Default::default();
let scene_path = Arc::from(Path::new("./scene.json"));
read_scene(&scene_path, &scene);

let mut watcher = recommended_watcher({
    let scene_path = scene_path.clone();
    let event_loop = event_loop.create_proxy();
    let scene = scene.clone();
    move |event| {
        if let Ok(notify::event::Event {
            kind: notify::event::EventKind::Modify(_),
            ..
        }) = event
        {
            read_scene(dbg!(&scene_path), &scene);
            event_loop.send_event(()).unwrap();
        }
    }
})
.expect("Could not watch scene file");

watcher
    .watch(&scene_path, RecursiveMode::NonRecursive)
    .unwrap();
```

This lets me edit the scene dynamically from my text editor
and see changes live for debugging purposes. And as I
mentioned earlier, this same system could power automated
testing that specifies exactly what we expect each scene
should look like.

The work here wasn't necessary to make progress, but helps
me to have confidence in the work so far. I'm hopeful it can
be used to test apps like neovide in the future as well.

With this work out of the way, I'm more confident I can get
a better idea of whats going wrong with the text shaping.
Once that's out of the way, I hope to add scissor based
clipping and blurred background layers which is the last
hard requirement before I can start integrating this work
into Neovide.

Till tomorrow,  
Kaylee
