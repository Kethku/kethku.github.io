+++
title = "Day12 - Project Pages"
description = "Central Project Pages"
date = 2019-02-18

[extra]
project = "blog"
+++

Today instead of doing much programming, I decided to do some site maintenance.
It has been bugging me for a while that I don't have a central place on the blog
that contains a list of all of the projects I am working on as a part of the
Every Day posts so for my daily today, I decided to fix that.

[Zola](https://www.getzola.org/), the Rust based static site generator I have
been using, has two main content concepts: Sections and Pages. Since I already
implemented a simple section template and page template for the blog, adding a
new concept on the same level as the blog was as easy as creating a new folder
and writing posts just like I have been writing blog posts. 

For descriptions, I tried to summarize each project without rewriting much of
the related blog posts. For the tools I gave a description of what each does,
why I wanted it, and how to use it. Each of the projects are listed on the
central [project page](../../projects/). I have linked to the current projects
here:

[Daily Blog](../../projects/blog/)
[Okeydokey](../../projects/okeydokey/)
[Pando](../../projects/pando/)
[8Bomb](../../projects/8bomb/)

From now on, each blog post will contain a link to the associated project page
(excluding meta posts about the blog), and each project page will contain a link
to all of the blog posts which make progress on the project. Hopefully this will
make the site more descoverable and useful to any that might read it.
Furthermore, I will link to these project pages from each of the associated
GitHub repos.

In a future post I may automate this process using the tags feature in Zola, but
for now I think handling it manually is fine until it starts to bug me.

Quick post today, but I think I've done more than enough writing for one day.
I'll be back to the regularly scheduled projects next time.

Till tomorrow,  
Kaylee
