use crate::sketch::embedded::{delay, i2c};
use linux_embedded_hal::I2cdev;
use wasmtime::{
    component::{bindgen, Component, Linker},
    Config, Engine, Result, Store,
};
use wasmtime_wasi::preview2::{ResourceTable, WasiCtx, WasiCtxBuilder, WasiView};

bindgen!({
    path: "../wit",
    world: "screen",
    with: {
        "sketch:embedded/delay/delay": Delay,
        "sketch:embedded/i2c/i2c": I2c,
    }
});

pub struct Delay;
pub struct I2c(I2cdev);

struct HostComponent {
    table: ResourceTable,
}

impl i2c::Host for HostComponent {}
impl delay::Host for HostComponent {}

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

struct MyState {
    host: HostComponent,
    wasi: WasiCtx,
}

// Needed for wasmtime_wasi::preview2
impl WasiView for MyState {
    fn table(&self) -> &ResourceTable {
        &self.host.table
    }
    fn table_mut(&mut self) -> &mut ResourceTable {
        &mut self.host.table
    }
    fn ctx(&self) -> &WasiCtx {
        &self.wasi
    }
    fn ctx_mut(&mut self) -> &mut WasiCtx {
        &mut self.wasi
    }
}

fn main() -> Result<()> {
    let mut config = Config::new();
    config.wasm_component_model(true);
    let engine = Engine::new(&config)?;
    let component = Component::from_file(&engine, "../guest/target/wasm32-wasi/debug/thesis.wasm")?;

    let mut linker = Linker::new(&engine);

    // Bind wasi commmand world
    wasmtime_wasi::preview2::command::sync::add_to_linker(&mut linker)?;
    // Binding host
    Screen::add_to_linker(&mut linker, |state: &mut MyState| &mut state.host)?;

    let wasi = WasiCtxBuilder::new()
        .inherit_stdout()
        .inherit_stdio()
        .build();

    let mut state = MyState {
        host: HostComponent {
            table: ResourceTable::new(),
        },
        wasi,
    };

    let i2cdev = I2cdev::new(format!("/dev/i2c-{}", 1))?;

    let screen = state.host.table.push(I2c(i2cdev))?;
    let delay = state.host.table.push(Delay)?;

    let mut store = Store::new(&engine, state);

    let (bindings, _) = Screen::instantiate(&mut store, &component, &linker)?;

    let _ = bindings
        .sketch_embedded_run()
        .call_run(&mut store, screen, delay)?;

    Ok::<(), anyhow::Error>(())
}
