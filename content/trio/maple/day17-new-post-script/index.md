+++
title = "Day17 - New Post Script"
description = "Worked on adding new post script using rust-script"
date = 2023-04-21
+++

Today I worked on adding a new post script to my blogging
setup. I was realizing yesterday that I was procrastinating
on writing my blog post. Usually when this happens my first
reaction is to make it as easy as possible to get the thing
I don't want to do done. This sparks some interest in doing
the task again, and makes it easier next time (assuming I
complete the optimization).

So today I did just that, I worked on writing a simple
script which finds the next day number for the blog post,
makes a new post directory with a passed in name, and makes
an index.md file with the standard header in it. I ended up
using a lightweight script preprocessor called rust-script
so that I could write the logic in Rust using some helper
crates for the casing changes.

## Rust-Script

This is my first time using rust-script for this sort of
thing, but I'm very impressed. The runner parses the rust
code looking for doc comments with cargo code blocks and
uses the contents for the Cargo.toml file. 

~~~rs
//! ```cargo
//! [dependencies]
//! chrono = "0.4.24"
//! convert_case = "0.6.0"
//! ```

use std::io::Write;

use convert_case::{Case, Casing};

fn main() {
    // Go to ./content/trio/maple/ and find all the files and directories that
    // start with "day##-"
    let mut days: Vec<String> = Vec::new();
    for entry in std::fs::read_dir("./content/trio/maple/").unwrap() {
~~~

It then compiles the code using a generated project on the
fly in a hidden directory so that it doesn't have to be
recompiled.

## Convert Case

The code itself is pretty straight forward. Only cool thing
is the use of a crate I found called ConvertCase which
provides easy automatic conversion between different case
styles. The upside is that I can write the title however I
like and it will get converted to the kebab (words with
dashes) and title (Capitalized text with spaces) cases
automatically.

## Copilot

As a fun extra challenge for this task, I decided to rely on
copilot for the majority of the programming. To do this, I
split the task into high level steps I expected to need, and
wrote comments for them as I would normally for documenting
a piece of code, but with some extra detail.

```rs
fn main()
    // Go to ./content/trio/maple/ and find all the files and directories that
    // start with "day##-"

    // Parse the ## from each day and find the max

    // Create the new day name using the passed in commandline argument
    // as the suffix

    // Create a new directory for the new day and create an index.md file
    // in it with a header formatted like so:
    //
    //   +++
    //   title = "Day<number> - <suffix>"
    //   description = ""
    //   date = <date>
    //   +++
    //
    //  With the date being the current date in the format of YYYY-MM-DD
}
```

Then I navigated to each of the blank lines after a comment
and accepted the first copilot suggestion. For example,
copilot completed the following for the first one.

```rs
let mut files = std::fs::read_dir("./content/trio/maple/")
    .unwrap()
    .filter_map(|entry| {
        let entry = entry.unwrap();
        let path = entry.path();
        let file_name = path.file_name().unwrap().to_str().unwrap();
        if file_name.starts_with("day") {
            Some((path, file_name.to_string()))
        } else {
            None
        }
    })
    .collect::<Vec<_>>();
```

Which is close except for an error in the lifetime of the
path file_name. But with a quick fix, the step is written
flawlessly. I did the entire script like that and I estimate
that I had to fix 5 separate bugs in the final code. Imho
this is pretty good and I'm pretty impressed.

## Okeydokey script

With the script written I then used Okeydokey (my directory
script runner that I talk about [here](/trio/hemlock/projects/okeydokey/)).
I already had an okeydokey script for the blog source, so it
was as simple as adding a `new` command with a whole for the
title.

```pwsh
new: e "$(rust-script .\scripts\new.rs "{0}")"
```

Tricky bit here is that I wrote the rust code to output the
index.md path when done setting everything up so that I
could open it using my `e` function in powershell. With this
done I just have to type `ok new "Title for a new blog
post"` and it will create a properly named file in the right
place with the header properly filled out and edit the file
automatically. Pretty clean!

Till tomorrow,  
Kay
