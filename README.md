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

With `oniri` started in "tiling layout" mode in your niri configuration file (`spawn-sh-at-startup "oniri --tiling-layout"` in `~/.config/niri/config.kdl`), the first window gets unmaximized when a second one is opened, mimicking the behavior of a tiling compositor:

<https://github.com/user-attachments/assets/d8ebfab0-3d88-44f9-9613-96b734b572ee>

See `oniri --help` and the [oniri(1) man page](https://raw.githubusercontent.com/Antiz96/oniri/refs/heads/main/doc/man/oniri.1.scd) for additional options and features.

**Note:** Due to current limitations of the niri IPC, "buggy" behaviors (e.g. window not being correctly maximized) *may* be expected in specific setups / edgy cases. Those limitations should *hopefully* be addressed on the niri IPC side at some point, allowing to fix those eventual "buggy" behaviors once and for all (see <https://github.com/Antiz96/oniri/issues/3> for more details).  
In the mean time, things *should* still work just fine for most "classic" setups though!

## Installation

### Packages

[![Packaging status](https://repology.org/badge/vertical-allrepos/oniri.svg)](https://repology.org/project/oniri/versions)

### Pre-compiled binary

A (statically linked) pre-compiled binary for the `x86_64 (amd64)` architecture is distributed as a [release asset](https://github.com/Antiz96/oniri/releases/latest) (`oniri-<release_version>-x86_64`).

The pre-compiled binary can be reproduced from source (in the sense of [reproducible builds](https://reproducible-builds.org)).  
The build environment is created and fully documented via [repro-env](https://github.com/kpcyrd/repro-env), and is tracked in this repository.

To reproduce the pre-compiled binary for a given release, [install repro-env](https://github.com/kpcyrd/repro-env#download) and run the following:

```bash
git clone https://github.com/Antiz96/oniri.git
cd oniri
git checkout <tag> # Where <tag> is the git tag for the targeted release, e.g. "v1.2.2"
repro-env build -- cargo build --release --target x86_64-unknown-linux-musl
sha256sum target/x86_64-unknown-linux-musl/release/oniri
```

Then, compare the `sha256` hash of the built binary to the one of the pre-compiled release binary (which is also recorded in the `oniri-<release_version>-x86_64.sha256` file in the release assets). Both hashes should be equal, indicating that the binary has been successfully reproduced.

Each release assets are also cryptographically signed, with the detached signature for each asset distributed as `<asset_name>.asc` (see the [MAINTAINERS.md file](https://github.com/Antiz96/oniri/blob/main/MAINTAINERS.md) for a list of keys expected to emit signatures).

### Build from source

```bash
git clone https://github.com/Antiz96/oniri.git
cd oniri
cargo build --release
```

The built binary will be located at `./target/release/oniri`.  

The [man page](https://github.com/Antiz96/oniri/tree/main/doc/man) can be generated with `scdoc`:

```bash
scdoc < doc/man/oniri.1.scd > doc/man/oniri.1
```

There are also shell completions available in the [`res/completions/`](https://github.com/Antiz96/oniri/tree/main/res/completions) directory.

## Usage

Add the following to your niri configuration file (`~/.config/niri/config.kdl`):

```text
spawn-at-startup "oniri"
```

See `oniri --help` or the [oniri(1) man page](https://raw.githubusercontent.com/Antiz96/oniri/refs/heads/main/doc/man/oniri.1.scd) for a list of options & arguments that can be passed.  
Note that passing options / arguments requires to use 'spawn**-sh-**at-startup' in the niri configuration file.

## Documentation

See `oniri --help` and the [oniri(1) man page](https://raw.githubusercontent.com/Antiz96/oniri/refs/heads/main/doc/man/oniri.1.scd).

## Contributing

See the [contributing guidelines](https://github.com/Antiz96/oniri/blob/main/CONTRIBUTING.md).

## License

Oniri is licensed under the [GPL-3.0 license](https://github.com/Antiz96/oniri/blob/main/LICENSE) (or any later version of that license).
