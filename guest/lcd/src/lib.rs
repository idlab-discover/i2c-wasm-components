#![no_std]
#![no_main]

#[allow(warnings)]
mod bindings;

use crate::bindings::exports::lcd::Guest;
use bindings::wasi::i2c::delay::Delay;
use bindings::wasi::i2c::i2c::I2c;

use lol_alloc::{AssumeSingleThreaded, FreeListAllocator};

extern crate alloc;
use alloc::string::String;

struct Component;

const ADDRESS: u16 = 0x27;
const BACKLIGHT: u8 = 0b0000_1000;
const ENABLE: u8 = 0b0000_0100;
const REGISTER_SELECT: u8 = 0b0000_0001;

fn write_nibble(connection: &mut I2c, nibble: u8, data: bool, delay: &mut Delay) {
    let rs = match data {
        false => 0u8,
        true => REGISTER_SELECT,
    };
    let byte = nibble | rs | BACKLIGHT;

    let _ = connection.write(ADDRESS, &[byte, byte | ENABLE]);
    delay.delay_ns(2_000_000);
    let _ = connection.write(ADDRESS, &[byte]);
}

fn write(connection: &mut I2c, byte: u8, data: bool, delay: &mut Delay) {
    let upper_nibble = byte & 0xF0;
    write_nibble(connection, upper_nibble, data, delay);

    let lower_nibble = (byte & 0x0F) << 4;
    write_nibble(connection, lower_nibble, data, delay);
}

impl Guest for Component {
    fn write(mut connection: I2c, mut delay: Delay, message: String) {
        // reset
        write(&mut connection, 0b0000_0010, false, &mut delay);
        delay.delay_ns(100_000);

        // clear
        write(&mut connection, 0b0000_0001, false, &mut delay);
        delay.delay_ns(100_000);

        // set display mode
        write(&mut connection, 0b0000_1111, false, &mut delay);
        delay.delay_ns(100_000);

        for &b in message.as_bytes() {
            write(&mut connection, b, true, &mut delay);
            delay.delay_ns(100_000);
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
