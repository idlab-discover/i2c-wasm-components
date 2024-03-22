# i2c-wasm-components
The purpose of this repository is to serve as a proof of concept of a potential WASI and I2C integration. 

Currently, the setup is as follows: Raspberry Pi 4 Model B → I2C Interface → HD44780 LCD. It is my intention to switch out the Pi for a Pi Pico microcontroller, to have a proof of concept for a more constrained piece of hardware. I also have a Pi 3 Model B hooked up with a HTS221.

## Just

To make building easier [just](https://just.systems/man/en/) is used, see `Justfile`. 

## Host
To compile for Raspberry Pi make sure to have the corresponding target and linker installed.

```bash
rustup target add aarch64-unknown-linux-gnu
# Or the equivalent for your package manager
yay -S aarch64-linux-gnu-gcc
```

### Switching guest

1. Change the `included` guest in the wit.
2. Change the method invocations in the `run` function inside `device.rs`.
3. Change the used `wasm` file inside `main`.

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

### `app` world and how I tried to make a generic interface for the host

As I currently have multiple guest components, I would like to use whichever in the host via a CLI. Problem is that the [bindgen](https://docs.rs/wasmtime/latest/wasmtime/component/macro.bindgen.html) macro is quite restrictive, e.g. it's not possible to have one for each world or to define a wrapper around the guest components and then call that one (see [this commit](https://github.com/Zelzahn/i2c-wasm-components/pull/10/commits/5ea3c0f43e3e46022cf8d05a31e439431b359e2d)).

So I came to the `app` world, it includes one of the guest components. The benefit of this wrapper is that the world itself does not change, thus limiting the number of changes required to switch the linked guest in the host.

Another solution would be to define a host for each guest component, see [this commit](https://github.com/Zelzahn/i2c-wasm-components/commit/7b9648b57c24aad50015215e89f6b6db9342f19e), but this leads to loads of code duplication.

## Embedded HAL
The [embedded-hal](https://crates.io/crates/embedded-hal) crate is the main inspiration for the design of the API. But I currently have not found a way to package a crate that uses this API into a WASM module.
