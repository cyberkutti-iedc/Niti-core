# Niti V1 Board - EAL (Embedded Abstraction Layer) 

**Niti V1** is an embedded abstraction layer for AVR microcontrollers and common boards like the Niti V1 board. It is based on the [`avr-device`](repository = "https://github.com/cyberkutti-iedc/Niti-core") crate and makes it easier to develop firmware for AVR-based microcontrollers.

## Quickstart

To develop for Niti V1, you will need a nightly Rust compiler, which will be automatically installed due to the `rust-toolchain.toml` file included in the project.

### Install Dependencies:

- **Ubuntu**:
  ```bash
  sudo apt install avr-libc gcc-avr pkg-config avrdude libudev-dev build-essential
  ```

- **MacOS**:
  ```bash
  xcode-select --install
  brew tap osx-cross/avr
  brew install avr-gcc avrdude
  ```

- **Windows**:
  Use `winget` on Windows 10 & 11:
  ```bash
  winget install AVRDudes.AVRDUDE ZakKemble.avr-gcc
  ```

  Alternatively, you can use **Scoop** on older systems:
  ```PowerShell
  Set-ExecutionPolicy RemoteSigned -Scope CurrentUser
  irm get.scoop.sh | iex
  scoop install avr-gcc avrdude
  ```

See [Setting up environment](repository = "https://github.com/cyberkutti-iedc/Niti-core"/avr-hal/wiki/Setting-up-environment) for more information.

### Installing Waterman:

To install **Waterman**, a tool that seamlessly integrates flashing your board into the usual cargo workflow, use:

```bash
cargo install waterman
```

### Building and Running:

Once you've installed everything, navigate to the directory for your board:

```bash
cd examples/niti-v1

# Build and run on a connected Niti V1 board
cargo run --bin niti-blink
```

## Starting Your Own Project:

To create your own project, use the [`niti-core-template`](repository = "https://github.com/cyberkutti-iedc/Niti-core") repository. Install **cargo-generate** and generate a new project:

```bash
cargo install cargo-generate
cargo generate --git repository = "https://github.com/cyberkutti-iedc/Niti-core".git
```

## Repository Structure:

The **Niti-core** repository is organized into different components making up the Embedded Abstraction Layer (EAL). Below is an overview:

### `niti-eal` ![niti-eal docs](https://img.shields.io/badge/docs-git-4d76ae)

**Niti-eal** is a complete abstraction layer for the Niti V1 board. It abstracts the hardware and provides easy-to-use interfaces for interacting with peripherals, sensors, and displays.

### `examples/*`

The [examples directory](./examples) contains practical usage examples for common peripherals and components.

### `mcu/atmega-hal` ![atmega-hal docs](https://img.shields.io/badge/docs-git-4d76ae)

This is the abstraction layer for the **ATmega** family of microcontrollers. It includes basic functionality and helper modules for working with microcontrollers like ATmega328P.

### `avr-hal-generic` ![avr-hal-generic docs](https://img.shields.io/badge/docs-git-4d76ae)

This is the generic HAL that implements most of the HAL functionality using macros. It is intended to be used by developers creating drivers that work across different AVR microcontrollers.

### `avr-specs/`

This directory contains Rust compiler target definitions for all supported microcontrollers. You will need these for compiling Rust code for AVR microcontrollers. The [`niti-core-template`](repository = "https://github.com/cyberkutti-iedc/Niti-core") already includes these for convenience.

## Disclaimer:

This project is not affiliated with Microchip (formerly Atmel) or any vendors associated with the supported boards.

## License:

This project is licensed under the **MIT License** ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT).

## Contribution:

Unless stated otherwise, any contributions made for inclusion in this project shall be dual-licensed as above, without additional terms or conditions.

---

### GitHub Repository:
Find the full project here: [Niti-core GitHub](repository = "https://github.com/cyberkutti-iedc/Niti-core")

---

### Badges:

![Continuous Integration](repository = "https://github.com/cyberkutti-iedc/Niti-core"/avr-hal/workflows/Continuous%20Integration/badge.svg)  
[![niti-eal docs](https://img.shields.io/badge/docs-git-4d76ae)][niti-eal docs]  
[![atmega-hal docs](https://img.shields.io/badge/docs-atmega--hal-4d76ae)][atmega-hal docs]  