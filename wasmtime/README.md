# Wasmtime implementation

## Just

To make building easier [just](https://just.systems/man/en/) is used, see `Justfile`. 

## Host
To compile for Raspberry Pi, make sure to have the corresponding target and linker installed.

```bash
rustup target add aarch64-unknown-linux-gnu
# Or the equivalent for your package manager
yay -S aarch64-linux-gnu-gcc
```

### Switching guest

1. Change the method invocations in the `run` function inside `device.rs`.
2. Change the used `wasm` file inside `main`.

## Guest
### Screen
This guest component is written in Rust and uses the received I²C connection to display `hello world` to the HD44780 LCD screen.

This code is heavily influenced by the Rust crate [hd44780-driver](https://crates.io/crates/hd44780-driver).

### Sensor
This guest component is also written in Rust and uses the connection to read the current temperature from a hts221.

The [hts221](https://crates.io/crates/hts221) crate is used, to be precise a fork I made that adds `embedded-hal` version 1 support, in tandem with [wasi-embedded-hal](https://crates.io/crates/wasi-embedded-hal).

### `no_std`
To allow us to use the feature and thus use no std currently the nightly version of Rust is needed:
```bash
rustup install nightly 
# The build commando is now slightly modified
cargo +nightly component build
```

## WIT
See [wasi-i2c](https://github.com/WebAssembly/wasi-i2c) for the source of the wit files.

