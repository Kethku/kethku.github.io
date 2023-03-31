+++
title = "Day6 - Glyph Research"
description = "Refactoring and looking into rendering glyphs"
date = 2023-03-31
+++

Short post today because much of my time was spent
refactoring the wgpu boilerplate and investigating atlas
crates for managing rendered glyphs on the gpu.

## Wgpu Refactorings

I dropped off yesterday once I had rectangles rendering to
the window. That code was pretty messy and involved hacking
together some old demos and pieces of tutorials. So today I
refactored things in preparation for text rendering.

The first change I made was to look up how to pull a set of
entry points in the shader crate into their own module. This
took some searching, but eventually I found an issue from
somebody trying to do something similar. The trick was to
reference the entry point using the module name when
creating the render pipeline.

```rs
let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
    label: Some("Quad Pipeline"),
    layout: Some(&render_pipeline_layout),
    vertex: wgpu::VertexState {
        module: &shader,
        entry_point: "quad::vertex",
        buffers: &[],
    },
    fragment: Some(wgpu::FragmentState {
        module: &shader,
        entry_point: "quad::fragment",
        targets: &[Some(wgpu::ColorTargetState {
            format: swapchain_format,
            blend: None,
            write_mask: wgpu::ColorWrites::ALL,
        })],
    }),
```

This lets me separate the code relevant to each draw call in
their own sections rather than co-locating them all in the
same place like many shaders end up being.

The next step was to pull the portions of the GraphicsState
struct out into its own struct containing all of the quad
related data and gpu handles relevant for rendering the quad
shader. I wont elaborate too much here as its relatively
straight forward, but the end goal is to be able to generate
this code from the shader and attributes in the shader
crate.

I have this idea of writing a proc macro which reads
arguments to the fragment and vertex shaders and completely
generates the pipeline which can execute those shaders
automatically or with some minor hints via custom
attributes. Again taking some ideas from the twgl library I
mentioned yesterday. I have a rule that you should only
refactor something like that if you have at least 2 examples
and expect to write more, so now is not the time. However
its good to have that in mind when pulling this code out.

## Glyph Atlases

The other half of my work today involved researching how to
render glyphs. The standard way to draw text to the screen
using the gpu is a texture atlas. This is a persistent
texture to which glyphs are rendered as they are requested.
Any time that same glyph needs to be drawn again in the
future, rather than using cpu time to redraw the glyph, the
position of it in the texture can be looked up and used
instead.

A common crate for doing this kind of work is [etegere](https://docs.rs/etagere/latest/etagere/)
which provides a simple interface for allocating rectangles
of a given size to a memory buffer. Once that is integrated
and working properly, we have to actually draw the glyphs.
For that I plan on using [swash](https://github.com/dfrg/swash)
which is a new text shaper written in rust that we have used
for a while now in Neovide. It also has a performant text
renderer which should work perfectly for this demo.

With those pieces in place, it shouldn't be hard to
implement text rendering. I'm looking forward to having a
pretty understandable stack using such low level components.

Some of the first software I ever wrote was some basic
graphics programming demos in c# using the old gdi apis to
draw pixels to the screen. At the time I was pretty
frustrated at how slow that was since it was writing pixels
directly to a back buffer on the cpu. It feels pretty great
to finally have the equivalent working but for modern
hardware and in such a way that I can fully understand all
of the moving parts.

I'm hoping to have basic glyph rendering done tomorrow, but
its quite possible I'll get distracted by something else. We
will see.

Till tomorrow,  
Kay
