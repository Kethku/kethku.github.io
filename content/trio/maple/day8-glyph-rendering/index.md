+++
title = "Day8 - Glyph Rendering"
description = "Glyphs Rendering on Screen"
date = 2023-04-02
+++

Today I finally got some text drawing to the screen. This
update is going to be quick as it is very late, but I will
likely write a more full fledged summary soon once I can
confirm everything renders correctly.

![Subpixel](WorkingSubpixelK.png)

Subpixel font rendering takes advantage of the structure in
modern lcd monitors to render text sharper than would
otherwise be possible. The red and blue outline aliasing in
the text looks strange when blown up like this, but when
rendered on an lcd, make the text look crisp.

## Ping Pong Textures

Text rendering is VERY complicated. To do it properly with
good subpixel anti aliasing, you have to take into account
the color of the thing you are rendering onto as well as the
desired color of the text itself. For graphics apis, this
isn't an easy lift because the shader is drawing into the
buffer it is potentially reading from.

The solution I eventually landed on was to maintain a two
textures of screen textures which get alternately read and
written to at the same time as the screen buffer. This way
we can reference the most recent state as of the last draw
in a shader. This is used today in text rendering, but will
eventually power background blurs once I get that working.

I wont bore with the details about what above involved, but
it took quite some debugging and fiddling around to arrive
at this solution. I'm pretty happy with the approach now
though as it means there's a standard way to do post
processing style effects for when we need it.

## Sub Pixel Text Rendering

With todays work done I have glyphs rendering as subpixel masks
into an atlas texture which is then read in the glyph shader
to draw individual characters. The glyph shader uses a
combination of the current destination pixel color, the
desired glyph color, and the mask alpha values to compute
the final color.

```rs

#[spirv(fragment)]
pub fn glyph_fragment(
    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] glyphs: &[InstancedGlyph],
    #[spirv(descriptor_set = 0, binding = 1)] atlas: &Image2d,
    #[spirv(descriptor_set = 1, binding = 0)] surface: &Image2d,
    #[spirv(descriptor_set = 1, binding = 1)] sampler: &Sampler,
    #[spirv(push_constant)] constants: &ShaderConstants,
    #[spirv(flat)] instance_index: i32,
    #[spirv(frag_coord)] surface_position: Vec4,
    atlas_position: Vec2,
    out_color_surface: &mut Vec4,
    out_color_texture: &mut Vec4,
) {
    let glyph = glyphs[instance_index as usize];
    // Here we have to sample specifically the 0 LOD. I don't
    // fully understand why, but I think it has to do with how
    // the spirv is generated.
    // More details here: https://github.com/gfx-rs/wgpu-rs/issues/912
    let surface_color =
        surface.sample_by_lod(*sampler, surface_position.xy() / constants.surface_size, 0.);
    let mask_color = atlas.sample_by_lod(*sampler, atlas_position, 0.);
    *out_color_texture =
        glyph.color * mask_color + (1.0 - glyph.color.w * mask_color) * surface_color;
    *out_color_surface = *out_color_texture;
}
```

This approach is outlined in the [webrender docs](https://github.com/servo/webrender/blob/master/webrender/doc/text-rendering.md)
which go over it in much greater detail. The solution I
chose isn't the most performant, but I believe may be as
good as I'm going to get with wgpu because it doesn't yet
support duel blend modes.

Next up I plan on porting the swash based text shaping so
that I can render more than just a single glyph at a time.

Till tomorrow,  
Kay
