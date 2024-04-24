use crate::device;
use crate::device::wasi::i2c::i2c;
use crate::device::wasi::i2c::delay;
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
    world: "screen",
    with: {
        "wasi:i2c/i2c/i2c": device::I2c,
        "wasi:i2c/delay/delay": device::Delay,
    }
});

pub struct DeviceState {
    connection: u32,
    delay: u32,
    bindings: Screen,
    store: Store<HostState>,
}

impl device::Device for DeviceState {
    fn new(
        mut linker: Linker<HostState>,
        engine: Engine,
        component: Component,
        wasi: WasiCtx,
    ) -> Result<Self, anyhow::Error> {
        i2c::add_to_linker(&mut linker, |state: &mut HostState| &mut state.host)?;
        delay::add_to_linker(&mut linker, |state: &mut HostState| &mut state.host)?;

        let mut state = HostState {
            host: HostComponent {
                table: ResourceTable::new(),
            },
            wasi,
        };

        let i2cdev = I2cdev::new("/dev/i2c-1")?;

        let connection = state.host.table.push(I2c(i2cdev))?;
        let delay = state.host.table.push(device::Delay)?;
        
        let mut store = Store::new(&engine, state);
        let (bindings, _) = Screen::instantiate(&mut store, &component, &linker)?;

        Ok(DeviceState {
            connection: connection.rep(),
            delay: delay.rep(),
            store,
            bindings,
        })
    }

    fn run(&mut self) -> Result<String, anyhow::Error> {
        let connection = Resource::new_own(self.connection);
        let delay = Resource::new_own(self.delay);

        let message = "1234";

        self
            .bindings
            .sketch_implementation_lcd()
            .call_write(&mut self.store, connection, delay, message)?;

        Ok(message.to_string())
    }
}
