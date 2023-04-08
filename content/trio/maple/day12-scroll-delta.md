+++
title = "Day12 - Scroll Delta"
description = "Using the new scroll delta field in neovim 0.9"
date = 2023-04-07
+++

Today I tried to push forward on refactoring the
scene_viewer to add a clap based argument for rendering the
scene into an image. I ended up biting off more than I could
chew and have rolled that work back.

On the side though I took a look at the newly released
Neovim 0.9 which includes `scroll_delta` data in the
`viewport` events. This was a field introduced in part for
Neovide to get better smooth scrolling.

With the existing implementation, Neovide uses the top line
of the rendered screen as reported by the viewport event to
decide where to draw the new content. The hope being that if
the user has scrolled the screen, the new content will be
inserted into the correct place visually and can be animated
into view.

This works for simple buffers but falls apart when lines are
soft wrapped or when there are folds because there will be
larger jumps in the top line than the view should actually
be scrolled. The `scroll_delta` field addresses this by including an
amount to scroll the screen by irrespective of the actual
top line and which takes into account the folds and soft
wraps.

To take advantage of this new data, I swapped the snapshots
off of using the topline and instead stored a vertical
position float. Any time a viewport event is recieved which
has a `scroll_delta`, a snapshot is taken with a vertical
position adjusted by the `scroll_delta` so that it can be
rendered in the correct place.

In my manual testing, this does fix the softwrap issues I
encountered in the original approach, but when talking with
contributors there are still problems with folds that I
haven't tracked down yet.

Tomorrow I may pull down another project from the shelf to
work on to build some steam back up after my failure today,
but we will see what I'm up for as the day is likely to be
pretty busy.

Till tomorrow,  
Kaylee
