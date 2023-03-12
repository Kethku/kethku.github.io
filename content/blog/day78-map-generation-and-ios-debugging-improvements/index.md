+++
title = "Day78 - Map Generation and iOS Debugging Improvements"
description = "Refactored the map rendering and enabled some basic terrain generation"
date = 2019-05-08

[extra]
project = "robot"
+++

![Todo](./todo.svg)

Today I worked on cleaning up the isometric map rendering code, added some very
basic terrain generation, and improved the iOS debugging experience from Visual
Studio.

## Map Rendering

Last time I left off with a manually created terrain and some simple animation
across that terrain. To clean that code up I pulled most of the map management
logic out into a `MapManager` class which contains the map array, entities, and a
simple algorithm for randomly generating both.

Because my current tileset is somewhat limited and the fact that isometric tiles
cannot render all geometries properly, I had to be careful how I built the map.
What I eventually landed on was to first generate a constrained monotonically
downward sloping (I'll explain in a moment) height map, and then used a lookup
table to pick which sprite to render at each map location.

This two step approach lets me reason about the map outside of the particular
tiles, and then abstract away picking which tile would make sense for a given
geometry.

{% code(lang="c#") %}
public void RandomizeMap() {
    int[][] heightMap = new int[Radius][];
    for (int x = Radius - 1; x >= 0; x--) {
        heightMap[x] = new int[Radius];
        for (int y = 0; y < Radius; y++) {
            if (y == 0 || x == Radius - 1) heightMap[x][y] = 0;
            else {
                int upParent = heightMap[x + 1][y - 1];
                int leftParent = heightMap[x][y - 1];
                int rightParent = heightMap[x + 1][y];

                if (leftParent == upParent && rightParent == upParent && random.Next(30) == 1) {
                    heightMap[x][y] = leftParent - 1;
                } else {
                    heightMap[x][y] = new[] { leftParent, upParent, rightParent }.Min();
                }
            }
        }
    }
{% end %}

The first step was to layout the height of the terrain. The basic idea is that
since I can't render slopes angling away from the camera, the slopes must all
angle toward the camera. To achieve this I loop over each height index and
randomly decrease the height if the indexes above are all the same. Otherwise I
set the index to the minimum of the above indexes so as to prevent the slopes
from getting too steep.

With the heightmap out of the way I use a lookup table in a similar way to
drawing marching squares terrains. I normalize 4 values by subtracting the
smallest one from each and set the indexed Tiles value into the map array.

{% code(lang="c#") %}
Dictionary<(int left, int up, int right, int down), Tiles> SlopeLookup = new Dictionary<(int, int, int, int), Tiles> {
    [(0, 0, 0, 0)] = Tiles.Dirt,
    [(1, 1, 0, 0)] = Tiles.SlopeLeft,
    [(1, 1, 1, 0)] = Tiles.SlopeDown,
    [(0, 1, 1, 0)] = Tiles.SlopeRight,
    [(0, 1, 0, 0)] = Tiles.SlopeUp
};

TileMap = new (Tiles, int)[heightMap.Length - 1][];
for (int x = 0; x < TileMap.Length; x++) {
    TileMap[x] = new (Tiles, int)[heightMap[x].Length - 1];
    for (int y = 0; y < TileMap[x].Length; y++) {
        int left = heightMap[x][y];
        int up = heightMap[x + 1][y];
        int right = heightMap[x + 1][y + 1];
        int down = heightMap[x][y + 1];

        int lowest = new[] { left, up, right, down }.Min();
        Tiles tile = SlopeLookup[(left - lowest, up - lowest, right - lowest, down - lowest)];
        TileMap[x][y] = (tile, lowest);
    }
}
{% end %}

Finally I use the height map calculated above to randomly place tree entities.
Since the entities look weird on a non-flat tile, I also lookup the map array to
make sure I'm not placing trees on slopes.

{% code(lang="c#") %}
Entities.Clear();
for (int i = 0; i < 10; i++) {
    int x = random.Next(Width - 1);
    int y = random.Next(Height - 1);

    if (TileMap[x][y].tile == Tiles.Dirt) {
        var tile = Tiles.Tree;
        var position = new Vector2(x, y);

        Entities.Add((tile, position));
    }
}
{% end %}

Finally to test out the random generation, I added a simple update method to the
Map Manager which checks the touch controls to see if a touch point is present,
and regenerates the whole thing. The resulting app looks like this:

![Random Isometric Map](RandomIsometricMap.gif)

## iOS Debugging

The rest of my time the past couple of days was spent stumbling around trying to
improve the debugging experience on my actual iOS device. My previous strategy
of using the plugged in iPhone made things much to difficult because some
combination of the cord and usb ports I was using caused things to be extra
spotty.

Eventually I discovered that xCode, the Mac dev environment allows for debugging
iOS devices remotely. I don't have the device close, so I don't have screen caps
of the actual UI. However I got it working by opening xCode, using the windows
menu to open the devices window, connecting my iPhone to the computer using a
usb cord, and then checking the debug this device over the network button.

With that setting set, I was able to disconnect the iPhone, and Visual Studio
would continue to debug over the network to the Mac, which would then turn
around and send the app to my iPhone over the network. The combined hops do slow
the actual deployment down, but I found it to be much more reliable.

Thats it for today. I had a 2 day gap between last post and this one, which is
pretty disappointing, but I'm hopeful to get back on track. Next up is to build
some kind of path finding or entity movement paying attention to the actual
tiles. I'm running into design blocks however, so I might go back and work on
the Tetris Attack clone a bit more. We'll see.

Till tomorrow,  
Keith
