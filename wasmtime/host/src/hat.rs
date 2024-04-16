use crate::device::wasi::i2c::i2c::add_to_linker;
use crate::device::{HostComponent, I2c, MyState};
use crate::device;

use linux_embedded_hal::I2cdev;
use wasmtime::{component::bindgen, Result};
use wasmtime::{
    component::{Component, Linker},
    Engine, Store,
};
use wasmtime_wasi::{ResourceTable, WasiCtx};

bindgen!({
    path: "../wit",
    world: "sensor",
    with: {
        "wasi:i2c/i2c/i2c": device::I2c,
    }
});

pub fn run(
    mut linker: Linker<MyState>,
    engine: Engine,
    component: Component,
    wasi: WasiCtx,
) -> Result<String, anyhow::Error> {
    // Binding host
    add_to_linker(&mut linker, |state: &mut MyState| &mut state.host)?;

    let mut state = MyState {
        host: HostComponent {
            table: ResourceTable::new(),
        },
        wasi,
    };

    let i2cdev = I2cdev::new("/dev/i2c-1")?;

    let connection = state.host.table.push(I2c(i2cdev))?;

    let mut store = Store::new(&engine, state);

    let (bindings, _) = Sensor::instantiate(&mut store, &component, &linker)?;

    Ok(bindings
        .interface0
        .call_get_temperature(&mut store, connection)??)
}
