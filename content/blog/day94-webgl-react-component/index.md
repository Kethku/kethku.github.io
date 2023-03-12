+++
title = "Day94 - WebGL React Component"
description = "Wrapping game engine in a React component"
date = 2019-10-27

[extra]
project = "ta"
+++

![Todo](./todo.svg)

Today I added React UI support to my game engine by wrapping the webgl canvas in a React component and using hooks to
detect mounts and unmounts of the canvas element. This should make menus and even in game UI much simpler to implement
while still preserving the performance and graphics capabilities I've been working on.

## Hooks

The newish versions of React support hooks as a new pattern for dealing with effects and state. They work by exposing
some tricky functions which "hook" into the React rendering code and store some state from render call to render call.
The particular hook I am using is called `useEffect` and it runs a given callback whenever the containing React function
component is updated, or unmounted.

{% code(lang="typescript") %}
export function GameComponent() {
  useEffect(() => {
    return function cleanup() {
      running = false;
    }
  }, [])
  return <canvas ref={canvasMounted} touch-action="none" />;
}

export const CanvasMounted = new EventManager<[HTMLCanvasElement]>();
async function canvasMounted(newCanvas: HTMLCanvasElement) {
  await Promise.all(baseScreen.Setup.Publish());

  currentScreen = playScreen;
  baseScreen.Setup.Publish();
  playScreen.Setup.Publish();

  CanvasMounted.Publish(newCanvas);
  window.requestAnimationFrame(loop);
  running = true;
}
{% end %}

Here I use the effect hook and return a function from inside the hook which tracks unmount events to stop the draw loop.
To track updates, since I need a reference to the actual canvas HTML element, I use the React Ref attribute to pass a
callback which gets called with the mounted element every time it changes.

Using my standard event manager pattern I publish a new CanvasMounted event with the mounted element to hook in the
webgl specifics.

{% code(lang="typescript") %}
Setup.Subscribe(async () => {
  CanvasMounted.Subscribe(async (newCanvas) => {
    canvas = newCanvas;
    resize();
    console.log(newCanvas);
    gl = newCanvas.getContext("webgl", {alpha: false});
    spriteProgram = twgl.createProgramInfo(gl, [vertex, fragment]);
    gl.useProgram(spriteProgram.program);
    spriteArrays = {
      a_coord: {numComponents: 2, data: new Float32Array(maxCount * 2), drawType: gl.DYNAMIC_DRAW},
      a_position: {numComponents: 3, data: new Float32Array(maxCount * 2), drawType: gl.DYNAMIC_DRAW},
      a_texcoord: {numComponents: 2, data: new Float32Array(maxCount * 2), drawType: gl.DYNAMIC_DRAW},
      a_rotation: {numComponents: 1, data: new Float32Array(maxCount * 2), drawType: gl.DYNAMIC_DRAW},
      a_dimensions: {numComponents: 2, data: new Float32Array(maxCount * 2), drawType: gl.DYNAMIC_DRAW},
      a_center: {numComponents: 2, data: new Float32Array(maxCount * 2), drawType: gl.DYNAMIC_DRAW},
      a_scale: {numComponents: 1, data: new Float32Array(maxCount * 2), drawType: gl.DYNAMIC_DRAW},
      a_color: {numComponents: 4, data: new Float32Array(maxCount), drawType: gl.DYNAMIC_DRAW},
      indices: {numComponents: 3, data: new Uint16Array(maxCount * 2), drawType: gl.DYNAMIC_DRAW}
    };
    bufferInfo = twgl.createBufferInfoFromArrays(gl, spriteArrays);
    textures = await setupTextures(gl, imageURLs);
    gl.enable(gl.BLEND);
    gl.blendFunc(gl.SRC_ALPHA, gl.ONE_MINUS_SRC_ALPHA);
    gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MIN_FILTER, gl.NEAREST_MIPMAP_NEAREST);
    gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MAG_FILTER, gl.NEAREST);
  });
});
{% code %}

So here instead of running the webgl setup code once, I run it every time a new canvas is mounted allowing the graphics
to setup and restart for another game after the canvas no longer exists. To use the new component I simply mount it to 
a known div and the rest is handled automatically.

{% code(lang="typescript") %}
import { GameComponent } from "./game";

ReactDOM.render(
  <GameComponent />,
  document.getElementById("app")
);
{% end %}

In the future I will add multiple screens and dynamically swap between them depending on what menu buttons and links are
clicked, but for now the proof of concept is complete and the game is rendering properly again. I did some research and
came across [nes-react](https://github.com/bschulte/nes-react) which is an awesome react component library that provides
html elements styled to look like NES games or graphics. I plan on using it to build the lobby and game host windows
once multiplayer is more fleshed out.

Till tomorrow,  
Keith
