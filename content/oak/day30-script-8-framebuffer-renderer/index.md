+++
title = "Day30 - SCRIPT-8 FrameBuffer Renderer"
description = "PR merged with faster draw API calls and ported 8bomb"
date = 2019-03-08

[extra]
project = "8bomb"
+++
![Todo](./todo.svg)

The faster "frame buffer" drawing API I have been working on got merged today!
Great reward for the occasion of completing my first month of Dailies! Things
went fairly smoothly. There were a couple of silly bugs on my side due to last
minute changes when porting things over, but the code is live and working.
Gabriel added some search query url parsing to specify which renderer to use
which let us do some AB testing between the two renderers.

## The Issues

In testing I made two errors. First had to do with the `map` function in
SCRIPT-8. 

{% code(lang="javascript") %}
  const map = (x = 0, y = 0) => {
    // Loop over every element in the map
    _runningMap.forEach((row, rowNumber) => {
      row.forEach((spriteIndex, colNumber) => {
        // If the element has a sprite index,
        if (spriteIndex !== null) {
          // Render at the correct offset position
          const dx = colNumber + x * 8
          const dy = rowNumber * 8
          sprite(dx, dy, spriteIndex)
        }
      })
    })
  }
{% end %}

I didn't fully understand the function but noticed that the passed in
x and y values were only used in the x component of the final sprite position,
not the y. So I assumed that they weren't actually used anywhere and pulled the
variable out. The fix was just to revert my silly change.

{% code(lang="javascript") %}
const dx = (colNumber + x) * 8
{% end %}

The other issue we found was that the print function had bounds checks similar
to the sprite function which would skip drawing if the passed coordinates were
off screen. This was just me itching to eak every bit of performance out without
thinking.

After fixing those simple problems, Gabriel pushed the code live and any
cassette can be played with the new API just by adding &renderer=framebuffer to
the end of the URL.

## 8Bomb Port

After the PR merged, I set about porting 8Bomb over from my hand made renderer.
I ran into a couple of interesting things in the process.

### Code Order

Since my original version of 8Bomb used javascript modules heavily, I needed to
think carefully about the order of declarations. Since all of the code depends
on the event objects, EventManagers and Vector classes, they needed to be
defined first. Then I looked at the dependency tree and added each module based
on what depended on what. Then I placed the `draw` and `update` functions in the
last file so that they have access to all of the declarations.

Interestingly this opened my eyes to how convenient the modules are for code
splitting. The structure of 8Bomb was much cleaner before I pulled it all
together.

### Constants

A slightly non obvious error came from the fact that I defined constants used in
more than one module with the same name. Turns out evaling code with constants
defined twice doesn't work well. Whod have thunk it.

### Event Managers

My event manager classes make it really easy to structure code in a way that
dependencies between modules are surprisingly light. Since the 8Bomb standalone
project was designed with Parcel as the bundler, I was able to import the
original typescript versions directly. But when I copied them into SCRIPT-8 it
became necessary to remove the typescript bits so that it could run as pure
javascript. Interestingly beyond just stripping the type annotations, I also had
to initialize the variables in each object in a constructor. Not sure why this
isn't a part of the main JS spec, but w/e.

That was about it. After a bit of fiddling I was able to get a [working
version](https://script-8.github.io/?id=28ffa97d6a6a04a1d15bb191ed66322e) of
8Bomb running in the SCRIPT-8 client! I still have to do some polishing, and add
some sort of sound effect, but its incredibly close at this point.

## Dailies

With that, I have finished my 30th daily post! It has been an incredible
experience which has gotten me excited about working on personal projects again!
I've really enjoyed building Pando, Okeydokey and 8Bomb this past month, and
have been very impressed with how much the daily progress forces me to actually
get things done. I think I have a ways to go before I can confidently share
these posts outside of my close friends and colleagues, but I believe I will get
there with time.

Since I have proven that I can do this consistently at least for a little while,
I have decided to make a slight modification on the process. Since programming
Dailies require a lot of work to finish and document, I will add some optional
leeway on post time. If for whatever reason I am not able to make a full daily
post on some day, then optionally I will postpone the post to the next day. I
expect not to use this option frequently, but this way if I am overly busy, I
can make some room for myself. I still want to add some constraint to the
project, so I will still require myself to make a post at least every other day.

### Review

Project wise, I am fairly pleased with the this month. I spent the majority of
the time working on SCRIPT-8 and 8Bomb, but I did spend some time working on
Pando, Okeydokey, and general blogging infrastructure. As a goal for the next
month, I want to vary my projects a bit and maybe look into implementing
something closer to my work and research. We will see if I am able to pull it
off, but I think it could be good fun.

I also have some ideas regarding a fantasy console of my own somewhat in the
vein of [VectorBoy](https://davidjalbert.itch.io/vectorboy). I have no idea if I
will actually get started on it, but it is fun to think about and plan.

With that, here is to a successful month of Dailies! May the next month be just
as good.

Till tomorrow!  
Kaylee
