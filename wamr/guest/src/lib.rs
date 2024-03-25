fn to_data(i: usize) -> u8 {
    [0x3F, 0x06, 0x5B, 0x4F, 0x66, 0x6D, 0x7D, 0x07, 0x7F, 0x6F][(i - 48) % 10]
}

extern "C" {
    fn host_i2c_write(slave_address: u16, data: *const u8, data_len: usize);
}

fn i2c_write(slave_address: u16, data: &[u8]) {
    unsafe { host_i2c_write(slave_address, data.as_ptr(), data.len()) }
}

/// Set the display on and the brightness to max
#[no_mangle]
pub fn setup() {
    // Set display on
    i2c_write(0x24, &[0x81]);

    // Set brightness to max
    i2c_write(0x24, &[(0 << 4) | 0x01]);
}

#[no_mangle]
pub fn write(message: &str) {
    let str_bytes = message.as_bytes();

    for (i, &number) in str_bytes.iter().enumerate() {
        let dig = to_data(number.into());

        i2c_write(0x34 + (i as u16), &[dig]);
    }
}
