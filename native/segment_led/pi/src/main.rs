/// Print the current time to a 4x7 segment display

use rppal::i2c::I2c;
use chrono::Local;

/// The inner workings of this is a complete mystery, but it works.
fn to_data(i: usize) -> u8 {
    [0x3F,0x06,0x5B,0x4F,0x66,0x6D,0x7D,0x07,0x7F,0x6F][(i - 48) % 10]
}

fn main() -> anyhow::Result<()> {
    let mut i2c = I2c::new()?;
  
    i2c.set_slave_address(0x24)?;
    // Set display on
    i2c.write(&[0x81])?;

    // Set brightness to max
    i2c.write(&[(0<<4) | 0x01])?;

    let now = format!("{}", Local::now().format("%H%M"));
    println!("Time is: {}", now);
    let now_bytes = now.as_str().as_bytes();

    for (i, &number) in now_bytes.iter().enumerate() {
        let dig = to_data(number.into());

        i2c.set_slave_address(0x34 + (i as u16))?;
        i2c.write(&[dig])?;
    }

    Ok::<(), anyhow::Error>(())
}
