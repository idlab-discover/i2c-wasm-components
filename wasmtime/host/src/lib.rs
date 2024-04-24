mod device;
mod sensor;
mod display;

use device::Device;
use wasmtime::{
    component::{Component, Linker},
    Config, Engine, Result,
};
use wasmtime_wasi::WasiCtxBuilder;

pub enum Guest {
   Sensor,
   LCDDisplay,
   SegmentLEDDisplay
}

pub fn execute(guest: Guest) -> Result<String, anyhow::Error> {
    let engine = Engine::new(Config::new().wasm_component_model(true))?;

    let mut linker = Linker::new(&engine);

    // Bind wasi commmand world
    wasmtime_wasi::command::sync::add_to_linker(&mut linker)?;

    let wasi = WasiCtxBuilder::new().inherit_stdout().build();

    let component = match guest {
        Guest::Sensor => Component::from_file(&engine, "hat.wasm")?,
        Guest::LCDDisplay => Component::from_file(&engine, "lcd.wasm")?,
        Guest::SegmentLEDDisplay => Component::from_file(&engine, "led.wasm")?
    };
    
    match guest {
        Guest::Sensor => sensor::DeviceState::new(linker, engine, component, wasi)?.run(),
        Guest::LCDDisplay | Guest::SegmentLEDDisplay => display::DeviceState::new(linker, engine, component, wasi)?.run(),
    }
}
