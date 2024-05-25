mod device;
mod display;
mod sensor;

use device::Device;
use std::fs;
use wasmtime::{
    component::{Component, Linker},
    Config, Engine, Result,
};
use wasmtime_wasi::WasiCtxBuilder;

pub enum Guest {
    Sensor,
    LCDDisplay,
    SegmentLEDDisplay,
}

pub fn execute(guest: Guest, option: Option<&str>) -> Result<String, anyhow::Error> {
    let engine = Engine::new(Config::new().wasm_component_model(true))?;

    let mut linker = Linker::new(&engine);

    // Bind wasi commmand world
    wasmtime_wasi::command::sync::add_to_linker(&mut linker)?;

    let wasi = WasiCtxBuilder::new().inherit_stdout().build();

    let component = get_component(&engine, &guest, option)?;

    match guest {
        Guest::Sensor => sensor::DeviceState::new(linker, engine, component, wasi)?.run(),
        Guest::LCDDisplay | Guest::SegmentLEDDisplay => {
            display::DeviceState::new(linker, engine, component, wasi)?.run()
        }
    }
}

fn get_component(engine: &Engine, guest: &Guest, option: Option<&str>) -> Result<Component> {
    if matches!(option, Some("serialize")) || option.is_none() {
        let component = match guest {
            Guest::Sensor => Component::from_file(engine, "hat.wasm")?,
            Guest::LCDDisplay => Component::from_file(engine, "lcd.wasm")?,
            Guest::SegmentLEDDisplay => Component::from_file(engine, "led.wasm")?,
        };

        if matches!(option, Some("serialize")) {
            let data = component.serialize()?;
            fs::write("./component.serialized", data)?;
        }

        Ok(component)
    } else {
        unsafe { Component::deserialize_file(engine, "./component.serialized") }
    }
}
