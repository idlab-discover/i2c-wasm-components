#![no_std]
#![no_main]
#[allow(warnings)]
mod bindings;

use crate::bindings::exports::sketch::implementation::hts::Guest;
use lol_alloc::{AssumeSingleThreaded, FreeListAllocator};
use wasi_embedded_hal::add_i2c_hal;

use bindings::wasi::i2c::i2c;

#[macro_use]
extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

add_i2c_hal!(i2c);

struct Component {}

impl Guest for Component {
    fn get_humidity(mut connection: I2c) -> Result<String, ErrorCode> {
        let mut hts221 = hts221::Builder::new()
            .with_avg_t(hts221::AvgT::Avg256)
            .with_avg_h(hts221::AvgH::Avg512)
            .build(&mut connection)?;

        let humidity_x2 = hts221.humidity_x2(&mut connection)?;
        Ok(format!(
            "rH = {}.{}%",
            humidity_x2 >> 1,
            5 * (humidity_x2 & 0b1)
        ))
    }

    fn get_temperature(mut connection: I2c) -> Result<String, ErrorCode> {
        let mut hts221 = hts221::Builder::new()
            .with_avg_t(hts221::AvgT::Avg256)
            .with_avg_h(hts221::AvgH::Avg512)
            .build(&mut connection)?;

        let temperature_x8 = hts221.temperature_x8(&mut connection)?;
        Ok(format!(
            "Temp = {}.{} deg C",
            temperature_x8 >> 3,
            125 * (temperature_x8 & 0b111)
        ))
    }
}

/// Define a global allocator, since we're using `no_std`.
/// SAFETY: We're single-threaded.
#[global_allocator]
static GLOBAL_ALLOCATOR: AssumeSingleThreaded<FreeListAllocator> =
    unsafe { AssumeSingleThreaded::new(FreeListAllocator::new()) };

/// Define a panic handler, since we're using `no_std`. Just infloop for
/// now and hope we don't panic.
#[panic_handler]
fn panic(_panic: &core::panic::PanicInfo<'_>) -> ! {
    // Don't panic ;-).
    loop {}
}

bindings::export!(Component with_types_in bindings);
