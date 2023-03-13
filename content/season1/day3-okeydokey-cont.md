+++
title = "Day3 - Okeydokey Cont."
description = "Rust Command Line Tools"
date = 2019-02-09

[extra]
project = "okeydokey"
+++
Continuing on from yesterday, I built a feature equivalent version of my lost
directory profile tool in rust. What follows were the steps and tools I used in
the process.

## Previous Version Structure

The decompiled
[source](https://gist.github.com/Kethku/ee982b01ef4ef022da3522b28e3997ad) we got
from yesterday gave us the basic structure of the original version. The tool has
two main executable paths. Either it is run without any arguments, or it is
passed the prefix of a command to run. 

In the first case, it searches for the `.ok` profile and lists the command names
to the console.

{% code() %}
> ScriptProfileManager
use
build
clean
{% end %}

In the second case it finds the longest match in the profile commands for the
passed in prefix and prints the associated command to the console.

{% code() %}
> ScriptProfileManager b
cargo build
{% end %}

The original version of the tool assumed that the output list of commands would
be post-processed by some form of pretty printer. I think this puts too much
burden on the script side of things, so my new version will print the command
list in one line.

Similarly the old version assumed that the script environment contained a pushd
and popd command. In the new version I am relaxing that requirement by adding
prefix and suffix arguments. They take a string which can optionally have a `{}`
hole which will be filled with the directory path to the `.ok` file. This lets
the user of okeydokey specify how they would like to wrap the command.

## Clap

I decided to use the popular [Clap](https://github.com/clap-rs/clap) crate for
command line argument parsing. The library allows to declaritively describe the
command line interface and then query the parsed arguments in a simple way. Clap
gives you many options for ways to describe the interface, but he one I picked
was a yaml file which gets parsed at compile time. This lets me separate the
interface out of the rust code and move on with the actual command logic.

The yaml file was pretty simple, but I did have to do some guesswork in order to
get everything right. For example it wasn't clear what `value_name` did.
Eventually I landed on this structure:

{% code(lang="yaml") %}
name: Okeydokey
version: "0.1"
author: Kaylee Simmons
about: .ok file manager
args:
  - COMMAND:
      help: The command in the profile to run
  - prefix:
      short: p
      long: prefix
      value_name: PREFIX
      help: Prepends argument to the returned command replacing {} with the full path to the found .ok file.
  - suffix:
      short: s
      long: suffix
      value_name: SUFFIX
      help: Appends argument to the returned command replacing {} with the full path to the found .ok file.
{% end %}

Then in the main function I was able to build up the matches and query them like so:

{% code(lang="rust") %}
let yaml = load_yaml!("cli.yml");
let matches = App::from_yaml(yaml).get_matches();

println!("{}", matches.value_of("COMMAND"));
{% end %}

## Walking Up the Directory Tree

Both of the execution paths of the tool require a parsed `.ok` file. To find it
the tool needs to walk up the directory tree searching for a parent directory
which contains the `.ok` file. Rust has safe and complete file system apis, but
as with most things in the Rust standard library, it does some gymnastics to
make sure everything is above board with regards to memory safety. Similar to
the relationship between `&str` and `String`, Rust has `Path` and `PathBuf`
where `Path` is an immutable filesystem path and `PathBuf` is an owned mutable
variant. The confusing part is that `PathBuf` implements the trait `Deref` to
`Path` which as far as I can understand it means that the compiler is allowed to
dereference the `PathBuf` implicitly. So any place a function can take a `Path`
you can also pass in a `PathBuf` and things *should* work out. For example,
although `PathBuf` does not contain a `parent` function directly, you can still
call `parent` on it since it gets dereferenced into a `Path` which does.

On top of being immutable, `str` and `Path` are *unsized* types meaning that you
can't store variables to them without a pointer or similar without the compiler
yelling at you. Given my C# background, I'm still a little fuzzy about the
idiomatic way to use these types, but in practice I have found that using the
PathBuf and String versions of the types is maybe not the most efficient method,
but gets the job done, allows you the most flexibility, and keeps our friend the
compiler happy. 

Frustratingly, there is a fair amount of syntactic overhead for ensuring that
you are using the correct type. The previously mentioned `parent` function does
not return a `PathBuf` but instead the more frustrating `Path`, so whenever I
use the function, I needed to call `to_path_buf` just adding to the visual
noise. I get that it is for my own good, but its an example of the Rust making
easy things harder.

After stumbling through understanding the above, the actual task of finding the
`.ok` file and parsing it into command name, command pairs was pretty trivial.

{% code(lang="rust") %}
fn find_profile(current_path: PathBuf) -> Option<Profile> {
    let possible_profile = current_path.join(".ok");
    if possible_profile.exists() {
        Some(read_profile(possible_profile)?)
    } else {
        Some(find_profile(current_path.parent()?.to_path_buf())?)
    }
}

fn read_profile(profile_path: PathBuf) -> Option<Profile> {
    match File::open(profile_path.clone()) {
        Ok(ref mut file) => {
            let mut commands = HashMap::new();

            for line in BufReader::new(file).lines() {
                let (name, command) = split_on_colon(line.unwrap())?;
                commands.insert(name, command);
            }

            Some(Profile { commands, path: profile_path })
        },
        Err(_) => None
    }
}

fn split_on_colon(line: String) -> Option<(String, String)> {
    let mut splitter = line.splitn(2, ':');
    let name = splitter.next()?;
    let command = splitter.next()?;
    Some((name.to_string(), command.to_string()))
}
{% end %}

## String Manipulation

The argumentless version of okeydokey which prints the commands to the console
was deceptively difficult to get right. The problem stems from the fact that I
store the commands in a `HashMap<String, String>` where the key is the command
name and the value is the command. The naive solution would be to pull the keys
out into a collection, and use the `String` utilities to join them into a single
`String`. In practice though, we run into ownership problems. `HashMap.keys`
returns an iterator of `&String`, not `String`. This prevents us from using
`join` which must operate on values not references. Eventually I landed
on a call to `fold` passing in `String` accumulator but it wasn't my first choice.

The query execution path went relatively smoothly after the above. The `query` function finds the best match in the commands list using `filter_map` and `max_by_key` and then passes the found command as well as the passed in prefix and suffix options on to get formatted into the final output.

{% code(lang="rust") %}
fn list(profile: Profile) {
    let list = profile.commands
        .keys()
        .fold(String::new(), |acc, next| {
            acc + " " + next
        });
    println!("{}", list.trim());
}

fn query(profile: Profile, command: &str, prefix: Option<&str>, suffix: Option<&str>) {
    let best_option = profile
        .commands
        .keys()
        .filter_map(|possible_command| shared_prefix(possible_command, command))
        .max_by_key(|&(shared_chars, _)| shared_chars);

    match best_option {
        Some((_, actual_command)) => print_decorated_command(profile, actual_command, prefix, suffix),
        None => ()
    }
}

fn shared_prefix(possible_command: &str, command: &str) -> Option<(usize, String)> {
    match possible_command.starts_with(command) {
        true => Some((command.len(), possible_command.to_string())),
        false => None
    }
}

fn print_decorated_command(profile: Profile, command_name: String, prefix: Option<&str>, suffix: Option<&str>) {
    let prefix = fill_in_profile_directory(&profile, prefix);
    let suffix = fill_in_profile_directory(&profile, suffix);
    let command = profile.commands.get(&command_name).unwrap();

    println!("{}", vec![prefix, command.to_string(), suffix].concat())
}

fn fill_in_profile_directory(profile: &Profile, pattern: Option<&str>) -> String {
    let profile_directory = profile.path.parent().unwrap().to_str().unwrap();
    pattern.unwrap_or_default().replace("{}", profile_directory)
}
{% end %}

## The Script Wrapper

At this stage, the tool is feature complete for the first version, but it
requires a couple of modifications to my script profile to work properly. I have
a function defined in my PowerShell profile called `ok` which gets called in my
prompt function so that ever time I change directories the command list is
printed if it exists. If the function is given any arguments, they are fed into
the profile utility and the output is executed directly.

Previously, I did special formatting of the command list in PowerShell, which
has since been pushed to the tool side. Similarly the command execution assumed
that the directory management was handled in the tool, but I have pushed that to
the command arguments instead to be more flexible. So the execution of okeydokey
needed to be modified as well. I landed on this:

{% code(lang="powershell") %}
function ok
{
  Param($command = $null)
  if ($command -eq $null) {
    $fore = $host.UI.RawUI.ForegroundColor
    $host.UI.RawUI.ForegroundColor = 'Blue'
    okeydokey
    $host.UI.RawUI.ForegroundColor = $fore
  } else {
    $script = okeydokey $command -p "pushd {};" -s "; popd"
    if ($script -ne $null) {
      iex $script
    }
  }
}
{% end %}

Simple and straight forward.

## Summary

I have pushed the current version of the tool to
[github](https://github.com/Kethku/okeydokey) so anyone can use it if they like.
Depending on how much I've got left in me (this took many hours again... I need
to dial this back if I want to do it every day), I may build a simple home page
for the tool describing its usage shortly. I've used a version of this tool
pretty much daily for the past 6 months, and I have some ideas for how to make
it even better. Among them is argument support, profile references, and a better
UI. For now though, I will probably do some site styling and call it a day.

Till tomorrow,  
Kaylee
