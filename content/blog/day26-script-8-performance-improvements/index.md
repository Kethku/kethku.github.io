+++
title = "Day26 - SCRIPT-8 Performance Improvements"
description = "Using Typed Arrays to Improve Javascript Canvas Performance"
date = 2019-03-04
+++

In yesterday's blog post, I focused on talking about the gameplay improvements
in 8Bomb, but I glossed over the performance improvements I made in the
rendering side for SCRIPT-8. Before my changes, SCRIPT-8 did the majority of
it's rendering using single pixel wide `fillRect` calls on the 2d canvas
graphics context. This worked great because a lot of things such as aliasing,
filling, and camera transforms were handled automatically. In practice though it
became a huge rendering bottleneck because the canvas would draw to the screen
imediately after each call.

The solution to this problem was to do the pixel manipulation manually in a
frameBuffer instead. This adds complexity to the drawing code because the
transformation, boundary checks, and integer math had to be done manually, but
it runs much quicker since the frame data is only drawn to the screen once every
frame instead of every canvas api call (or even multiple times per canvas api call).

The last piece to the puzzle was to utilize typed arrays for the pixel data
instead of the traditional javascript number arrays. By using typed arrays, the
browser is able to optimize the setting of colors further since it doesn't need
to do boundary checks for each pixel. These three tricks make the drawing
performance much faster and allowed the fancy end screen animations I built
yesterday.

## Setup

{% code(lang="javascript") %}
let ctx = canvas.getContext('2d');
let pixelData = ctx.getImageData(0, 0, 128, 128);
let pixelBuffer = new ArrayBuffer(pixelData.data.length);
let pixelBytes = new Uint8ClampedArray(pixelBuffer);
let pixelIntegers = new Uint32Array(pixelBuffer);
{% end %}

There is a lot of goo to get things working.

1. The graphics context, `ctx` is still required since it is how we set the
   image data back to the screen.
2. `pixelData` is used here just to determine the correct array size. (this is
   probably not needed as we could calculate it ourselves, I'm just lazy.)
3. `pixelBuffer` is the actual array buffer which contains the data. We can't
   manipulate it directly, so we have to use the next variables instead to set
   and read values.
4. `pixelBytes` is needed to set the data back into the `pixelData` object.
   Setting the `pixelBuffer` or `pixelIntegers` doesn't work. No idea why.
5. `pixelIntegers` instead of setting a byte for each channel of a pixel, it is
   much faster to set them all at once using an integer. So I use the
   `Uint32Array` view of the `pixelBuffer` instead of the bytes view.
   
Since `pixelBytes` and `pixelIntegers` both refer to the same `ArrayBuffer`,
either one can be used to index the pixel data. Its faster to write using
integers, but the canvas won't read from the integer array, so I keep a
`pixelBytes` version arround for drawing the data.

{% code(lang="javascript") %}
export function drawPixels() {
  pixelData.data.set(pixelBytes);
  ctx.putImageData(pixelData, 0, 0);
}
{% end %}

I then added a `drawPixels` function which I added call each frame to push the
current frame buffer data to the screen.

## Drawing

Since the various drawing functions previously used graphics context methods to
draw, they needed to be modified to work with the pixel buffer. I achieved this
by changing setPixel first, and rewriting the rest to use setPixel instead of
drawing single pixel rectangles.

The first step was to implement a color lookup function to find the correct
integer for each color index. It wasn't too complicated, just a bit of bit
twiddling.

{% code(lang="javascript") %}
function int (i) {
  let values = triplets[i % triplets.length];
  return (255 << 24) |
    (values[2] << 16) |
    (values[1] << 8) |
    values[0];
}
{% end %}

Then use the color integer function to modify the frame buffer.

{% code(lang="javascript") %}
export function setPixel(x, y, c = 0) {
  x = Math.floor(x - _cameraX);
  y = Math.floor(y - _cameraY);
  if (x < 0 || x >= 128 || y < 0 || y >= 128) return;
  pixelIntegers[y * 128 + x] = colors.int(c);
}
{% end %}

The camera value is adjusted for here since the canvas will no longer do it for us.

Then as an example port to the new pattern, I updated the print draw command.
Since the graphics no longer needs to read from the screen, things got a lot
simpler.

{% code(lang="javascript") %}
export function print(x, y, letters, c = 0) {
  let currentX = Math.floor(x - _cameraX);
  let currentY = Math.floor(y - _cameraY);

  for (let letter of letters.toString().split('')) {
    const pixels = alphabet[letter.toLowerCase()];
    if (!pixels) currentX += 3; // Couldn't find a character

    let letterWidth = pixels.length / 6;
    for (let x = 0; x < letterWidth; x++) {
      for (let y = 0; y < 6; y++) {
        if (pixels[y * letterWidth + x]) {
          setPixel(currentX + x, currentY + y, c);
        }
      }
    }
    currentX += letterWidth + 1;
  }
}
{% end %}


Thats all I have time for tonight. Tomorrow I will work on porting the few
remaining functions and actually get a PR up for the performance improvements.
Once that PR merges, I will be able to port my game code back into the SCRIPT-8
editor and get it published to the shelf!

Till Tomorrow,  
Keith
