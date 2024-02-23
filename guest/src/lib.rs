mod bindings;

use crate::bindings::exports::sketch::embedded::run::Guest;
use crate::bindings::sketch::embedded::delay::Delay;
use crate::bindings::sketch::embedded::i2c::I2c;

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
    fn run(mut connection: I2c, mut delay: Delay) {
        for &b in "hello world".as_bytes() {
            write(&mut connection, b, true, &mut delay);
            delay.delay_ns(100_000);
        }
    }
}
