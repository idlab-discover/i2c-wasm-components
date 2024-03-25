use chrono::Local;
use once_cell::sync::Lazy;
use rppal::i2c::I2c;
use std::{ffi::c_void, path::PathBuf};
use wamr_rust_sdk::{
    function::Function, instance::Instance, module::Module, runtime::Runtime, value::WasmValue,
    RuntimeError,
};

#[allow(dead_code)]
static mut I2C: once_cell::sync::Lazy<I2c> = Lazy::new(|| I2c::new().unwrap());

#[allow(dead_code)]
extern "C" fn host_i2c_write(slave_address: u16, data: *const u8, data_len: usize) {
    let slice = unsafe { std::slice::from_raw_parts(data, data_len) };
    unsafe {
        let _ = I2C.set_slave_address(slave_address);
        let _ = I2C.write(slice);
    };
}

fn main() -> Result<(), RuntimeError> {
    let runtime = Runtime::builder()
        .use_system_allocator()
        .register_host_function("host_i2c_write", host_i2c_write as *mut c_void)
        .build()?;

    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("guest.wasm");
    let module = Module::from_file(&runtime, d.as_path())?;

    // let wasi_ctx = WasiCtxBuilder::new()
    //     .set_pre_open_path(vec!["."], vec![])
    //     .build();

    // module.set_wasi_context(wasi_ctx);

    let instance = Instance::new(&runtime, &module, 1024 * 64)?;

    let function = Function::find_export_func(&instance, "setup")?;
    function.call(&instance, &vec![WasmValue::Void])?;

    let function = Function::find_export_func(&instance, "write")?;

    let now = format!("{}", Local::now().format("%H%M"));
    let params: Vec<WasmValue> = now
        .as_str()
        .as_bytes()
        .iter()
        .map(|b| WasmValue::I32(i32::from(*b)))
        .collect();

    // let params: Vec<WasmValue> = vec![WasmValue::I32(1)];
    function.call(&instance, &params)?;

    // Via encode we go from a WasmValue to a Vec of the decoded type
    // println!("{}", result.encode()[0]);
    // assert_eq!(result, WasmValue::I32(9));

    Ok(())
}
