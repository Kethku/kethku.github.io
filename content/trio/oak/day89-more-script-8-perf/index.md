+++
title = "Day89 - More Script-8 Perf"
description = "Worked on a perf pr today"
date = 2019-10-22
+++

Today I got off work late and decided to work on a perf PR to help out the SCRIPT-8 project. Turns out the map API was
having performance problems and needed some TLC. After some investigation and fiddling around, I noticed that the map
drawer was still using the old canvas API instead of my faster FrameBufferCanvasAPI. So the fix was to swap it over and
add a single draw call to batch all of the rendering up.

Took a good long while to get it all working, but I think its in a good state now. The PR can be seen
[here](https://github.com/script-8/script-8.github.io/pull/302).

Its pretty late at this point though, so this post is mostly just to mark that I got something done rather than any
actual content. I am looking into building some cool stuff with SCRIPT-8 though so this was a good warm up to page back
in how everything works.

These kinds of exercises in reading someone else's code and fixing some simple problem are super valuable. Its about as
close as one can get to working in industry. Plus you get to help out a local Open Source project in need!

Hopefully my PR goes in and helps fix the problem.

Till tomorrow,  
Kaylee
