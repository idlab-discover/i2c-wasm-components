bindgen!({
    path: "../wit",
    world: "app",
    with: {
        "wasi:i2c/delay/delay": Delay,
        "wasi:i2c/i2c/i2c": I2c,
    }
});

use linux_embedded_hal::I2cdev;
use wasi::i2c::*;
use wasmtime::{component::bindgen, Result};
use wasmtime::{
    component::{Component, Linker},
    Engine, Store,
};
use wasmtime_wasi::WasiView;
use wasmtime_wasi::{ResourceTable, WasiCtx};

struct HostComponent {
    table: ResourceTable,
}

pub struct MyState {
    host: HostComponent,
    wasi: WasiCtx,
}

// Needed for wasmtime_wasi::preview2
impl WasiView for MyState {
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.host.table
    }
    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.wasi
    }
}

pub struct Delay;
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

impl delay::Host for HostComponent {}
impl delay::HostDelay for HostComponent {
    fn delay_ns(
        &mut self,
        self_: wasmtime::component::Resource<delay::Delay>,
        ns: u32,
    ) -> wasmtime::Result<()> {
        let _self_ = self.table.get_mut(&self_)?;
        std::thread::sleep(std::time::Duration::from_nanos(ns.into()));
        Ok(())
    }

    fn drop(&mut self, self_: wasmtime::component::Resource<delay::Delay>) -> wasmtime::Result<()> {
        self.table.delete(self_)?;
        Ok(())
    }
}

pub fn run(
    mut linker: Linker<MyState>,
    engine: Engine,
    component: Component,
    wasi: WasiCtx,
) -> Result<String, anyhow::Error> {
    // Binding host
    wasi::i2c::i2c::add_to_linker(&mut linker, |state: &mut MyState| &mut state.host)?;

    let mut state = MyState {
        host: HostComponent {
            table: ResourceTable::new(),
        },
        wasi,
    };

    let i2cdev = I2cdev::new(format!("/dev/i2c-{}", 1))?;

    let connection = state.host.table.push(I2c(i2cdev))?;
    let delay = state.host.table.push(Delay)?;

    let mut store = Store::new(&engine, state);

    let (bindings, _) = App::instantiate(&mut store, &component, &linker)?;

    Ok(bindings
        .interface0
        .call_get_temperature(&mut store, connection)??)
}
