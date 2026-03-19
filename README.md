# Oniri

## Table of contents

- [Description](#description)
- [Installation](#installation)
- [Usage](#usage)
- [Documentation](#documentation)
- [Contributing](#contributing)
- [License](#license)

## Description

Oniri is a tool that automatically maximizes the **on**ly window of a **niri** workspace.

With `oniri` started in your niri configuration file (`spawn-at-startup "oniri"` in `~/.config/niri/config.kdl`), the only window of a niri workspace gets automatically maximized, whether it's the first opened window or the last remaining one after all the other windows got closed:

<https://github.com/user-attachments/assets/d5eab7a2-d0c2-4fc9-b1be-1d4bc1cb8fd8>

With `oniri` started in "first window only" mode in your niri configuration file (`spawn-sh-at-startup "oniri --first-only"` in `~/.config/niri/config.kdl`), only the first opened window of a niri workspace gets automatically maximized (the last remaining window after all the other windows got closed is **not** automatically maximized):

<https://github.com/user-attachments/assets/d97f1416-a5f0-452c-b2d4-16b6af12631f>

**Note:** Due to current limitations of the niri IPC, "buggy" behaviors (e.g. window not being correctly maximized) *may* be expected in specific setups / edgy cases. Those limitations should *hopefully* be addressed on the niri IPC side at some point, allowing to fix those eventual "buggy" behaviors once and for all (see <https://github.com/Antiz96/oniri/issues/3> for more details).  
In the mean time, things *should* still work just fine for most "classic" setups though!

## Installation

### Packages

[![Packaging status](https://repology.org/badge/vertical-allrepos/oniri.svg)](https://repology.org/project/oniri/versions)

### Pre-compiled binary

A pre-compiled binary for the `x86_64 (amd64)` architecture is distributed as a [release asset](https://github.com/Antiz96/oniri/releases/latest) (`oniri-<version>-amd64`).  
You can download it, make it executable, rename it `oniri` and copy it somewhere in your `$PATH`.

### Build from source

Requires `rustup` and `gcc`.

```bash
git clone https://github.com/Antiz96/oniri.git
cd oniri
cargo build --release
```

The built binary will be located at `./target/release/oniri`.  
You can copy it somewhere in your `$PATH`.

The man page can be generated with `scdoc`:

```bash
scdoc < doc/man/oniri.1.scd > doc/man/oniri.1
```

There are also shell completions available in the `res/completions/` directory.

## Usage

Add the following to your niri configuration file (`~/.config/niri/config.kdl`):

```text
spawn-at-startup "oniri"
```

See `oniri --help` or the [oniri(1) man page](https://github.com/Antiz96/oniri/blob/main/doc/man/oniri.1.scd) for a list of options & arguments that can be passed.  
Note that passing options / arguments requires to use 'spawn**-sh-**at-startup' in the niri configuration file.

## Documentation

See `oniri --help` and the [oniri(1) man page](https://github.com/Antiz96/oniri/blob/main/doc/man/oniri.1.scd).

## Contributing

See the [contributing guidelines](https://github.com/Antiz96/oniri/blob/main/CONTRIBUTING.md).

## License

Oniri is licensed under the [GPL-3.0 license](https://github.com/Antiz96/oniri/blob/main/LICENSE) (or any later version of that license).
