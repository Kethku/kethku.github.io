+++
title = "Day1 - Everydays"
description = "Reintro to Everydays"
date = 2023-03-23

[extra]
project = "blog"
+++

> A makeshift platform made out of boards and cordage is
> attached around one of the Maple's branches. There is a
> sketchy rope ladder hanging down from a branch higher up.
> Looking through the canopy you can make out that it
> connects to one of the Oak's branches a ways out and then
> higher up into the Hemlock.
>
> Scattered around the platform are carving tools, some
> metal scribes that are blackened at the end and a panel
> with text written on it. The tools are collected in a
> haphazard pile next to a burlap sack as though whoever
> used them is getting ready to move everything to another
> location soon.

Here goes nothing.

I'm back on my writing again. There are a couple reasons I'm
trying to write consistently. One is that I'm currently
without a job (hit me up on twitter @Kethku if you're
interested in hiring me). Another is that I'm feeling
drained.

Politics are scary, my life is chaotic and unorganized, and
it is becoming gradually harder to make progress on the
things that used to bring me joy. The last time I felt this
way I got a lot of value out of the every day blog posting.
I hope that by using this as a forcing function I might be
able to capture some of that excitement again. We will see.

## The Blog - World Building

This time around I'm doing things a little differently. I
have this idea of a site that you can visit and interact
with. One where you can see other people visit as well. That
combined with a new interest in TTRPGs has lead to the
current incarnation of the site. I want my website to be
something you can explore and I want to use it as a place to
play around with more creative writing.

So the site has a new coat of paint. Right now everything is
static, but eventually I would like to build a multi user
dungeon on top of it that feels more live. Even without
those dynamic features, I think its an interesting take I
haven't seen before.

At the moment its pretty abstract but I think with each post
I want to iterate on it and at least expand the places you
can explore. I suspect there may be some daily posts about
implementing the dynamic pieces, but we'll see where it
takes me. As it stands today, the older posts can be found
in [The Oak](@/oak/_index.md) so you can scrub through and
read those if you like. New posts will show up in [The
Maple](@/maple/_index.md).

## Build and Publish Github Action

This time around I've also done some work to automate the
publishing process. When I first wrote my post, I didn't
understand the ci build system in github that well. I also
think it was pretty new. These days though its the way to go
for publishing github sites. There are even off the shelf
actions you can use to build and deploy zola sites without
compiling them locally.

The previous setup used two repositories, one for the source
code which contained all of the markdown content and scripts
for building things, and the other which was the github.io
repo containing the built html pages. I previously used
[okeydokey](@/hemlock/projects/okeydokey) to automate
building of the content, copying it to the publish repo, and
pushing it up in individual commits. This worked ok but
meant abusing git for something it wasn't made for.

Now I have a simple `publish.yml` script which builds and
pushes the new content to a gh-pages branch so I don't ever
have to touch it. Its continuously deployed whenever I push
anything to main. I write markdown, push it up, and a few
minutes later the change is live ezpz.

``` yml
# On every push this script is executed
on: push
name: Build and deploy GH Pages
jobs:
  build:
    runs-on: ubuntu-latest
    if: github.ref == 'refs/heads/main'
    steps:
      - name: checkout
        uses: actions/checkout@v3.0.0
      - name: build_and_deploy
        uses: shalzz/zola-deploy-action@v0.16.1-1
        env:
          # Target branch
          PAGES_BRANCH: gh-pages
          # Provide personal access token
          TOKEN: ${{ secrets.GITHUB_TOKEN }}
```

I will likely automate the creation of new post templates
similar to what I had with the old site but for now this is
easy enough. I'm pretty pleased with the setup and that the
decision to use zola 4 years ago has held up.

## Relative Links

The other big change aside from moving content around and
updating names #trans-things was to change the links to be
relative. I've update the url to from the old site to a new
domain `kaylees.dev`. Doing so broke a ton of links which
were hard coded to the old url. A quick look at the zola
docs showed that you can get build time checked relative
links by using `[Link Text](@/relative/path/from/site/root)`
which is great as it looks up the published root url and
swaps the relative path out for the final built url.

I believe I had avoided this approach in the past because it
was a zola specific hack, but at this point I don't think
zola is going anywhere and even if it did it wasn't that
hard to fix the urls this time. So for now at least I'm
going with this approach.

Next up for the blog I am hoping to update my current
iteration of pando to produce todo trees like I had with the
old blog. This may take some doing 

Till tomorrow,  
Kaylee
