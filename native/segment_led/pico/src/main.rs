#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_rp::gpio::{Level, Output};
use embassy_rp::i2c::{self, Config};
use embassy_time::Delay;
use embedded_hal::delay::DelayNs;
use embedded_hal::i2c::I2c;
use {defmt_rtt as _, panic_probe as _};

const ADDRESS: u8 = 0x24;

/// The inner workings of this is a complete mystery, but it works.
fn to_data(i: usize) -> u8 {
    [0x3F,0x06,0x5B,0x4F,0x66,0x6D,0x7D,0x07,0x7F,0x6F][(i - 48) % 10]
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    // Blink the onboard led
    let mut led = Output::new(p.PIN_25, Level::Low);

    let sda = p.PIN_0;
    let scl = p.PIN_1;

    let mut connection: i2c::I2c<'_, embassy_rp::peripherals::I2C0, i2c::Blocking> =
        i2c::I2c::new_blocking(p.I2C0, scl, sda, Config::default());
    let mut delay = Delay;

    delay.delay_ns(2_000_000_000);
    led.set_high();

    // Turn on
    let _ = connection.write(ADDRESS, &[0x81]);


    // Set brightness to max
    let _ = connection.write(ADDRESS, &[(0<<4) | 0x01]);

    let now = "13:30";
    let now_bytes = now.as_bytes();

    for (i, &number) in now_bytes.iter().enumerate() {
        let dig = to_data(number.into());

        let _ = connection.write(0x34 + (i as u8), &[dig]);
    }
    
    delay.delay_ns(1_000_000_000);
    led.set_low();
}
