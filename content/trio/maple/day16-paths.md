+++
title = "Day16 - Paths"
description = "Worked on adding arbitrary paths to the renderer"
date = 2023-04-18
+++

I didn't finish an entire chunk of work today, but I did get
a big portion of the path rendering started. The plan is to
use a crate I've used before called Lyon which is a path
tessellator. It takes arbitrary svg style paths and converts
them to vertices. It does this with a lot of fancy
algorithms which let it pump out tons of vertices pretty
quickly without much overhead.

I worked on adding a render pass which takes paths described
as path commands from the scene layers and runs them through
the tessellator and puts the result into a vertex buffer.
This required a bunch of new logic as the other existing
render passes (quads and glyphs) use uniform buffers instead
of classic vertex buffers.

```rs
#[derive(Deserialize, Debug)]
pub enum PathCommand {
    MoveTo(Vec2),
    LineTo(Vec2),
    QuadraticBezierTo {
        control: Vec2,
        to: Vec2,
    },
    CubicBezierTo {
        control1: Vec2,
        control2: Vec2,
        to: Vec2,
    },
}

#[derive(Deserialize, Debug)]
pub struct Path {
    start: Vec2,
    commands: Vec<PathCommand>,
}
```

I can get away with no vertex buffers with the quad and
glyphs because they only ever have a set duel triangle quad
shape that is reused for every primitive. In fact the
position of the quad is generated shader side from the
vertex ids.

In contrast, the tessellated paths are more traditional
triangle lists, so I needed to specify the vertex and the
vertex buffer layout. One of the benefits of the uniform
buffer approach is that the exact structure of the instance
data doesn't need to be communicated to the graphics
hardware. Vertex buffers need to know the internal structure
for reasons that I don't fully understand. To mitigate this
extra complexity, I added a `layout` function to the
`PathVertex` struct so that the structure is co-located with
the layout rather than being split up.

```rs
#[derive(Copy, Clone)]
#[cfg_attr(
    not(target_arch = "spirv"),
    derive(bytemuck::Pod, bytemuck::Zeroable, Default)
)]
#[repr(C)]
// NOTE: Keep the ATTRIBS array in sync with this struct
pub struct PathVertex {
    pub color: Vec4,
    pub position: Vec2,
    pub _padding: Vec2,
}

impl PathVertex {
    #[cfg(not(target_arch = "spirv"))]
    // NOTE: Keep PathVertex struct in sync with this array
    const ATTRIBS: [VertexAttribute; 3] =
        vertex_attr_array![0 => Float32x4, 1 => Float32x2, 2 => Float32x2];

    #[cfg(not(target_arch = "spirv"))]
    pub fn layout<'a>() -> VertexBufferLayout<'a> {
        use std::mem;

        VertexBufferLayout {
            array_stride: mem::size_of::<Self>() as BufferAddress,
            step_mode: VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }
}
```

I've yet to finish the path drawable implementation, so I'll
likely complete that tomorrow and get some cursor shapes
drawing to the screen. Either way though, this
implementation feels good as it adds a bunch of flexibility
without a ton of new concepts. It's possible that doing this
work on the cpu will become too expensive long term, but I
think for now it will get my Neovide integration unblocked
and I can work on more complicated solutions if needed later.

Till tomorrow,  
Kay
