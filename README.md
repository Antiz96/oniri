# Oniri

## Table of contents

- [Description](#description)
- [Installation](#installation)
- [Usage](#usage)
- [Contributing](#contributing)
- [License](#license)

## Description

Oniri is a tool that automatically maximizes the **on**ly window of a **niri** workspace (whether it's the first or the last remaining one).

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

## Usage

Add the following to your niri configuration file (`~/.config/niri/config.kdl`):

```text
spawn-at-startup "oniri"
```

## Contributing

See the [contributing guidelines](https://github.com/Antiz96/oniri/blob/main/CONTRIBUTING.md).

## License

Oniri is licensed under the [GPL-3.0 license](https://github.com/Antiz96/oniri/blob/main/LICENSE) (or any later version of that license).
