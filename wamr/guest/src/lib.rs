#![no_std]
#![no_main]

use lol_alloc::{AssumeSingleThreaded, FreeListAllocator};

fn to_data(i: usize) -> u8 {
    [0x3F, 0x06, 0x5B, 0x4F, 0x66, 0x6D, 0x7D, 0x07, 0x7F, 0x6F][(i - 48) % 10]
}

#[link(wasm_import_module = "host")]
extern "C" {
    fn host_i2c_write(slave_address: u16, data: u8);
}

fn i2c_write(slave_address: u16, data: u8) {
    // println!("WASM i2c_write: {:?} {:?}", slave_address, data);
    unsafe { host_i2c_write(slave_address, data) }
}

/// Set the display on and the brightness to max
#[export_name = "setup"]
pub fn setup() {
    // Set display on
    i2c_write(0x24, 0x81);

    // Set brightness to max
    i2c_write(0x24, (0 << 4) | 0x01);
}

#[export_name = "write"]
pub fn write(d0: i32, d1: i32, d2: i32, d3: i32) {
    for (i, &number) in [d0, d1, d2, d3].iter().enumerate() {
        let dig = to_data(number as usize);

        i2c_write(0x34 + (i as u16), dig);
    }
}

/// no_std required stuff
#[global_allocator]
static GLOBAL_ALLOCATOR: AssumeSingleThreaded<FreeListAllocator> =
    unsafe { AssumeSingleThreaded::new(FreeListAllocator::new()) };

#[panic_handler]
fn panic(_panic: &core::panic::PanicInfo<'_>) -> ! {
    loop {}
}
