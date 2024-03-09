#![no_std]
#![no_main]
#[allow(warnings)]
mod bindings;

use crate::bindings::exports::sketch::embedded::temperature::Guest;
use crate::bindings::sketch::embedded::i2c::I2c;
use lol_alloc::{AssumeSingleThreaded, FreeListAllocator};

#[macro_use]
extern crate alloc;
use alloc::string::String;

struct Component;

const ADDRESS: u16 = 0x5F;

fn read_register_pair(i2c: &mut I2c, register_address: u8) -> i16 {
    let mut data = i2c.write_read(ADDRESS, &[register_address], 2).unwrap();
    ((data[1] as i16) << 8) | (data[0] as i16)
}

fn read_register(i2c: &mut I2c, register_address: u8) -> u8 {
    let mut data = i2c.write_read(ADDRESS, &[register_address], 1).unwrap();

    data[0]
}

fn data_available(i2c: &mut I2c) -> bool {
    let data = read_register(i2c, 0x27);

    // Humidity & temperature available
    data & (1 << 1) > 0 && data & (1 << 0) > 0
}

struct Calibration {
    /// Relative humidity from calibration point 0.
    h0_rh_x2: u8,
    /// Relative humidity from calibration point 1.
    h1_rh_x2: u8,
    /// Temperature from calibration point 0.
    t0_deg_c_x8: u16,
    /// Temperature from calibration point 1.
    t1_deg_c_x8: u16,
    /// Humidity ADC reading from calibration point 0.
    h0_t0_out: i16,
    /// Humidity ADC reading from calibration point 1.
    h1_t0_out: i16,
    /// Temperature ADC reading from calibration point 0.
    t0_out: i16,
    /// Temperature ADC reading from calibration point 1.
    t1_out: i16,
}

/// These coefficients are sensor specific, that's why we need to get them.
fn get_calibration_coefficients(i2c: &mut I2c) -> Calibration {
    // Registers start at 0x30. By setting the high bit, we can read all registers in the same transfer.
    let mut data = i2c.write_read(ADDRESS, &[0x80 | 0x30], 16).unwrap();

    Calibration {
        h0_rh_x2: data[0],
        h1_rh_x2: data[1],
        t0_deg_c_x8: ((((data[5] & 0b11) as u16) << 8) | data[2] as u16),
        t1_deg_c_x8: (((((data[5] & 0b1100) >> 2) as u16) << 8) | data[3] as u16),
        h0_t0_out: (data[7] as i16) << 8 | data[6] as i16,
        h1_t0_out: (data[11] as i16) << 8 | data[10] as i16,
        t0_out: (data[13] as i16) << 8 | data[12] as i16,
        t1_out: (data[15] as i16) << 8 | data[14] as i16,
    }
}

fn read_temperature(i2c: &mut I2c) -> i16 {
    // TEMP_OUT_L: 0x2A; TEMP_OUT_H: 0x2B
    // We set the high bit to read both in the same transfer
    let raw = read_register_pair(i2c, 0x80 | 0x2A);

    // Convert the ADC 16-bit values into degrees Celsius
    const MIN_TEMPERATURE: i16 = -40;
    const MAX_TEMPERATURE: i16 = 120;
    let coefficients = get_calibration_coefficients(i2c);

    let t_range_x8 = (coefficients.t1_deg_c_x8 - coefficients.t0_deg_c_x8) as i16;
    let adc_range = coefficients.t1_out - coefficients.t0_out;
    let meas = raw - coefficients.t0_out;

    let temperature_x8 = coefficients.t0_deg_c_x8 as i16
        + (meas as i32 * t_range_x8 as i32 / adc_range as i32) as i16;

    if temperature_x8 < MIN_TEMPERATURE * 8 {
        MIN_TEMPERATURE * 8
    } else if temperature_x8 > MAX_TEMPERATURE * 8 {
        MAX_TEMPERATURE * 8
    } else {
        temperature_x8
    }
}
impl Guest for Component {
    fn run(mut connection: I2c) -> String {
        while !data_available(&mut connection) {}

        let temperature_x8 = read_temperature(&mut connection);

        format!(
            "Temp = {}.{} deg C",
            temperature_x8 >> 3,
            125 * (temperature_x8 & 0b111)
        )
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
