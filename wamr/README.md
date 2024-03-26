# WAMR

`guest/` handles the showing of a message to the segment LED. The `write` function accepts 4 `i32`'s, one for each led.

In `host/` a WAMR implementation is provided. This uses the [wamr-rust-sdk](https://github.com/bytecodealliance/wamr-rust-sdk). To circumvent the need of dealing with passing an I2C connection, the connection is kept global inside the host.
