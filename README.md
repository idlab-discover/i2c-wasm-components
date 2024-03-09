# i2c-wasm-components
The purpose of this repository is to serve as a proof of concept of a potential WASI and I2C integration. 

Currently, the setup is as follows: Raspberry Pi 4 Model B → I2C Interface → HD44780 LCD. It is my intention to switch out the Pi for a Pi Pico microcontroller, to have a proof of concept for a more constrained piece of hardware.

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

Inspiration is taken from [hts221](https://crates.io/crates/hts221). Actually this library takes a connection as an argument, thus the library itself could be used instead of the current cherrypicked parts. But this is not done because it still uses the old `embedded_hal` 0.2 and not the current 1.0.

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
