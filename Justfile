build: build-hat build-command
    just link-hat

build-hat:
    cd guest/hat && cargo +nightly component build --release

build-lcd:
    cd guest/lcd && cargo +nightly component build --release

build-command:
    cd command && cargo  +nightly component build --release

link-hat:
    wasm-tools compose command/target/wasm32-wasi/release/command.wasm -d guest/target/wasm32-wasi/release/hat.wasm -o composed.wasm