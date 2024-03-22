default: hat host

hat: build-hat cp-hat
lcd: build-lcd cp-lcd
host: build-host

build-hat:
    cd guest/hat && cargo +nightly component build --release

cp-hat:
    cp guest/target/wasm32-wasi/release/hat.wasm hat.wasm

build-lcd:
    cd guest/lcd && cargo +nightly component build --release

cp-lcd:
    cp guest/target/wasm32-wasi/release/lcd.wasm lcd.wasm

build-host:
    cd host && cargo build --release --target aarch64-unknown-linux-gnu
