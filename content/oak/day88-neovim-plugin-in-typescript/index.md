+++
title = "Day88 - Neovim Plugin in Typescript"
description = "Writing a plugin for Neovim in Typescript"
date = 2019-10-21
+++

Recently I switched editors from Spacemacs (a vim mode distribution of emacs) to Neovim. I made the change due to
frustration with how complicated my editor was in emacs, and honestly the experience has been wonderful. That discussion
is for another time however.

Having made the switch, I have been interested in playing around with Neovim's headline feature: the remote plugin API.
The jist is that Neovim provides a general purpose API in the form of a MessagePack protocol. Messagepack is a binary
format which allows remote process communication in an efficient and language agnostic way. To make life easier, the
maintainers of Neovim have created a set of language integrations which set things up for you. I've decided to develop a
plugin using the nodejs integration. I will capture here the basic template for how to get things working and some tips
and tricks I learned along the way.

## Plugin Format

Neovim's language client integrations are very particular about how and where you layout your plugin file structure. If
the structure is not just so, Neovim won't know where your remote plugin is located. Further, so called remote plugins
must be registered before you can use them which adds another layer of complexity.

I found it useful to use a plugin manager, and add my local plugin to it to ensure that vim knows that my plugin exists.
I use [Dein](https://github.com/Shougo/dein.vim), but any vim plugin manager should work.

```vim
call dein#add('c:/dev/Projects/vim-balsamic')
```

With my custom plugin folder added, I created a folder structure matching this pattern: 

```
Project Root > rplugin > node > Plugin Name > Javascript Project
```

- Project Root represents the name of the plugin we added to the plugin manager.
- rplugin indicates that this plugin has a remote script to run in the background.
- node indicates that the remote script is a nodejs project and that Neovim should use the Neovim client.
- Javascript Project represents the same name as above, but allows for multiple remote node scripts per project.

In the inner Javascript Project folder (in my case vim-balsamic), I made a simple nodejs project. In particular I use
typescript to make life easier

```
qcK7y:dist\
0Oubx:lib\
Tyhqo:node_modules\
aSW1g:.gitignore
5IzLv:package.json
T7EH2:tsconfig.json
itiAC:yarn.lock
```

(Note: the weird characters before file/folder names are related to my plugin. I will discuss them soon.)

The package.json file must contain the path to the entry point script so that Neovim knows which file to run with
node.js. It is also important that the package depend on the `neovim` package to enable the MessagePack communication.

``` json
{
  "name": "vim-balsamic",
  "version": "0.0.1",
  "description": "Super powered acetic file exploration in vim",
  "main": "dist/index.js",
  "author": "Kaylee Simmons",
  "devDependencies": {
    "@types/node": "^12.7.7",
    "neovim": "^4.5.0"
  },
  "dependencies": {
    "@types/fs-extra": "^8.0.0",
    "fs-extra": "^8.1.0"
  }
}
```

## Plugin Writing

The documentation for the Neovim node client can be found [here](https://github.com/neovim/node-client). However I found
it somewhat confusing. The cleanest method I found was to define a class and use attributes to hook everything up.

``` ts
@Plugin({ dev: false })
export default class BalsamicPlugin {
  constructor(public nvim: Neovim) {  }

  @Command("Balsamic")
  async openParent() {
    const fullFilePath = (await this.nvim.commandOutput("echo expand('%:p')")) + "/" // Query the current file directory path
    if (directoryLookup.has(fullFilePath)) {
      let directory = directoryLookup.get(fullFilePath);
      createDirectoryBuffer(this.nvim, directory.parent);
    } else {
      const fullDirectoryPath = path.resolve(path.join(fullFilePath, '..'));
      createDirectoryBuffer(this.nvim, fullDirectoryPath);
    }
  }

  @Command("BalsamicOpen")
  async openCurrentLine() {
    let line = await this.nvim.getLine();
    let parsedLine = parseLine(line);
    if (parsedLine) {
      let { id, name } = parsedLine;
      let fullDirectoryPath = await this.nvim.commandOutput("echo expand('%:p')");

      if (itemIsDirectory(name)) {
        createDirectoryBuffer(this.nvim, path.join(fullDirectoryPath, name));
      } else {
        if (initialState.has(id)) {
          let file = initialState.get(id);
          await this.nvim.command(`e ${file}`);
        } else {
          await this.nvim.outWriteLine("File does not exits.");
        }
      }
    }
  }

  @Command("BalsamicCommit")
  balsamicCommit() {
    commitChanges(this.nvim);
  }

  @Command("BalsamicExecute")
  balsamicExecute() {
    executeOperations(this.nvim)
  }
}
```

The plugin attribute indicates that this class should be used as a Neovim plugin and the `dev: false` flag is passed to
prevent Neovim from reloading the script on every command.

Similarly the command attribute defines a method on the plugin class as representing a command with the named passed in.
This exposes that function in Neovim for use. Lastly the constructor for the plugin class takes a Neovim argument which
I store as a public property. This object contains all of the API methods and properties needed for interacting with the
Neovim app.

``` ts
async function tempBuffer(nvim: Neovim, name: string, lines: string[] = [], fileType = "balsamic") {
  nvim.callAtomic([
    await nvim.command("enew"),
    await nvim.buffer.setOption("buftype", "nofile"), // Ensure the buffer won't be written to disk
    await nvim.buffer.setOption("bufhidden", "wipe"), // Close the buffer when done
    await nvim.buffer.setOption("ft", fileType), // Set file type to balsamic or filetype
    await nvim.command("setlocal noswapfile"),
    await nvim.command("0f"),
    await nvim.command(`file ${name.replace(/\\/g, "/")}`), // Change buffer name to match the current file
    await nvim.buffer.setLines(lines, { start: 0, end: -1, strictIndexing: false })
  ]);
  return nvim.buffer;
}
```

This object can be passed to methods like the one above, and called using async await to do operations one after another
efficiently.

Once the plugin is written or at least compiles, running the `UpdateRemotePlugins` command in Neovim will run your
plugin and inspect it to figure out what commands are defined. This way the plugin can be run lazily instead of on
startup slowing down vim.

## Debugging

Crucially it is difficult to really debug or understand what is going on in a remote plugin without some debugger
support. To make this happen, two steps are necessary. First a chrome browser with the Node.js V8 -- inspector Manager
must be running. This can be setup
[here](https://chrome.google.com/webstore/detail/nodejs-v8-inspector-manag/gnhhdgbaldcilmgcpfddgdbkhjohddkj). Second, an
environment variable must be set to tell Neovim to enable nodejs debugging. This can be done with this vim command: 
`:let $NVIM_NODE_HOST_DEBUG = 1` which will set the NVIM_NODE_HOST_DEBUG environment variable. Then when any command is
run which triggers the nodejs plugin, the chrome window will pop the debugger window and attach to the running process.

This makes life significantly easier and removes a lot of the print debugging which would otherwise be necessary.

## My Plugins

I have been working on a file explorer plugin for Neovim using the above techniques. Its not quite ready yet, but I plan
on writing about it soon. Its changed the way I interact with files :)

Till tomorrow,  
Kaylee
