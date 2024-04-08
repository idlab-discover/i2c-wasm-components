# i2c-wasm-components
The purpose of this repository is to serve as a proof of concept of a potential WASI and I2C integration. 

Currently, the setup is as follows: Raspberry Pi 4 Model B → I2C Interface → HD44780 LCD. It is my intention to switch out the Pi for a Pi Pico microcontroller, to have a proof of concept for a more constrained piece of hardware. I also have a Pi 3 Model B hooked up with a HTS221.

## Embedded HAL

The [embedded-hal](https://crates.io/crates/embedded-hal) crate is the main inspiration for the design of the API. To embed a crate that uses this HAL as a guest component [wasi-embedded-hal](https://github.com/Zelzahn/wasi-embedded-hal) can be used.

## Benchmarking

### Execution Time

For this [criterion.rs](https://github.com/bheisler/criterion.rs) is used. Sadly, it isn't possible to use this in conjunction with a binary crate, thus the crate is split up into a library and a binary.

Benchmarks can be found in the `benches/` directory inside the hosts and native implementations.

#### Building

The executable is built via `cargo bench --no-run`. Afterwards it can be `scp`'ed and run using `./sensor_read-a3122c62d6d7e506 --bench`. The results are stored inside `target/criterion/sensor\\\ read/report` and are then `scp`'ed back into `report/`.

### Memory Usage

The used memory is collected via [dhat-rs](https://github.com/nnethercote/dhat-rs). This gives output of the following format after execution:
```
dhat: Total:     xxx bytes in y blocks
dhat: At t-gmax: xxx bytes in y blocks
dhat: At t-end:  xxx bytes in y blocks
```

To build run `cargo build --target aarch64-unknown-linux-gnu --release --features dhat-heap`.