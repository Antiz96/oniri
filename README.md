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

## Usage

Add the following to your niri configuration file (`~/.config/niri/config.kdl`):

```text
spawn-at-startup "oniri"
```

See `oniri --help` or the [oniri(1) man page](https://github.com/Antiz96/oniri/blob/main/doc/man/oniri.1.scd) for a list of options & arguments that can be passed.

## Documentation

See `oniri --help` and the [oniri(1) man page](https://github.com/Antiz96/oniri/blob/main/doc/man/oniri.1.scd).

## Contributing

See the [contributing guidelines](https://github.com/Antiz96/oniri/blob/main/CONTRIBUTING.md).

## License

Oniri is licensed under the [GPL-3.0 license](https://github.com/Antiz96/oniri/blob/main/LICENSE) (or any later version of that license).
