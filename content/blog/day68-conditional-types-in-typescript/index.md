+++
title = "Day68 - Conditional Types in Typescript"
description = "Using conditional types to port javascript code to typescript"
date = 2019-04-18

[extra]
project = "ta"
+++

![Todo](./todo.svg)

Today I finished porting all of the source files in my Tetris Attack remake to
Typescript. Most of the work was similar to my last post on the topic, but I did
run into an interesting type system issue which I figured would be interesting
to talk about.

## Javascript Version

The function in question was the gridToScreen function used for transforming
block location information from grid space to screen space. The original
function looked like this:

{% code(lang="javascript") %}
export function gridToScreen(location) {
  let result = {};

  if (location.position) {
    let blocksTopLeft = new Vector(
      gridCenter.x - gridDimensions.width / 2,
      gridCenter.y - gridDimensions.height / 2 + blockPixelAdvancement);

    result.position = blocksTopLeft.add(
      location.position.multiply(blockWidth)
        .multiplyParts(new Vector(1, -1)));
  }

  if (location.dimensions) {
    result.dimensions = location.dimensions.multiply(blockWidth);
  }

  return result;
}
{% end %}

Its a pretty simple function which creates a return object with transformed
variables depending upon which properties exist on the incoming object. This
pattern is somewhat common in dynamic programming languages because you can
group a series of operations that are done sometimes together or sometimes
separately into one unit. Unfortunately with traditional type systems this can
be difficult to handle properly.

## Naive Approach

Standard type annotations for the argument might look like this:

{% code(lang="typescript") %}
interface Location {
  position?: Vector,
  dimensions?: Vector
}

export function gridToScreen(location: Location) {
  let result = {} as Location;

  if (location.position) {
    let blocksTopLeft = new Vector(
      gridCenter.x - gridDimensions.width / 2,
      gridCenter.y - gridDimensions.height / 2 + blockPixelAdvancement);

    result.position = blocksTopLeft.add(
      location.position.multiply(blockWidth)
        .multiplyParts(new Vector(1, -1)));
  }

  if (location.dimensions) {
    result.dimensions = location.dimensions.multiply(blockWidth);
  }

  return result;
}
{% end %}

This compiles fine, but we run into problems if we want to use properties in the
output of the function. 

{% code(lang="typescript") %}
for (let coveredSlot of this.coveredSlots.values()) {
  let renderInfo = gridToScreen({
    position: coveredSlot,
    dimensions: Vector.one
  });

  image({
    imageUrl: garbageImages.Clear,
    center: Vector.topLeft,
    ...renderInfo
  });
}
{% end %}

For example in the render function on the `ClearAnimation` class we get a
compiler error complaining that the image function argument does not contain the
position and dimensions properties. The compiler has no way to guarantee that
the properties on `renderInfo` are actually there.

## Set-ish Types

To fix this issue and help the type system along we need to take advantage of
some more advanced type system features in the recent versions of Typescript.
But first, some background terminology.

Typescript contains two concepts that have names related to Set operations, but
are a bit misleading: Union and Intersection types. The Union of two types in
Typescript produces a new type containing *all* of the properties of each of the
types combined. Similarly the intersection of two types produces a new type with
either the properties of the first object or the properties of the second.

The union type makes good sense since a union of two sets contains all of the
elements that exist in one of either of the sets.

{% code() %}
(A, B, C) Union (C, D, E) Equals (A, B, C, D)
{% end %}

The intersection type is weird though because a valid element inhabiting the
intersection between two overlapping types has no guarantee about what
properties exist on it. In normal set theory terms:

{% code() %}
(A, B, C) Intersection (C, D, E) Equals (C)
{% end %}

But in typescript it means that the final object could be the first type or the
second type. It could be me, but I find this somewhat confusing.

## Dependent er... Conditional Types to the Rescue

Luckily modern Typescript gives a way to define our own versions of these ideas.
In my case I need a type which truly is the "intersection" of two types which
has the common properties between the two. To do this I use type conditions to
specify the constrain I have in mind.

{% code(lang="typescript") %}
export type Common<A, B> = {
  [P in keyof A & keyof B]: A[P] | B[P]
};
{% end %}

The syntax is a little bit weird, but in English this says the following:

{% code() %}
Define the Common of two types, A and B as
A new type with 
Keys such that every key P exists in both A and B,
And values that are either the type of A[P] or B[P]
{% end %}

In summary, do something closer to the Set union of two bags of properties. The
last bit of useful information before I show the final solution is the existence
of a `Partial` type which is another bit of fancy Typescript type shenanigans
which just takes a type and creates a new type where each of the properties are
optional. It is defined as such:

{% code(lang="typescript") %}
export type Filter<T> = {
  [P in keyof T]?: T[P]
};
{% end %}

In this form you can see the structure of a mapped type or conditional type a
little easier. Its just a way to specify properties in terms of the properties
on other types.

## Better Type Annotations

With our new found fancy types in hand, the more expressive version of
`gridToScreen` type annotations is pretty simple:

{% code(lang="typescript") %}
interface Location {
  position: Vector,
  dimensions: Vector
};
export function gridToScreen<T extends Partial<Location>>(location: T) {
  let result = {} as Common<T, Location>;

  if ("position" in location) {
    let blocksTopLeft = new Vector(
      gridCenter.x - gridDimensions.width / 2,
      gridCenter.y - gridDimensions.height / 2 + blockPixelAdvancement);

    result.position = blocksTopLeft.add(
      location.position.multiply(blockWidth)
        .multiplyParts(new Vector(1, -1)));
  }

  if ("dimensions" in location) {
    result.dimensions = location.dimensions.multiply(blockWidth);
  }

  return result;
}
{% end %}

First step was to specify that the properties on the input argument are optional
using the `Partial` mapped type. Then the type of the result is simply the
common properties from the passed in argument and the location type itself. So
if the object passed in only contains the `Position` property, then the result
type will only contain `Position` as well since the only common properties are
`Position`.

The only slightly confusing bit was that I had to modify the if statements to
use the `in` operator to check for the existence of the properties so the type
system can be confident that the `position` property actually exists on the
argument at runtime.

And thats really it! My `ClearAnimation` `render` function doesn't need changed
at all because the Types provide *proof* that the correct arguments are
available when I expect them to be. I'm incredibly pleased that the type system
in Typescript continues to get more and more expressive. This is just the
smallest baby step toward more complicated proofs in software, but any progress
is commendable. Heres to hoping for full fledged Pi types in the future!

Till tomorrow,  
Kaylee
