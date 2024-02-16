mod bindings;

use bindings::Guest;
use crate::bindings::my::project::types::{i2c_init, write};

// This address is the default for the used i2c interface
const I2C_ADDRESS: u8 = 0x27;

struct Component;

impl Guest for Component {
    fn start() -> Result<(), ()> {
       let _ = i2c_init(I2C_ADDRESS);
       let _ = write("Hello, world!");

       Ok(())
    }
}
