use crate::device;
use crate::device::wasi::i2c::i2c::add_to_linker;
use crate::device::{HostComponent, HostState, I2c};

use linux_embedded_hal::I2cdev;
use wasmtime::{
    component::{bindgen, Resource},
    Result,
};
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

pub struct DeviceState {
    connection: u32,
    bindings: Sensor,
    store: Store<HostState>,
}

impl device::Device for DeviceState {
    fn new(
        mut linker: Linker<HostState>,
        engine: Engine,
        component: Component,
        wasi: WasiCtx,
    ) -> Result<Self, anyhow::Error> {
        add_to_linker(&mut linker, |state: &mut HostState| &mut state.host)?;

        let mut state = HostState {
            host: HostComponent {
                table: ResourceTable::new(),
            },
            wasi,
        };

        let i2cdev = I2cdev::new("/dev/i2c-1")?;

        let connection = state.host.table.push(I2c(i2cdev))?;
        let mut store = Store::new(&engine, state);
        let (bindings, _) = Sensor::instantiate(&mut store, &component, &linker)?;

        Ok(DeviceState {
            connection: connection.rep(),
            store,
            bindings,
        })
    }

    fn run(&mut self) -> Result<String, anyhow::Error> {
        let connection = Resource::new_borrow(self.connection);

        Ok(self
            .bindings
            .interface0
            .call_get_temperature(&mut self.store, connection)??)
    }
}
