use embedded_hal::i2c::I2c;
use linux_embedded_hal::I2cdev;

const ADDRESS: u8 = 0x5F;

fn read_register_pair(i2c: &mut I2cdev, register_address: u8) -> Result<i16, anyhow::Error> {
    let mut data: [u8; 2] = [0; 2];
    i2c.write_read(ADDRESS, &[register_address], &mut data)?;
    Ok(((data[1] as i16) << 8) | (data[0] as i16))
}

fn read_register(i2c: &mut I2cdev, register_address: u8) -> Result<u8, anyhow::Error> {
    let mut data: [u8; 1] = [0];
    i2c.write_read(ADDRESS, &[register_address], &mut data)?;

    Ok(data[0])
}

fn data_available(i2c: &mut I2cdev) -> Result<bool, anyhow::Error> {
    let data = read_register(i2c, 0x27)?;

    // Humidity & temperature available
    Ok(data & (1 << 1) > 0 && data & (1 << 0) > 0)
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
fn get_calibration_coefficients(i2c: &mut I2cdev) -> Result<Calibration, anyhow::Error> {
    let mut data: [u8; 16] = [0; 16];
    // Registers start at 0x30. By setting the high bit, we can read all registers in the same transfer.
    i2c.write_read(ADDRESS, &[0x80 | 0x30], &mut data)?;

    Ok(Calibration {
        h0_rh_x2: data[0],
        h1_rh_x2: data[1],
        t0_deg_c_x8: ((((data[5] & 0b11) as u16) << 8) | data[2] as u16),
        t1_deg_c_x8: (((((data[5] & 0b1100) >> 2) as u16) << 8) | data[3] as u16),
        h0_t0_out: (data[7] as i16) << 8 | data[6] as i16,
        h1_t0_out: (data[11] as i16) << 8 | data[10] as i16,
        t0_out: (data[13] as i16) << 8 | data[12] as i16,
        t1_out: (data[15] as i16) << 8 | data[14] as i16,
    })
}

fn read_temperature(i2c: &mut I2cdev) -> Result<i16, anyhow::Error> {
    // TEMP_OUT_L: 0x2A; TEMP_OUT_H: 0x2B
    // We set the high bit to read both in the same transfer
    let raw = read_register_pair(i2c, 0x80 | 0x2A)?;

    // Convert the ADC 16-bit values into degrees Celsius
    const MIN_TEMPERATURE: i16 = -40;
    const MAX_TEMPERATURE: i16 = 120;
    let coefficients = get_calibration_coefficients(i2c)?;

    let t_range_x8 = (coefficients.t1_deg_c_x8 - coefficients.t0_deg_c_x8) as i16;
    let adc_range = coefficients.t1_out - coefficients.t0_out;
    let meas = raw - coefficients.t0_out;

    let temperature_x8 = coefficients.t0_deg_c_x8 as i16
        + (meas as i32 * t_range_x8 as i32 / adc_range as i32) as i16;

    Ok(if temperature_x8 < MIN_TEMPERATURE * 8 {
        MIN_TEMPERATURE * 8
    } else if temperature_x8 > MAX_TEMPERATURE * 8 {
        MAX_TEMPERATURE * 8
    } else {
        temperature_x8
    })
}

fn main() -> Result<(), anyhow::Error> {
    let mut i2c = I2cdev::new("/dev/i2c-1")?;

    while !data_available(&mut i2c).unwrap() {}

    let temperature_x8 = read_temperature(&mut i2c)?;

    println!(
        "Temp = {}.{} deg C",
        temperature_x8 >> 3,
        125 * (temperature_x8 & 0b111)
    );

    Ok(())
}
