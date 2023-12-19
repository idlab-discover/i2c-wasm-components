use hd44780_driver::{Cursor, CursorBlink, Display, DisplayMode, HD44780};
use rppal::hal::Delay;
use rppal::i2c::I2c;

wit_bindgen::generate!({
    path: "../wit",
    world: "i2c-app",
    exports: {
        world: Component
    }
});

struct Component;


// This address is the default for the used i2c interface
const I2C_ADDRESS: u8 = 0x27;

impl Guest for Component {
    fn write() {
        let i2c = I2c::new().unwrap();
        let mut delay = Delay::new();
        let mut lcd = HD44780::new_i2c(i2c, I2C_ADDRESS, &mut delay).unwrap();

        // Unshift display and set cursor to 0
        lcd.reset(&mut delay).unwrap();
        // Clear screen
        lcd.clear(&mut delay).unwrap();
        lcd.set_display_mode(
            DisplayMode {
                display: Display::On,
                cursor_visibility: Cursor::Visible,
                cursor_blink: CursorBlink::On,
            },
            &mut delay,
        ).unwrap();

        lcd.write_str(&message(), &mut delay).unwrap();
    }
}

