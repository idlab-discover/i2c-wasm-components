use crate::sensor::sketch::embedded::i2c;
use linux_embedded_hal::I2cdev;
use wasmtime::{
    component::{bindgen, Component, Linker},
    Engine, Result, Store,
};
use wasmtime_wasi::preview2::{ResourceTable, WasiCtx};

use super::{HostComponent, MyState};

bindgen!({
    path: "../wit",
    world: "sensor",
    with: {
        "sketch:embedded/delay/delay": Delay,
        "sketch:embedded/i2c/i2c": I2c,
    }
});
#[cfg(target_arch = "wasm32")]
pub struct I2c(I2cdev);

impl i2c::Host for HostComponent {}
impl i2c::HostI2c for HostComponent {
    fn transaction(
        &mut self,
        self_: wasmtime::component::Resource<I2c>,
        address: i2c::Address,
        operations: Vec<i2c::Operation>,
    ) -> wasmtime::Result<Result<Vec<Vec<u8>>, i2c::ErrorCode>> {
        todo!()
    }

    fn read(
        &mut self,
        self_: wasmtime::component::Resource<I2c>,
        address: i2c::Address,
        len: u64,
    ) -> wasmtime::Result<Result<Vec<u8>, i2c::ErrorCode>> {
        let self_ = self.table.get_mut(&self_)?;
        let mut data = vec![0; len.try_into().unwrap()];

        match embedded_hal::i2c::I2c::read(&mut self_.0, address, &mut data) {
            Ok(()) => Ok(Ok(data)),
            Err(_) => Ok(Err(i2c::ErrorCode::Other)),
        }
    }

    fn write(
        &mut self,
        self_: wasmtime::component::Resource<I2c>,
        address: i2c::Address,
        data: Vec<u8>,
    ) -> wasmtime::Result<Result<(), i2c::ErrorCode>> {
        let self_ = self.table.get_mut(&self_)?;

        match embedded_hal::i2c::I2c::write(&mut self_.0, address, &data) {
            Ok(()) => Ok(Ok(())),
            Err(_) => Ok(Err(i2c::ErrorCode::Other)),
        }
    }

    fn write_read(
        &mut self,
        self_: wasmtime::component::Resource<I2c>,
        address: i2c::Address,
        write: Vec<u8>,
        read_len: u64,
    ) -> wasmtime::Result<Result<Vec<u8>, i2c::ErrorCode>> {
        let self_ = self.table.get_mut(&self_)?;
        let mut data = vec![0; read_len.try_into().unwrap()];

        match embedded_hal::i2c::I2c::write_read(&mut self_.0, address, &write, &mut data) {
            Ok(()) => Ok(Ok(data)),
            Err(_) => Ok(Err(i2c::ErrorCode::Other)),
        }
    }

    fn drop(&mut self, self_: wasmtime::component::Resource<I2c>) -> wasmtime::Result<()> {
        self.table.delete(self_)?;
        Ok(())
    }
}

pub fn run(
    mut linker: Linker<MyState>,
    engine: Engine,
    component: Component,
    wasi: WasiCtx,
) -> Result<()> {
    // Binding host
    Sensor::add_to_linker(&mut linker, |state: &mut MyState| &mut state.host)?;

    let mut state = MyState {
        host: HostComponent {
            table: ResourceTable::new(),
        },
        wasi,
    };

    let i2cdev_1 = I2cdev::new(format!("/dev/i2c-{}", 1))?;
    let i2cdev_2 = I2cdev::new(format!("/dev/i2c-{}", 1))?;

    let connection_1 = state.host.table.push(I2c(i2cdev_1))?;
    let connection_2 = state.host.table.push(I2c(i2cdev_2))?;

    let mut store = Store::new(&engine, state);

    let (bindings, _) = Sensor::instantiate(&mut store, &component, &linker)?;

    let sensor = bindings.sketch_embedded_hts();
    let temperature = sensor.call_get_temperature(&mut store, connection_1)?;
    println!("{}", temperature?);

    let humidity = sensor.call_get_humidity(&mut store, connection_2)?;
    println!("{}", humidity?);

    Ok::<(), anyhow::Error>(())
}
