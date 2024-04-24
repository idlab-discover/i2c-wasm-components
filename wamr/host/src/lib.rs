use chrono::Local;
use once_cell::sync::Lazy;
use rppal::i2c::I2c;
use std::{ffi::c_void, path::PathBuf, vec};
use wamr_rust_sdk::{
    function::Function, instance::Instance, module::Module, runtime::Runtime, value::WasmValue,
    wasi_context::WasiCtxBuilder, RuntimeError,
};

static mut I2C: once_cell::sync::Lazy<I2c> = Lazy::new(|| I2c::new().unwrap());

extern "C" fn host_i2c_write(_exec_env: u16, slave_address: u16, data: u8) {
    let slice = &[data];
    unsafe {
        let _ = I2C.set_slave_address(slave_address);
        let _ = I2C.write(slice);
    };
}

pub fn execute() -> Result<(), RuntimeError> {
    let runtime = Runtime::builder()
        .use_system_allocator()
        .register_host_function("host_i2c_write", host_i2c_write as *mut c_void)
        .build()?;

    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("guest.wasm");
    let mut module = Module::from_file(&runtime, d.as_path())?;

    let wasi_ctx = WasiCtxBuilder::new()
        .set_pre_open_path(vec!["."], vec![])
        .build();

    module.set_wasi_context(wasi_ctx);

    let instance = Instance::new(&runtime, &module, 1024 * 64)?;

    let function = Function::find_export_func(&instance, "setup")?;
    function.call(&instance, &vec![WasmValue::Void])?;

    let function = Function::find_export_func(&instance, "write")?;

    let now = format!("{}", Local::now().format("%H%M"));
    let params = now
        .as_bytes()
        .iter()
        .map(|b| WasmValue::I32(i32::from(*b)))
        .collect();

    function.call(&instance, &params)?;

    Ok(())
}

