## Niti V1 `avr-hal` Examples

The subdirectories here contain various examples demonstrating how to write firmware using **`avr-hal`** for the Niti V1 board. Note that many examples designed for different boards can often be easily ported to other hardware. If you can't find something specifically for your board, check examples for other boards as well.

All examples are ready to use if you have the respective board available. Just switch to the appropriate subdirectory and run an example via Cargo. For example:

```bash
cd examples/niti-v1

# Build and run it on a connected Niti V1 board
cargo run --bin niti-blink
```

You need to install **Waterman** by running `cargo install waterman` to enable seamless flashing of your board.
