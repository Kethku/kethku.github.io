+++
title = "Day34 - Eval Error Sources"
description = "Regexes to find the location of an error in javascript from the trace text"
date = 2019-03-12

[extra]
project = "8bomb"
+++

Today I spent some time starting a PR for SCRIPT-8 to add jump to source support
for the error messages. I didn't end up finishing the PR because I wasn't sure
how the event routing should work, but I did write some regexes to pull out the
source location and route the position data through to the error messages. The
PR currently writes the position data to the log, but I am hopeful that with
Gabriel's help it shouldn't be that hard to finish it up.

## Why Not error-stack-parser

As far as I can tell, the recommended way for parsing error trace text is to
use the [error-stack-parser](https://github.com/stacktracejs/error-stack-parser)
library. Unfortunately in our case, the source location we care about exists
within an eval call. There is an [open
issue](https://github.com/stacktracejs/error-stack-parser/issues/32) for
error-stack-parser for this very case and as far as I can tell the package
currently throws the data away.

The issue of parsing trace text is complicated by the fact that there isn't a
standard format. The APIs grant access to the text, but do not say anything
about what the text should be. As such each browser has a slightly different
format. Luckily as far as I can tell, they all provide the data.

I did some further searching on the web, but never found anything to fix this,
so I rolled up my sleeves and got out my regex references.

## Gathering Data

The first step was to gather example traces for each of the browser I have
access to.

Chrome:
{% code() %}
TypeError: Cannot read property 'toString' of undefined
    at print (print.js:7)
    at print (index.js:72)
    at draw (eval at <anonymous> (eval at evalCode (Iframe.js:644)), <anonymous>:2:3)
    at Iframe.drawUserGraphics (Iframe.js:397)
    at timerCallback (Iframe.js:702)
    at tick (interval.js:10)
    at timerFlush (timer.js:61)
    at wake (timer.js:71)
{% end %}

Firefox:
{% code() %}
print@http://localhost:3001/static/js/bundle.js:69091:7
print@http://localhost:3001/static/js/bundle.js:68699:7
draw@http://localhost:3001/static/js/bundle.js line 67598 > eval line 6 > eval:2:3
drawUserGraphics@http://localhost:3001/static/js/bundle.js:67340:9
timerCallback@http://localhost:3001/static/js/bundle.js:67650:11
tick@http://localhost:3001/static/js/bundle.js:1475:5
timerFlush@http://localhost:3001/static/js/bundle.js:1580:40
wake@http://localhost:3001/static/js/bundle.js:1590:5
{% end %}

Edge:
{% code() %}
TypeError: Unable to get property 'toString' of undefined or null reference
  at print (http://localhost:3001/static/js/bundle.js:69091:3)
  at print (http://localhost:3001/static/js/bundle.js:68699:7)
  at draw (eval code:2:3)
  at drawUserGraphics (http://localhost:3001/static/js/bundle.js:67340:9)
  at timerCallback (http://localhost:3001/static/js/bundle.js:67650:11)
  at tick (http://localhost:3001/static/js/bundle.js:1475:5)
  at timerFlush (http://localhost:3001/static/js/bundle.js:1580:40)
  at wake (http://localhost:3001/static/js/bundle.js:1590:5)
{% end %}

From these logs, I was able to determine that the code position existed at the
end of a single line in every case.

## Writing Regexes

I hate writing Regular Expressions. I think they are clunky and lend themselves
to a write only read never coding style. It turns out that most people hate them
and are bad at writing them, but for some cases they are a necessary evil. When
I am forced to dabble, I tend to turn toward a regex authoring tool such as
[regex101](https://regex101.com/). I find the experience of writing regexes much
less painful when I can see what gets recognized in real time and when the
reference information is visible and handy.

Writing each of the browser trace regexes was as simple as copying the
associated log into the test string box, and copying the line with the eval
position into the regex field. This doesn't work as is because I need the code
to be more generalized, and there are special characters, so I added backslashes
before the parentheses, replaced the numbers with `\d+` to match one or more
digits, and replaced the file names with `.+` to match one or more characters.
Finally I surrounded the actual line and column `\d+`-s with un-escaped
parenthesis in order to mark them for extraction. And that was it! The regexes I
landed on were as follows:

{% code() %}
chrome: /\(eval at <anonymous> \(eval at evalCode \(.+:\d+\)\), <anonymous>:(\d+):(\d+)\)/
firefox: /line \d+ > eval line \d+ > eval:(\d+):(\d+)/
edge: /\(eval code:(\d+):(\d+)\)/
{% end %}

Pretty straight forward.

As I mentioned above, I was unable to figure out how to hook up the Redux parts
of the PR, but I did get part of the way there. After writing the regexes, I
needed to determine which browser is being used to pick the correct one. I
searched a bit for a vanilla javascript approach, but eventually landed on a
simple package called
[detect-browser](https://www.npmjs.com/package/detect-browser) because there
were enough edge cases to make is frustrating.

The actual guts of the code was mostly a bit of plumbing from the iframe to the
outer window, and to surface the position data in an event when the error list
item gets clicked. Nothing particularly interesting or confusing.

I pushed my code and created [this
PR](https://github.com/script-8/script-8.github.io/pull/270). Moments ago as I
was writing this, Gabriel told me he would be happy to help push the PR the rest
of the way, so I mark today's work as a success!

Till tomorrow,  
Keith
