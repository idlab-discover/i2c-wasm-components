# i2c-wasm-components
The purpose of this repository is to serve as a proof of concept of a potential WASI and I2C integration. 

Currently, the setup is as follows: Raspberry Pi 4 Model B → I2C Interface → HD44780 LCD. It is my intention to switch out the Pi for a Pi Pico microcontroller, to have a proof of concept for a more constrained piece of hardware. I also have a Pi 3 Model B hooked up with a HTS221.

## Host
To compile for Raspberry Pi make sure to have the corresponding target and linker installed.

```bash
rustup target add aarch64-unknown-linux-gnu
# Or the equivalent for your package manager
yay -S aarch64-linux-gnu-gcc
```

## Guest
### Screen
This guest component is written in Rust and uses the received I²C connection to display `hello world` to the HD44780 LCD screen.

This code is heavily influenced by the Rust crate [hd44780-driver](https://crates.io/crates/hd44780-driver).

### Sensor
This guest component is also written in Rust and uses the connection to read the current temperature from a hts221.

Inspiration is taken from [hts221](https://crates.io/crates/hts221). Actually this library takes a connection as an argument, thus the library itself could be used instead of the current cherrypicked parts (which I do in the native version). But I have not found a way tell the guest that my host will implement the `embedded_hal` traits. 

### `no_std`
To allow us to use the feature and thus use no std currently the nightly version of Rust is needed:
```bash
rustup install nightly 
# The build commando is now slightly modified
cargo +nightly component build
```

## WIT
`embedded.wit` comes from [hello-embedded](https://github.com/sunfishcode/hello-embedded) by sunfishcode. Only the `i2c` and `delay` interfaces are used from this.

I had to use the same package for my `screen.wit` to make the `bindgen` in the host work, more specifically the `with`.

## Embedded HAL
The [embedded-hal](https://crates.io/crates/embedded-hal) crate is the main inspiration for the design of the API. But I currently have not found a way to package a crate that uses this API into a WASM module.
