//! Blinks the LED on a Pico board
//!
//! This will blink an LED attached to GP25, which is the pin the Pico uses for the on-board LED.
#![no_std]
#![no_main]

use bsp::entry;
use defmt::*;
use defmt_rtt as _;
use embedded_hal::blocking::i2c::Write;
use panic_probe as _;

// Time handling traits:
use fugit::RateExtU32;

// Provide an alias for our BSP so we can switch targets quickly.
// Uncomment the BSP you included in Cargo.toml, the rest of the code does not need to change.
use rp_pico as bsp;
// use sparkfun_pro_micro_rp2040 as bsp;

use bsp::hal::{
    clocks::{init_clocks_and_plls, Clock},
    pac,
    sio::Sio,
    watchdog::Watchdog,
};

/// The inner workings of this is a complete mystery, but it works.
fn to_data(i: usize) -> u8 {
    [0x3F, 0x06, 0x5B, 0x4F, 0x66, 0x6D, 0x7D, 0x07, 0x7F, 0x6F][i % 10]
}

#[entry]
fn main() -> ! {
    info!("Program start");
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();
    let mut watchdog = Watchdog::new(pac.WATCHDOG);
    let sio = Sio::new(pac.SIO);

    // External high-speed crystal on the pico board is 12Mhz
    let external_xtal_freq_hz = 12_000_000u32;
    let clocks = init_clocks_and_plls(
        external_xtal_freq_hz,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    let pins = bsp::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    // Configure two pins as being I²C, not GPIO
    let sda_pin: bsp::hal::gpio::Pin<_, bsp::hal::gpio::FunctionI2C, _> = pins.gpio4.reconfigure();
    let scl_pin: bsp::hal::gpio::Pin<_, bsp::hal::gpio::FunctionI2C, _> = pins.gpio5.reconfigure();

    // Create the I²C driver, using the two pre-configured pins. This will fail
    // at compile time if the pins are in the wrong mode, or if this I²C
    // peripheral isn't available on these pins!
    let mut i2c = bsp::hal::I2C::i2c0(
        pac.I2C0,
        sda_pin,
        scl_pin,
        400.kHz(),
        &mut pac.RESETS,
        &clocks.system_clock,
    );

    // Set display on
    i2c.write(0x24u8, &[0x81]).unwrap();

    // Set brightness to max
    i2c.write(0x24u8, &[(0 << 4) | 0x01]).unwrap();

    // Simple incrementing counter
    let mut ctr: [u8; 4] = [0, 0, 0, 0];
    loop {
        for (i, &number) in ctr.iter().enumerate() {
            delay.delay_ms(500);

            let dig = to_data(number.into());
            i2c.write(0x34 + (i as u8), &[dig]).unwrap();
        }

        ctr = match ctr {
            [a, b, c, 9] => [a, b, c + 1, 0],
            [a, b, 9, d] => [a, b + 1, 0, d],
            [a, 9, c, d] => [a + 1, 0, c, d],
            [9, b, c, d] => [0, b, c, d],
            [a, b, c, d] => [a, b, c, d + 1],
        };
    }
}
