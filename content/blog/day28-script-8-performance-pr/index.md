+++
title = "Day28 - SCRIPT-8 Performance PR"
description = "Created a pull request to SCRIPT-8 to speed up draw calls"
date = 2019-03-06
+++

I spent all of today working on porting my engine changes back upstream to the
official SCRIPT-8 engine. Most of the changes were the ones I did yesterday, but
I added ports for the map functions, polyStroke, and tile functions.

After some discussion with Gabriel, we decided to postpone fixing the Actor UI
in the new PR because it does some complicated canvas magic to get things
working.

I opened the PR with the changes
[here](https://github.com/script-8/script-8.github.io/pull/254) and with this description:

{% code() %} 
Rewrite of the CanvasApi functions to use a frame buffer instead of the default
javascript 2d canvas draw calls.

I tried to simplify things where I could. Since we no longer use the default
draw calls, we cannot depend on the context object to keep track of the camera
position. So I rewrote most of the draw functions to reuse the setPixel function
which keeps track of the camera position instead.

I also changed the structure of the index.js file in canvasAPI/ so that draw
functions can depend on each other. I think this makes things easier to
understand and keeps things DRY but I can change it back if that would be
preferable.

Most of the cassettes work fine as far as I can tell except for the boot
animation which seems to have draw problems. At this point it is too late to
figure out what is really going on, so I am going to head to bed and will look
into it further.

TODO:
- Fix Boot Animation
- Deal with the Actor Magic in Iframe.js
- Verify that camera functions work as expected
- Do some more general testing
{% end %}

I am hopeful that tomorrow I can work with Gabriel to fix the remaining issues
and get it checked in.

I don't have much to talk about other than I spent a long time debugging and
fiddling with minor inconsistencies in the cassettes between the new graphics
api and the old one. This was incredibly time consuming, but in the end
rewarding to see code I didn't write working with such a crucial component that
I had written from scratch. I am excited to see what new things are enabled by
such a faster draw api!

Till tomorrow,  
Keith
