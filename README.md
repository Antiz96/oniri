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

### AUR

Arch Linux users can install the [oniri](https://aur.archlinux.org/packages/oniri) AUR package.

### From Source

```bash
git clone https://github.com/Antiz96/oniri.git
cd oniri
cargo build --release
```

The built binary will be located at `./target/release/oniri`, copy it somewhere in your `$PATH`.

## Usage

Add the following to your niri configuration file (`~/.config/niri/config.kdl`):

```text
spawn-at-startup "oniri"
```

## Contributing

See the [contributing guidelines](https://github.com/Antiz96/oniri/blob/main/CONTRIBUTING.md).

## License

Oniri is licensed under the [GPL-3.0 license](https://github.com/Antiz96/oniri/blob/main/LICENSE) (or any later version of that license).
