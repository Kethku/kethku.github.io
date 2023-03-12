+++
title = "Day65 - Multi Line Garbage Clearing"
description = "Reworked multi line garbage clearing"
date = 2019-04-15

[extra]
project = "ta"
+++

![Todo](./todo.svg)

Today I reworked the clear animation to allow for spawning garbage blocks when
garbage is cleared that is more than a single row tall. To do this I needed to
change how the spawned blocks are revealed and created.

## Grid Location Refactor

The first step was to pull the render location calculation out of the garbage
and block classes into it's own helper. This code was already fairly repetitive
so pulling it out was useful regardless.

{% code(lang="javascript") %}
export function gridToScreen({ position, dimensions }) {
  let result = {};

  if (position) {
    let blocksTopLeft = new Vector(
      gridCenter.x - gridDimensions.width / 2,
      gridCenter.y - gridDimensions.height / 2 + blockPixelAdvancement);

    result.position = blocksTopLeft.add(
      position.multiply(blockWidth)
        .multiplyParts(new Vector(1, -1)));
  }

  if (dimensions) {
    result.dimensions = dimensions.multiply(blockWidth);
  }

  return result;
}
{% end %}

I use a pattern here where the function takes as argument a single object and
returns an object which may have optional properties. This allows me to group
similar calculations together while still picking and choosing what parts I want
for a given situation.

## Covered Slots

Previous I rendered the block covers when breaking a garbage block by passing
the cover texture to the spawned block render functions. This was a quick fix to
reuse the block render logic, but now that I have factored that logic out into
it's own helper, this isn't needed. That code also prevented me from drawing
multiple cover textures over a single garbage block which is now necessary since
breaking a garbage block may spawn other smaller garbage blocks.

My change was to modify the `ClearAnimation` class to add every overlapping slot
to a "`Set`" of overlapping slots to draw instead of using a specialized object
with a visible parameter. Importantly I do not use the actual `Set` object here
because it uses reference equality when adding and removing from the set.
Instead I use a map where the key is the stringified version of the slot
`Vector` and the value is the instance itself. This will approximate value
equality which is what I actually want. I want to know if the given slot value
is in the "`Set`", not whether the particular instance of a slot `Vector` is
contained in the set.

{% code(lang="javascript") %}
class ClearAnimation {
  constructor(triggeringBlocks, garbageBlocks) {
    this.timer = 0;
    this.triggeringBlocks = triggeringBlocks;
    this.garbageBlocks = garbageBlocks;
    this.coveredSlots = new Map();
    this.spawnedBlocks = [];

    for (let garbage of garbageBlocks) {
      for (let slot of garbage.overlappingSlots()) {
        this.coveredSlots.set(JSON.stringify(slot), slot);
      }

      createSpawnedBlocks(garbage);
    }
  }
{% end %}

Then when updating the clear animation I remove each covered slot one by one
from the `coveredSlots` "`Set`" rather than removing them by which is covered by
a spawned block.

{% code(lang="javascript") %}
update() {
  if (this.timer > clearDelay) {
    if ((this.timer - clearDelay) % blockClearDelay == 0) {
      let anyUncovered = false;
      for (let coveredSlot of this.coveredSlots.values()) {
        this.coveredSlots.delete(JSON.stringify(coveredSlot));
        anyUncovered = true;
        break;
      }

      if (!anyUncovered && !this.breakTimeStarted) {
        this.breakTimeStarted = this.timer;
      }
    }
  }
{% end %}

When rendering, I simply draw every spawned block and then draw each covered
slot texture on top.

{% code(lang="javascript") %}
render() {
  for (let spawnedBlock of this.spawnedBlocks) {
    spawnedBlock.render();
  }

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
}
{% end %}

## Spawning Garbage

I also changed the way that blocks are spawned in from a broken garbage block.
Before I added a new block for every covered slot of the broken garbage blocks,
but now only the bottom row is broken into individual blocks while the rest of
each garbage block is turned into a garbage block with one smaller height.

{% code(lang="javascript") %}
createSpawnedBlocks(garbage) {
  for (let x = garbage.gridSlot.x; x < garbage.gridSlot.x + garbage.gridDimensions.x; x++) {
    let slot = new Vector(x, garbage.gridSlot.y + garbage.gridDimensions.y - 1);
    this.spawnedBlocks.push(new Block(slot));
  }

  if (garbage.gridDimensions.y > 1) {
    let dimensions = garbage.gridDimensions.withY(garbage.gridDimensions.height - 1);
    this.spawnedBlocks.push(new Garbage(garbage.gridSlot, dimensions));
  }
}
{% end %}

Since spawned blocks may now be garbage blocks I also had to add a check to the
animation completion logic to add the spawned garbage block to the
`garbageBlocks` set.

{% code(lang="javascript") %}
if (this.breakTimeStarted &&
    this.timer - this.breakTimeStarted > breakDelay) {
  clearAnimations.delete(this);
  for (let spawnedBlock of this.spawnedBlocks) {
    setBlock(spawnedBlock);
    if (spawnedBlock.type === type.GARBAGE) {
      garbageBlocks.add(spawnedBlock);
    }
    spawnedBlock.state = state.WAITING;
  }
  ClearAnimationFinished.Publish(this);
}
{% end %}

That about wraps it all up. The final animation with spawned garbage blocks
looks like this:

![MultiLineGarbage](MultiLineGarbage.gif)

This change completes the necessary modifications to enable high resolution
versions of the garbage block textures. I still need to build some form of
linking texture between individual lines of a multi line tall garbage block, but
I will wait to build this until I can confirm what needs to be done with my
friend who did the high resolution block textures.

I also noticed while working on this that the combo recognition doesn't actually
work perfectly across block breaks so I will also have to go back and fix that
at some point.

Till tomorrow,  
Kaylee
