# Holdable Caps Lock

## Contents:

### [Arguments](#arguments)

### [Installation](#installation)

## Arguments:

- '-' and '_' are ignored
(`display-off` is the same as `displayoff`)
- `displayoff` or `nodisplay`
  - Doesn't display the `"toggled <caps lock state> and <caps lock key hold state>"` message
- `t` or `time`
  - Next argument is the time that it will take to turn off Caps Lock in milliseconds (default is 300 ms)
- `notoggle` or `onlyhold`
  - Makes Caps Lock only holdable (not toggleable)
- `waittime` or `wait`
  - Makes it wait after each iteration (runs faster but less accurate)

## Installation

### Windows

1. Go to [this](https://github.com/tadaHrd/holdable-capslock/releases/tag/2.0) page and download the executable
2. Click on it and it will open a terminal window which **You can't close!**

### Other systems

1. Download this repo
2. Unzip it
3. Install [rustup](https://rustup.rs/) if you don't already have it installed
4. Open your terminal in the unzipped folder
5. Run `cargo run` or if you want to use rustc ([what is it](https://doc.rust-lang.org/rustc/index.html)) run `rustc src\main.rs` then run `./main` or `main` depending on your platform
6. All done so you can just remove all other files and keep the output `main` file and `main.pdb` file if there is one
