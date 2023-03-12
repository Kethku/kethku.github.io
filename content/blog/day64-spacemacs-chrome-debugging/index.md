+++
title = "Day64 - Spacemacs Chrome Debugging"
description = "Added chrome debugging layer to my spacemacs config"
date = 2019-04-14
+++

Today I spent some time researching and configuring a chrome debugging package
in my emacs config. The particular package I went with is called
[indium](https://github.com/NicolasPetton/Indium) and adds support debugging
javascript applications from within emacs. It achieves this using the chrome
debugger protocol which allows applications outside of chrome to manipulate the
chrome debugger by steping, navigating frames, and accessing local variables.

## Spacemacs

In emacs I use the spacemacs project to initialize sane defaults for languages,
setup vim emulation, and in general make emacs a better environment to live in.
Spacemacs configures itself using "layers" which are small snippets of emacs
lisp code which specify packages to pull down and install as well as
configuration for the keybindings and functionality of said packages.

To add support for Indium I created a new Indium layer by creating an `indium`
in my `~/.spacemacs.d/layers` folder and adding a packages.el file to it. In it
I added this lisp:

{% code(lang="lisp") %}
(defconst indium-packages
  '(indium))

(defun indium/init-indium ()
  (use-package indium
    :config
    (progn
      (spacemacs/declare-prefix-for-mode 'indium-debugger-mode "d" "debugger")
      (spacemacs/set-leader-keys-for-minor-mode 'indium-debugger-mode
        "dy" 'indium-debugger-previous-frame
        "do" 'indium-debugger-next-frame
        "dl" 'indium-debugger-step-into
        "dj" 'indium-debugger-step-over
        "dh" 'indium-debugger-step-out
        "dr" 'indium-debugger-resume)
      (spacemacs/set-leader-keys-for-major-mode 'js2-mode
        "l" 'indium-launch
        "br" 'indium-repl-switch-from-buffer))))
{% end %}

In short this specifies that my layer depends on a package called "indium" and
adds some keybinding support for the various functions I use in Indium. In
practice the main places I use the package are for jumping to the correct source
file in spacemacs when a breakpoing or error is thrown, navigating the debugger
when paused, and accessing the repl.

Then to add this layer to spacemacs, I just added indium to my layer list in my
`~/.spacemacs.d/init.el` file.

{% code(lang="lisp") %}
(defun dotspacemacs/layers ()
  (setq-default
   dotspacemacs-distribution 'spacemacs
   dotspacemacs-enable-lazy-installation 'unused
   dotspacemacs-ask-for-lazy-installation t
   dotspacemacs-configuration-layer-path '()

   dotspacemacs-configuration-layers
   `(indium
     graphviz
     csharp
     vimscript
     csv
{% end %}

## A Note About Editors

Unfortunately I wasted a bunch of time today also trying to chase down a weird
bug in my emacs configuration related to the portable dumper and it's
interaction with spacemacs. As some background, emacs is a huge monstrosity of a
program with a ton of legacy. The vast majority of it's source code is written
in emacs-lisp, a dialect of lisp with a ton of warts and cruft. As a result, it
tends to be pretty slow unless some crazy hacks are used. One that might not
immediately be obvious is that emacs runs a ton of configuration in the default
which gets dumped into a memory snapshot and reloaded on startup every time.
This works great for the base configuration but gets slow again if you use a
huge configuration file like spacemacs. 

To get around this I use an experimental feature in spacemacs which adds support
for dumping a personalized version of emacs with all of my configuration and
loading that instead of the distributed one. Achieving this requires a bunch of
bending over backwards including recompiling emacs to support larger dump files,
reworking the `init.el` to manually force load all of the configuration before
dumping, and fiddling with various spacemacs settings to fix everything that
gets broken when you fly too close to the sun like I have. Things start breaking
in weird ways. Various parts of emacs think they are running in a terminal when
they aren't, https calls cause emacs to crash without recovery, and some things
reload themselves on file open no matter how hard I try.

The fact that this affront to humanity is required to make emacs usable is sad
and concerning. I love the premise behind emacs, but I worry about how much work
is required to make the experience anywhere close to plug and play. When paired
with the fact that relatively few people go through the effort to get emacs
fully configured, I begin to wonder if there are better options. I love the
culture of absolute reconfigurability that emacs users seem to cherish, but I
worry that emacs is just too old and has too much baggage. Here are some other
options I am eyeing in a post emacs world:

Before using emacs I tried for a while to use vim. I switched to emacs after
getting incredibly frustrated with vim's configuration and performance
characteristics. These days however [Neovim](https://neovim.io/) has fixed most
of these issues by adding a async-first plugin api that uses messagepack to
allow any language as a plugin language. I may spend some time playing around
with neovim soon.

On the rust side of things, [Xi](https://github.com/xi-editor/xi-editor) is
incredibly promising. The project has been around for a while with relatively
little outward progress, but the core of the editor is nothing short of
incredible. I don't claim to understand the
[datastructures](https://github.com/xi-editor/xi-editor/blob/master/docs/docs/rope_science_00.md)
at Xi's core at all, but what I do understand is fascinating. Xi's team is
clearly very invested in making a performance oriented text editor which should
help solve some of the problems I encounter in emacs with performance. Xi also
has a strong focus on cross platform support which is refreshing. As an example
of the crazyness going on over there, the main developer has developed an entire
[UI framework](https://github.com/xi-editor/druid) for building fast
applications on windows and has spent countless hours ensuring that the resize
logic is flawless. The guy is nuts. The Xi folks claim that they are building an
editor for the next 25 years, and I believe them.

Lastly, the front runner among my peers seems to be [Visual Studio
Code](https://code.visualstudio.com/). I've spent some time with Visual Studio
Code, but was turned off by it's configuration scheme which reminded me too much
of Atom where configuration is highly constrained. Since that time however they
have continued to improve the story and add support for language after language
becoming best in class in each. Visual Studio Code feels like a fresh start from
the old legacy of Visual Studio while preserving their attention to detail and
engineering rigor.

The point I'm trying to make with this list is that these new editors are making
bold steps while emacs seems to be plodding along. Emacs has been around for
decades, but instead of being decades ahead I feel as though emacs is falling
behind. I switched to emacs with the hope that since it has survived this long,
maybe it will continue to survive longer. I'm beginning to realize that Emacs is
doing just that: Surviving. It is not however Thriving and that makes me sad.

## Book Keeping

Weird post today with a stronger focus on writing. I'm interested in using this
blog as a place to talk about what I'm passionate about as well as my coding
projects. I spend a ton of time in my editors and am never quite satisfied with
them. Maybe writing about it will help me get a clear idea of what needs done
next.

Today is also the first day back from my first 3 day break. I was unable to
write the past 3 days because I was attending a friend's wedding. Hopefully
breaks like that won't be too frequent.

Till tomorrow,  
Kaylee
