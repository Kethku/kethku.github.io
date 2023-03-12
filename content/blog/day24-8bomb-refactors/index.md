+++
title = "Day24 - 8Bomb Refactors"
description = "Cleaning Up 8Bomb Code Structure"
date = 2019-03-02

[extra]
project = "8bomb"
+++
![Todo](./todo.svg)

Today was a grab bag of random changes and cleanups to 8Bomb:

- Introduced a simple event manager concept to cleanup break the dependency of
  the main module on all of the other modules
- Built a vector class to simplify basic arithmetic
- Added screen shake to improve the explosion effect
- Fiddled with the collision logic to make it more symmetric

None of these were super complicated, but they were things that I had been
meaning to do for a while now, so it was good to knock them out all at once.

## Event Manager

For a while now I have used a very simple [event
manager](https://gist.github.com/Kethku/6ec9332e6f7b38fe2ac2eb6634bd85fa)
module. Defined in that file is a number of `EventManager` classes and
`PollManager` with a number suffix to indicate how many arguments they should
expect. The event manager is a super way to provide an event that can be
subscribed to for which the creator can publish events. Similarly the poll
manager is the exact same thing except subscribers can return a value which gets
collected into a list. This is super useful for providing interfaces between
modules in JavaScript.

I use these classes in two places. First, I modified the `draw` and `update`
functions to publish `Draw` and `Update` events instead of calling the various
draw and update functions directly. This way creating a new module which needs
to draw something or update on the main loop only requires modifying one file
instead of adding a new file and making sure to add the correct method calls in
the game file.

In practice I needed to change the pattern a little bit. Instead of having a
straight `Draw` `eventManager`, I split it into `DrawUI` and `DrawWorld` events.
This just lets me manage the camera all at once instead of having each draw
function set and reset the camera translation depending on their needs. In a
similar vein, I pulled the actual `EventAggregator` objects into their own file so
that any of the modules can depend on it without worrying about who required who
and such.

{% code(lang="javascript") %}
import { EventManager0, EventManager1 } from "../eventManager";

export const Update = new EventManager1();
export const DrawWorld = new EventManager0();
export const DrawUI = new EventManager0();
{% end %}

After the above changes, the game `draw` and `update` functions look a LOT cleaner:

{% code(lang="javascript") %}
export function update(state, input) {
  Update.Publish(input);
}

export function draw (state) {
  clear();
  camera(cameraX, cameraY);
  DrawWorld.Publish();
  camera();
  DrawUI.Publish();
  drawFunctions.drawInstructions(state);
}
{% end %}

Note: I still left the `drawInstructions` function because I didn't have enough
UI to pull into a single module. When I build the game over screen and such
though, I will likely put `drawInstructions` there.

## Vector

Throughout the codebase, there is a lot of repeat code for doing simple vector
operations on objects which have an x and y component. Until now, doing these
manually in each place was reasonable as the expedient thing, but now that the
game is getting more complicated, it makes more sense to pull this math out into
a centralized class. To that end I created a `Vector` class which contains a
standard library of base operators.

{% code(lang="javascript") %}
class Vector {
  constructor(x, y) {
    this.x = x;
    this.y = y;
  }

  get length() {
    return Math.sqrt(this.x * this.x + this.y * this.y);
  }

  distance(other) {
    return this.subtract(other).length;
  }

  add(other) {
    return new Vector(this.x + other.x, this.y + other.y);
  }

  subtract(other) {
    return new Vector(this.x - other.x, this.y - other.y);
  }

  multiply(scalar) {
    return new Vector(this.x * scalar, this.y * scalar);
  }

  divide(scalar) {
    return new Vector(this.x / scalar, this.y / scalar);
  }

  normalize() {
    return this.divide(this.length);
  }

  floor() {
    return new Vector(Math.floor(this.x), Math.floor(this.y));
  }
}

Vector.zero = new Vector(0, 0);

Vector.InRectangle = function* (topLeft, bottomRight, xJump = 1, yJump = 1) {
  for (let y = topLeft.y; y < bottomRight.y; y += yJump) {
    for (let x = topLeft.x; x < bottomRight.x; x += xJump) {
      yield new Vector(x, y);
    }
  }
};
{% end %}

Things are pretty simple with a `add` and `subtract` function for combining
vectors with each other. I also created multiply and divide functions which will
modify a `Vector` by a given scalar. I added a length property (which gets
translated by Babel at compile time) to calculate the length of a Vector. And
lastly I created normalize for dividing a vector by its length (used for
directions) and a floor function to coerse a `Vector` to integers.

I also added a static `zero` property for a global zero only `Vector` and a
helper function for enumerating all of the points in a rectangle defined by a
`topLeft` and `bottomRight` `Vector`. 

I spent a bunch of time porting much of the vector math to use the new `Vector`
class, but I'm sure I missed some. I will proceed by porting any code I come
across, but I won't worry too much about catching all of it for now. An example
of code that looks better now would be the `cutTerrain` function. Instead of
keeping separate variables and looping over multiple axes, the variables are
kept together, and a single loop is used. There are more variables overall but
the code more closely matches what is going on mathematically, so I think it is
more maintainable.

{% code(lang="javascript") %}
export function cutTerrain(x, y, r) {
  let center = new Vector(x, y);
  let radius = new Vector(r, r);
  let topLeft = center.subtract(radius).floor();
  let bottomRight = center.add(radius).floor();

  for (let pixel of Vector.InRectangle(topLeft, bottomRight)) {
    let offset = center.subtract(pixel);
    if (offset.length > r) continue;
    setTerrainAt(pixel.x, pixel.y, false);
  }
}
{% end %}

## Camera Shake

I pulled the camera update code into its own module as a part of the
`eventManager` rework. In the process I also took the time to implement screen
shake which is a single number that adds a random amount of jiggle to the camera
position which shrinks rapidly over time. I added a `shakeCamera` function which
sets the shake amount and modified camera update to decay the shake each frame.
I then pulled the `cameraPosition` into its own variable and created a `cameraX`
and `cameraY` variable which takes the shake into account.

{% code(lang="javascript") %}
let previousCameraPosition = 0;
let cameraPosition = 0;
let shake = 0;

export let cameraY = 0;
export let cameraX = 0;

export function shakeCamera(amount) {
  shake = amount;
}

Update.Subscribe(() => {
  shake *= shakeFalloff;

  let vy = cameraPosition - previousCameraPosition;
  previousCameraPosition = cameraPosition;
  cameraPosition += vy * cameraMomentum;

  if (player.position.y > cameraPosition + 96) {
    let cameraDiff = player.position.y - (cameraPosition + 96);
    cameraPosition += cameraDiff * cameraLag;
  }

  if (player.position.y < cameraPosition + 32) {
    let cameraDiff = player.position.y - (cameraPosition + 32);
    cameraPosition += cameraDiff * 0.2;
  }

  cameraX = Math.random() * shake;
  cameraY = cameraPosition + Math.random() * shake;
});
{% end %}

Then in the explosion code I added a call to `shakeCamera` from the
`newExplosion` function.

{% code(lang="javascript") %}
export function newExplosion(x, y) {
  explosions.push({
    x,
    y,
    r: startingRadius,
    c: 0,
    delay: animationSpeed
  });

  shakeCamera(shakeAmount);
}
{% end %}

I think this further improves the explosion work I did yesterday.

## Collision Improvements

Lastly I spent some time making the collision code match the graphics more
exactly. Before today, the collision test pixels were defined by calculating the
pixels around the radius at a consistent interval. This makes good geometric
sense, but in practice the bomb and player sprites are not perfectly circular,
and the pixels calculated using the old method often jittered and jumped due to
aliasing.

To fix this problem I did two things. First I defined a list of
`standardBorderPixels` which would provide the actual pixel offsets. Second I
added some calls to floor in the physics calculation to emulate the way sprites
are drawn.

{% code(lang="javascript") %}
const standardBorderPixels = [
  new Vector(-3.5, 0.5),
  new Vector(-3.5, 1.5),
  new Vector(-2.5, 2.5),
  new Vector(-1.5, 3.5),
  new Vector(-0.5, 3.5),
  new Vector(0.5, 3.5),
  new Vector(1.5, 3.5),
  new Vector(2.5, 2.5),
  new Vector(3.5, 1.5),
  new Vector(3.5, 0.5),
  new Vector(3.5, -0.5),
  new Vector(3.5, -1.5),
  new Vector(2.5, -2.5),
  new Vector(1.5, -3.5),
  new Vector(0.5, -3.5),
  new Vector(-0.5, -3.5),
  new Vector(-1.5, -3.5),
  new Vector(-2.5, -2.5),
  new Vector(-3.5, -1.5),
  new Vector(-3.5, -0.5),
];
{% end %}

I came to the values in the above list by looking at the pixels in the sprite
and calculating the offset if the center of the ball was in the center of the
sprite. Unfortunately, since SCRIPT-8 uses 8 pixel by 8 pixel sprite sizes, the
exact center of the sprite is not at a full integer offset. This causes the
border offsets to be off by 0.5 pixels.

This greatly simplified the terrain collision resolution code when combined with
the Vector class improvements.

{% code(lang="javascript") %}
function resolveTerrainCollisions(physicsObjects) {
  for (const obj of physicsObjects) {
    let total = Vector.zero;
    let count = 0;
    for (let positionOffset of standardBorderPixels) {
      let testPosition = obj.position.add(positionOffset.floor()).floor();

      if (terrainAt(testPosition.x, testPosition.y)) {
        if (positionOffset.y > 3) {
          obj.grounded = true;
        }
        total = total.add(positionOffset);
        count++;
      }
    }
    if (count == 0) {
      continue;
    }

    let collisionPosition = total.divide(count);
    let collisionDistance =  collisionPosition.length;
    let collisionDirection = collisionPosition.divide(collisionDistance);
    let displacement = obj.radius - collisionDistance;
    obj.position = obj.position.subtract(collisionDirection.multiply(displacement * 0.3));
  }
}
{% end %}

Importantly I added two calls to floor in the `testPosition` calculation which
corrects for the actual sprite rendering logic. Now the physics calculations
perfectly match the sprite position instead of being slightly larger and offset
to the bottom right.

And that is about it! I made a bunch of other simple code refactoring changes to
clean things up and add some organization. Next up I will finally get to the
game over screen and porting 8Bomb back to SCRIPT-8. Afterward I have some ideas
for how to add networking and multiplayer to the game. This will be a big change
probably split over multiple posts, but may make the game a lot more fun.

Till tomorrow,  
Keith
