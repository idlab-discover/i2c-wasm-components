mod bindings;

use bindings::{Guest, host};

struct Component;


// This address is the default for the used i2c interface
const I2C_ADDRESS: u8 = 0x27;

impl Guest for Component {
    fn start() -> Result<(), ()> {
       let _ = host::i2c_init(I2C_ADDRESS);
       let _ = host::write("Hello, world!");

       Ok(())
    }
}

