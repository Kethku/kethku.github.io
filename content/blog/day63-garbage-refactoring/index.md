+++
title = "Day63 - Garbage Refactoring"
description = "Refactored garbage block rendering to simplify and enable new multi line garbage block rendering"
date = 2019-04-10

[extra]
project = "ta"
+++

![Todo](./todo.svg)

Today I refactored the garbage block rendering code to enable a new style of
multi line garbage block. After discussing texture options with my friend who
provided the updated block graphics, we decided that building a high quality
multi line garbage block texture which can be expanded to any height would be
unreasonable in the new style. 

Instead we are going to build a high quality version of each of the single block
tall garbage blocks and add a lock sprite between them to indicate that the
block is one unit. This makes good sense mechanic wise since multi line garbage
blocks break one line at a time rather than all at once. So the new animation
will have the bottom line break into separate blocks while leaving the blocks
above unsplit.

## Code Cleanup

With the above in mind, I now have the opportunity to delete a bunch of the
clunky multi line block rendering in favor of a much simpler algorithm.
Similarly, since the code is much smaller after deleting the unnecessary bits, I
can move the render code back into the garbage block class. I moved the
`singleRowGarbageTexture` function into `garbage.js` directly unchanged.

{% code(lang="javascript") %}
function singleRowGarbageTexture(width) {
  switch (width) {
  case 3:
    return garbageImages.ThreeWide;
  case 4:
    return garbageImages.FourWide;
  case 5:
    return garbageImages.FiveWide;
  case 6:
    return garbageImages.SingleLine;
  default:
    throw "Invalid single high block.";
  }
}
{% end %}

I also moved the main render utility into the `render` function of the `Garbage`
class. Instead of the clunky render info list I replaced it with a simple for
loop.

{% code(lang="javascript") %}
render() {
  let topLeft = this.calculateTopLeft();
  for (let i = 0; i < this.gridDimensions.height; i++) {
    image({
      imageUrl: singleRowGarbageTexture(this.gridDimensions.width),
      position: topLeft.withY(topLeft.y - i * blockWidth).withZ(-0.1),
      dimensions: new Vector(this.gridDimensions.width * blockWidth, blockWidth),
      center: Vector.topLeft
    });
  }
}
{% end %}

Next up to build the new rendering system is to add some form of temperary dev
art for the locked sprite as well as a particle system for the lock break
animation.

## Idle Optimization

Unrelated to the garbage block rendering update, I've been doing some
development on my laptop which is much slower than my main dev machine. The
higher resolution textures slowed down the drawing code a ton and made using
emacs while the game was running unbearable. To fix this I added a check in the
renderer

{% code(lang="javascript") %}
////////////////
// Draw Calls //
////////////////
export function drawToScreen() {
  if (document.hasFocus()) {
    gl.clearColor(0, 0, 0, 1);
    gl.clear(gl.COLOR_BUFFER_BIT);
    ...
{% end %}

This quick change stops the game from rerendering when it is not the focused
window allowing me to make editor changes while the game is still running. A
quick change but a big quality life improvement.

Thats it for today. Short posts for the next couple of days as I am busy
attending a friends wedding, but I will see what I can do.

Till tomorrow,  
Kaylee
