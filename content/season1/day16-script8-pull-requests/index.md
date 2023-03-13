+++
title = "Day16 - SCRIPT-8 Pull Requests"
description = "Miscellanious Pull Requests to SCRIPT-8"
date = 2019-02-22

[extra]
project = "8bomb"
+++

Today I spent my time working on 3 pull requests for various issues in the
[SCRIPT-8](https://github.com/script-8/script-8.github.io) GitHub repo. None of
them were really related to a project in particular, but I used the simple tasks
as a forcing function to more closely understand the code base. Hopefully this
will get me in good position to making more complicated changes later on.

Yesterday I visited the Fantasy Console Discord Server to see if there was any
activity. Turns out people chat on the server somewhat frequently, so I decided
to pop in and say hello. I mentioned that I was working on
[8Bomb](../../projects/8bomb/) and that I would be willing to help contribute in
any way I could. Gabriel pointed me to some issues which I then made pull
requests for today.

## Vertical Sprite Flip

The first of the three was very simple. SCRIPT-8 supports drawing flipped
sprites by passing a boolean to the sprite function to indicate whether to flip
horizontally over the vertical axis. There was an
[issue](https://github.com/script-8/script-8.github.io/issues/170) created
requesting an extra boolean for flipping vertically as well.

This change was pretty similar to my previous `getPixel` and `setPixel` changes, so
it went by pretty quickly.

{% code(lang="javascript") %}
sprites[spriteIndex].slice(0, 8).forEach((cells, rowIndex) => {
  cells.split('').forEach((color, colIndex) => {
    if (color !== ' ') {
      const clamped = clamp(+color - darken, 0, 7)
      ctx.fillStyle = colors.rgb(clamped)
      ctx.fillRect(
        Math.floor(x) + (flipHorizontal ? 7 - colIndex : colIndex),
        Math.floor(y) + (flipVertical ? 7 - rowIndex : rowIndex),
        1,
        1
      )
    }
  })
})
{% end %}

The important part was adding an identical inline condition that checks
`flipVertical` and subtracts the `rowIndex` from 7 if a vertical flip was requested.
Easy peasy. Gabriel merged the
[PR](https://github.com/script-8/script-8.github.io/pull/212) pretty quickly.

## Token Count

The second PR was a bit more complicated. Fantasy consoles frequently impose
character limits upon programs written for them to simulate the space
constraints that existed on the old systems they are based on. Gabriel has
mentioned that he is interested in adding a similar thing to SCRIPT-8, and had
laid the groundwork via a simple minified source character count UI. This works,
but tends to encourage poor coding style since character counts can be lowered
by changing variables names etc.

[Pico-8](https://www.lexaloffle.com/pico-8.php) generally concidered the best of
all the fantasy consoles uses a token limit instead, so all variables
reguardless of size count equally. Gabriel created an issue pointing toward
[acorn.js](https://github.com/acornjs/acorn) which is a parser for JavaScript
written in JavaScript and suggesting that the character count recording should
be implemented using it instead of the existing method.

After some searching I eventually found that the source code UI was located in a
React container called Output. Previously there was a fair amount of repeated
code and specialization to minify the source before counting.

{% code(lang="javascript") %}
getSize() {
  const { game, songs, chains, phrases, sprites, map } = this.props

  const gameText = assembleOrderedGame(game)

  const gameTextLz = lz.compress(gameText)
  const art = JSON.stringify({ sprites, map })
  const artLz = lz.compress(art)
  const music = JSON.stringify({ phrases, chains, songs })
  const musicLz = lz.compress(music)

  const sizes = [
    ['code', gameText, gameTextLz],
    ['art', art, artLz],
    ['music', music, musicLz]
  ]

  return (
    <ul>
      {sizes.map((d, i) => (
        <li key={i}>
          {d[0]}: {numberWithCommas(d[1].length)}/
          {numberWithCommas(d[2].length)}
        </li>
      ))}
      <li>
        total: {numberWithCommas(sum(sizes.map(d => d[1].length)))}/
        {numberWithCommas(sum(sizes.map(d => d[2].length)))}
      </li>
    </ul>
  )
}
{% end %}

Instead of compressing the text and counting the size that way I used acorn's
tokenizer at the suggestion of Gabriel to split the text into tokens, and then
used the token count for the display.

{% code(lang="javascript") %}
import { tokenizer } from "acorn";
const getTokenCount = src => {
  try {
    return numberWithCommas([...tokenizer(src)].length)
  } catch (error) {
    return "ERROR"
  }
}
{% end %}

Since the tokenizer depends on the source being at least somewhat well formed, I
wrapped the acorn call in a `try catch` to keep things happy.

Then I simplified the existing code a bit by storing the data in an object with
useful property names, moving the total element into the same object, and using
some lodash trickery to loop over the object and call `getTokenCount` with the
correct arguments.

{% code(lang="javascript") %}
getSize() {
  const { game, songs, chains, phrases, sprites, map } = this.props

  const code = assembleOrderedGame(game)
  const art = JSON.stringify({ sprites, map })
  const music = JSON.stringify({ phrases, chains, songs })
  const total = code + art + music

  const assets = {
    code,
    art,
    music,
    total
  };

  return (
    <ul>
      {_.toPairs(assets).map(pair => ((name, code) => (
        <li key={name}>
          {name}: {getTokenCount(code)}
        </li>
      ))(...pair))}
    </ul>
  )
}
{% end %}

I think these changes reduced the repeated code, and made things somewhat more
understandable. It strayed a bit further than I normally like to from just doing
the minimal change possible, but Gabriel didn't seem to mind because he merged
the [PR](https://github.com/script-8/script-8.github.io/pull/213) shortly after
I posted it.

## Scroll Bug

The last PR I created was the most complicated. As mentioned above, SCRIPT-8
uses React to render the UI. It also uses a library called
[CodeMirror](https://codemirror.net/) for the text editor. There was already
support for preserving the scroll position across tab changes, but there was an
issue to add support for preserving the cursor position as well.

This seems like an easy change, but SCRIPT-8 also uses a library called
[Redux](https://redux.js.org/) for state management, which formalizes how the
app stores and modifies state. This makes the application easier to reason
about, but makes it much harder to change the state manipulation in the app.

With Redux, all state changes must be done via specialized actions which are
like events that describe the change to be made. This means if a new type of
action is introduced, it must be added in many different places.

My initial attempt at the change added an entirely new action for cursor
position storage. After chatting in the discord for a bit, we decided to instead
modify the existing `scrollInfo` action to also contain the cursor position. In
the process I also renamed the `scrollInfo` action to be `scrollData` to
differentiate it from the `scrollInfo` in CodeMirror.

The actual changes were pretty small, just record the cursor position any time
the editor unmounts, and restore the cursor position when the code editor is
remounted. The code in the code editor component had a fair amount of
repitition, so I will show one example of the store and load routines.

{% code(lang="javascript") %}
// If found, restore scroll data.
const { scrollData } = activeGame
if (scrollData) {
  this.codeMirror.scrollTo(scrollData.left || 0, scrollData.top || 0)
  this.codeMirror.setCursor(scrollData.cursorPosition)
}
{% end %}

If scrollData is a property in the activeGame object which contains the state,
then the codeMirror object is told to scroll to the store position and set the
cursor to the stored position.

{% code(lang="javascript") %}
componentWillUnmount() {
  window.removeEventListener('keyup', this.hideSlider)
  const activeGame = getActive(this.props.game)
  const scrollInfo = this.codeMirror.getScrollInfo()
  const cursorPosition = this.codeMirror.getCursor()
  const scrollData = { top: scrollInfo.top, left: scrollInfo.left, cursorPosition }
  this.props.setScrollData({ scrollData, tab: activeGame.key })
  this.props.updateHistory({
    index: activeGame.key,
    history: this.codeMirror.getDoc().getHistory()
  })
}
{% end %}

Similarly if the component is unmounted, the scroll position and cursor position
are stored and set using the setter passed to the component.

That was about it, getting to the point of understanding what the issue was and
how to make the intended changes took a while, but in the end I am pretty sure I
did it correctly. As of writing, this
[PR](https://github.com/script-8/script-8.github.io/pull/214) has not yet
merged, but Gabriel mentioned to me that he was planning on taking a look at it
shortly.

I find the process of contributing to an open source project very satisfying as
the nitty gritty pieces such as build system, documentation etc are for the most
part already ironed out, so what I get to work on is a cohesive app already
working. It also gives me the chance to practice reading and understanding code
that I didn't write. My hope is that as I do this more frequently my ability to
read and understand code for my work will improve. Time will tell.

Tomorrow I am going skiing, so I will likely have less time to work on my daily.
I will try to come up with something though.

Till tomorrow,  
Kaylee
