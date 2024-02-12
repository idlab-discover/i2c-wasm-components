#!/usr/bin/sh

cargo build --target wasm32-wasi
# It is important that the version is the same as the host wasmtime version
# https://github.com/bytecodealliance/wasmtime/tree/main/crates/wasi-preview1-component-adapter
wasm-tools component new ./target/wasm32-wasi/debug/guest.wasm \
    -o guest.component.wasm --adapt ./wasi_snapshot_preview1.reactor.wasm
