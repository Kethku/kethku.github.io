+++
title = "Day3 - Rust Gpu"
description = "Reviving an old project to experiment with a new Neovide renderer"
date = 2023-03-25
+++

> A branch splits off the main trunk of the Maple and stops
> abruptly at a stump. A rough bench is carved into the
> space for climbers to rest at before climbing on. Resting
> on the bench is a pile of papers containing research on
> the nature of the trio and speculation about their
> origins. Although maybe plausible, the ideas are likely
> unfounded.

Today I looked into reviving an old project of mine to start
playing with Rust-GPU for a Neovide front end replacement.

## Skia

When I first started working on Neovide, I did a survey of
the available 2d renderers in Rust. At the time, the only
options really were druid, pathfinder, and skia. Druid was
interesting because the Piet renderer worked in so many
places and the promise of Xi editor meant that it seemed
like it had some legs. I ended up leaving druid because I
was unable to get the window events I needed to make a good
neovim front end. Pathfinder was also very exciting as a
next gen, rust first solution, but I struggled to get it
building on my machine and gave up.

That left Skia which is a 2d renderer maintained by Google
and used heavily in parts of Android and Chrome. Its pretty
fast, very cross platform, and tested very well on many
platforms and in tons of apps including Sublime Text. Right
around when I started working on Neovide, a crate was
released called Skulpin which wrapped Skia-Safe and handled
the creation of the window and vulkan boilerplate making it
easy to get something on the screen. This worked great to
get things going.

Fast forward a couple years and I think the benefits of Skia
are not outweighing the drawbacks. Although the api is
relatively simple, the actual performance is hard to
predict. The build process for skia is also less than ideal
because it isn't well integrated into Cargo. Some platforms
struggle to build it because a prebuilt binary isn't
available. Even when it builds correctly, the compile time
isn't ideal. All of the above issues have lead me to look
for another solution.

## Rust-gpu
During my time at Zed I grew to appreciate the fact that we
owned the entire stack from windowing library to renderer.
When something wasn't working, it was always tractable
because it was written in relatively understandable rust.
Rather than having to dig down a layer or create a PR to
some other codebase with unknown constraints, all the moving
parts were available for us to look at and consider.

I also discovered that a relatively simple architecture
doing pretty basic graphics programming could get you a long
way before hitting some performance limit. Until working at
Zed I assumed that the advanced techniques described by
folks like [Raph Levien](https://raphlinus.github.io/rust/graphics/gpu/2020/06/13/fast-2d-rendering.html)
were necessary to achieve good 2d graphics performance. In
reality though nice, if you are smart about how you select
what to render and don't rerender unnecessarily, you can get
more than good enough performance without that complexity.
And then if you need more of those techniques, they are
available after the fact.

Enter [rust-gpu](https://github.com/EmbarkStudios/rust-gpu)
the successor to an earlier project called rlsl which works
to compile rust code directly to a target called spirv. In
turn, spirv is a binary compile target for higher level
shader languages like glsl and hlsl. By compiling to it, the
shaders can be run on many platforms.

I like rust-gpu not only because writing shaders in rust is
super cool, but because it enables sharing code between the
shader and the application. Rather than writing a type on
the app side and an equivalent type in the shader which must
be kept in sync, we can write a single type on the shader
side and *reference* that same type in the application.
Because the source code for the referenced type is literally
the same in both places, we don't have to worry about them
getting out of sync. Write once, use on cpu *and* gpu.

A couple years ago I wrote a ray marcher that explored this
idea further. The scene was a complex combination of signed
distance fields which described the world and was relatively
easily rendered in a full screen pixel shader. The player
controller then sampled *the exact same code* on the cpu in
order to let the player walk around the scene and bump into
objects in it without phasing through them. The code for the
basic game version of that can be found [here](https://github.com/Kethku/rusty-marcher/commit/1d9f11eb962929614d6de6cf94c962dc91eb24f5).
I later took that system and tried to create a simple
modeling application based on that idea and marching squares
to 3d print the sdfs, but I eventually abandoned that effort
due to accuracy issues.

## Where I'm At So Far
So I spent today working on updating the dependencies of my
rust-gpu demo and investigating what it would take to make a
simple renderer that was suitable for Neovide to replace
Skia with. My conclusion was that we could use the struct
replication strategy but instead of using PushConstants like
my demo does for the uniforms, use a Uniform Buffer filled
with a flat array of instance structs. That Buffer is then
bound in the shader and indexed into using the index id.
This way we define the shape of the instance in one place
and use it both in the shader and the app code.

The next step with this effort is to create a simple scene
object containing the list of quads and potentially layers.
As the scene is built up, new quads are pushed into the list
and when the scene is complete, they can all be marshalled
over to the uniform buffer together and rendered with a
single draw call. This pattern can then be extended with
glyph atlases to render text, and layers to handle blurs or
shadows as necessary. The goal here is simplicity. We aren't
going for the fastest possible solution. Just something that
can get us off Skia. Then we can profile to figure out what
needs more attention from there.

Till tomorrow,  
Kaylee
