mod device;

use wasmtime::{
    component::{Component, Linker},
    Config, Engine, Result,
};
use wasmtime_wasi::WasiCtxBuilder;

fn main() -> Result<(), anyhow::Error> {
    let engine = Engine::new(Config::new().wasm_component_model(true))?;

    let mut linker = Linker::new(&engine);

    // Bind wasi commmand world
    wasmtime_wasi::command::sync::add_to_linker(&mut linker)?;

    let wasi = WasiCtxBuilder::new()
        .inherit_stdout()
        .inherit_stdio()
        .build();

    let component = Component::from_file(&engine, "composed.wasm")?;

    let res = device::run(linker, engine, component, wasi, "hat")?;

    println!("{:?}", res);

    Ok(())
}
