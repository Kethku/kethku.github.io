+++
title = "Pando"
description = "General purpose graph editor"
date = 2019-02-10

[extra]
project_name = "pando"
+++
[https://github.com/Kethku/Pando](https://github.com/Kethku/Pando)

## What

Pando is a tool for designing and editing bespoke graph
structures. It is designed to be general purpose enough to
be used across a variety of domains such as shader graphs,
todo task trees, grammar specifications, and more.

## Why

Pando started as a project to easily edit and visualize todo
trees for my personal projects. I originally built the tool
just around graphviz and a special grammar for declaring the
structure of the todos, but as time has gone on, I've
realized that a more fluent ui is necessary to make
something like that truly useful.

After porting the basics to react and eventually druid in
rust, I realized that an editor for graphs like I've been
envisioning is both a more complex UI problem, and more
generally useful than I expected.

Many (most?) software domains have parts of the problem that
are best expressed as graphs. I believe that further, not 
representing the more visual parts of a software problem in 
a visual form is a mistake and leads to a lot of wasted time 
and effort. Pando is my attempt to make this problem simpler 
in a way that anybody could pick up for whatever project.

I think a project like Pando could change the way we build
software for the better. I imagine a world where Pando could
be used in any project when a visual problem occurs. Rather
than defining things like render graphs in code, we could
define the bounds of what they can do in a visual way that
highlights the graph structure and build the rest of the
application as an interpreter over the resulting graph.

Further by building a single editor for all of these
domains, we can invest in making the UI not just usable, but
truly great. No longer will we have to settle for good
enough for designers, vfx artists, or modelers. We can build
something that is truly wonderful to use and spend time
with.

## How
Pando in its final version will be composed of a few
interconnected parts:
- A graph file format built on Sqlite for storing the graph
  data and the specification for what is allowed in the
  graph and how to display/edit it.
- An editor which parses the graph specification and
  provides a UI for editing nodes that match it.
- A specification editor which lets users author the above
  specification using the same UI that end users can author
  themselves.
- A parser generator which will generate code in a variety
  of languages to parse the graph files into usable plain
  old data structures based on the specification.

Once these minimum features are implemented, there are some
further features that will take things to the next level for
some specific domains:
- Extract graphs to svg with configurable themeing for 
  displaying elsewhere.
- Allow connecting to the editor from external plugin
  servers to provide custom functionality such as inline
  inspect values/partial image renders or other linting. A
  relatively simple api like this would be required to
  really compete with other shader graph editors.
- An interface for linking to other pando files either for
  the specification (think standard specs defined in a known
  url for example or from a database of useful known specs)
  or for other parts of graphs (think libraries of parts of
  graphs that are useful in many places in a project).
- A standard way to diff and merge graphs in source
  control. Sqlite is not easily merged in git automatically
  so we will have to smooth over this problem in some way. I
  expect that teachings from crdts could be adapted to make
  merges automatic and conflict free.
