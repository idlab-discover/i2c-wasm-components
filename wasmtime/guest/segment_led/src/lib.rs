#![no_std]
#![no_main]

#[allow(warnings)]
mod bindings;

use crate::bindings::exports::sketch::implementation::lcd::Guest;
use bindings::wasi::i2c::delay::Delay;
use bindings::wasi::i2c::i2c::I2c;

use lol_alloc::{AssumeSingleThreaded, FreeListAllocator};

extern crate alloc;
use alloc::string::String;

/// The inner workings of this is a complete mystery, but it works.
fn to_data(i: usize) -> u8 {
    [0x3F,0x06,0x5B,0x4F,0x66,0x6D,0x7D,0x07,0x7F,0x6F][(i - 48) % 10]
}

struct Component;

impl Guest for Component {
    fn write(connection: I2c, _delay: Delay, message: String) {
        // Set display on
        let _ = connection.write(0x24, &[0x81]);

        // Set brightness to max
        let _ = connection.write(0x24, &[(0<<4) | 0x01]);

        let message_bytes = message.as_str().as_bytes();
        for (i, &number) in message_bytes.iter().enumerate() {
            let dig = to_data(number.into());

            let _ = connection.write(0x34 + (i as u16), &[dig]);
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
