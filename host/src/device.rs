use wasmtime_wasi::preview2::{ResourceTable, WasiCtx, WasiView};
pub mod screen;
pub mod sensor;

// Ideally we would like to define the I2C implementation here so that it is shared across devices,
// but this is currently not possible. (see: https://github.com/bytecodealliance/wit-bindgen/issues/546#issuecomment-1489213305)

struct HostComponent {
    table: ResourceTable,
}

pub struct MyState {
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
