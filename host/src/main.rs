mod device;
use device::screen;
use device::sensor;

use wasmtime::{
    component::{Component, Linker},
    Config, Engine, Result,
};
use wasmtime_wasi::preview2::WasiCtxBuilder;

fn main() -> Result<(), anyhow::Error> {
    let engine = Engine::new(Config::new().wasm_component_model(true))?;

    let mut linker = Linker::new(&engine);

    // Bind wasi commmand world
    wasmtime_wasi::preview2::command::sync::add_to_linker(&mut linker)?;

    let wasi = WasiCtxBuilder::new()
        .inherit_stdout()
        .inherit_stdio()
        .build();

    let run_sensor = true;
    let component = Component::from_file(
        &engine,
        if run_sensor {
            "../hat.wasm"
        } else {
            "../lcd.wasm"
        },
    )?;

    // TODO: Choose which one to run via a commandline argument
    if run_sensor {
        sensor::run(linker, engine, component, wasi)?;
    } else {
        screen::run(linker, engine, component, wasi)?;
    }

    Ok(())
}
