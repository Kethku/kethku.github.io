+++
title = "Day29 - SCRIPT-8 Graphics Debugging"
description = "Fixing a weird bug in the SCRIPT-8 Bios"
date = 2019-03-07
+++

![EntireBios](EntireBios.gif)

I spent today debugging a very obscure bug in the boot up animation for
SCRIPT-8. It took a lot of trial and error to arrive at the exact solution, but
it feels great to finally tie up the few remaining inconsistencies in the frame
buffer drawing API.

## The Issue

After ironing the last of the api differences that I know about in the cassettes
on the SCRIPT-8 shelf, I noticed a slight inconsistency in the first part of the
boot up animation. Instead of drawing blocks of glitchy looking color, groups of
pixels were drawn in a flickery pattern.

![BadBios](BadBios.gif)

Although interesting, this isn't remotely the same as the original. Confused, I
began to dig through the source code in the SCRIPT-8 repo to figure out how the
actual animation is drawn. I discovered that the entire bootup animation is
actually a special cassette that runs before the requested cassette. The source
as of 3/8/2019 can be found
[here](https://github.com/script-8/script-8.github.io/blob/ac51c6b4423896fa53bb36d3688e0aebc0c749f2/src/iframe/src/utils/bios.js).

## First Attempt

After a bit of fiddling I was able to extract the important bits into a
standalone cassette for testing purposes.

{% code(lang="javascript") %}
const size = range(128)
const d = 1
const rects = flatten(size.map(x => size.map(y => [x, y, d, d])))
let counter = 0

draw = () => {
  rects.forEach(rect => {
    rectFill(...rect, rect[0] + (rect[1] * counter) / 100)
  })
  counter++
}
{% end %}

This code was a bit strange, so I refactored slightly to just contain loop so I
could get a better idea what was actually going on.

{% code(lang="javascript") %}
let counter = 0;
draw = () => {
  counter++
  for (let x = 0; x < 128; x++) {
    for (let y = 0; y < 128; y++) {
      fillRect(x, y, 1, 1, x + (y * counter) / 100);
    }
  }
}
{% end %}

In this form, the algorithm becomes pretty clear. For every pixel on the screen,
fill a single pixel wide and tall rectangle with the color `x + (y * counter) /
100`. I arrived at this point day before yesterday and was incredibly stumped.
At first look, none of this makes any sense that it would draw the above. It was
very late at this point, so I eventually went to bed.

Fast forward to today, and I decided to take another stab at figuring out what
the heck was going on. The first insight was that the color functions in my
implementation of the canvas API cleared the color if an invalid pixel color is
set. In the bios animation, the vast majority of pixel colors are incorrect
because they are not integers. So for my API the pixels get cleared instead of
being transparent. I thought that maybe, if I added a check for invalid colors
and just skipped setting the pixel, then it might fix things.

![BetterBios](BetterBios.gif)

## Second Attempt

Welp... At least I learned something. Clearly something weird was going on. At
this point I looked back at the desired animation and noticed something
interesting.

![Pixels](Pixels.png)

The colors seemed to preserve the value from the pixel above. So a pretty simple
change to the bios code would likely create the same effect. Since I fixed the
transparent pixel problem, it was a one line change.

{% code(lang="javascript") %}
let counter = 0;
draw = () => {
  counter++
  for (let x = 0; x < 128; x++) {
    for (let y = 0; y < 128; y++) {
      rectFill(x, y, 1, 128, x + (y * counter) / 100);
    }
  }
}
{% end %}

Instead of drawing a single pixel in each position, I draw a rectangle with
height 128. This fills the pixels below each valid pixel value until another
valid pixel is drawn.

![CorrectBios](CorrectBios.gif)

## Success!

With my corrected version, I was able to modify the original bios code to do the
same thing by changing the rect array declaration like so:

{% code(lang="javascript") %}
const rects = flatten(size.map(x => size.map(y => [x, y, d, 128])))
{% end %}

With my accurate animation, I checked in and took a break.

## ... but y tho

Of course I couldn't just leave it at that. I needed to figure out why the old
API acted differently. After some more inconsequential fumbling about. I
eventually figured it out. The core issue has to do with the
`rectStroke` code:

{% code(lang="javascript") %}
rectStroke(x, y, w, h, c = 0) {
  ctx.strokeStyle = colors.rgb(c)
  ctx.strokeRect(
    Math.floor(x) + 0.5,
    Math.floor(y) + 0.5,
    Math.floor(w) - 1,
    Math.floor(h) - 1
  )
},
{% end %}

It wasn't obvious at first, but `colors.rgb(c)` returns undefined if the passed
color value isn't an integer. This combined with a strange quirk of the canvas
context means that for an invalid color, the stroke color doesn't actually get
changed and will preserve whatever stroke was set last. Since the loop over the
pixel list draws column by column left to right, each pixel draws the color of
the last pixel that had a valid color.

To prove my theory, I simply reversed the loop so that instead of drawing column
by column it drew row by row.

{% code(lang="javascript") %}
let counter = 0;
draw = () => {
  counter++
  for (let y = 0; y < 128; y++) {
    for (let x = 0; x < 128; x++) {
      fillRect(x, y, 1, 1, x + (y * counter) / 100);
    }
  }
}
{% end %}

Which presented me with this satisfying animation:

![YToXBios](YToXBios.gif)

Rows tend to have valid multiples all at once, so the color of a group of rows
is defined by the last pixel drawn in a valid row. Mystery solved!

At this point my [PR](https://github.com/script-8/script-8.github.io/pull/254)
is almost there. All that remains is to fix the UI surrounding actor
manipulation which depends strongly on the old canvas API. I will leave this for
next time.

Till tomorrow,  
Keith
