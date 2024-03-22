#![no_std]
#![no_main]
#[allow(warnings)]
mod bindings;

use crate::bindings::exports::run::Guest;
use crate::bindings::sketch::implementation::hts;
use crate::bindings::wasi::i2c::delay::Delay;
use crate::bindings::wasi::i2c::i2c::{ErrorCode, I2c};
use lol_alloc::{AssumeSingleThreaded, FreeListAllocator};

extern crate alloc;
use alloc::string::String;

struct Component {}

impl Guest for Component {
    fn run(connection: I2c, delay: Delay, device: String) -> Result<String, ErrorCode> {
        match device.as_str() {
            "screen" => hts::get_temperature(connection),
            _ => todo!(),
        }
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
