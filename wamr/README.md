# WAMR

`guest/` handles the showing of a message to the segment LED. 

In `host/` a WAMR implementation is provided. This uses the [wamr-rust-sdk](https://github.com/bytecodealliance/wamr-rust-sdk). To circumvent the need of dealing with passing an I2C connection, the connection is kept global inside the host.

Currently, compilation for the Pi fails because of [wamr-rust-sdk/#14](https://github.com/bytecodealliance/wamr-rust-sdk/issues/14).
