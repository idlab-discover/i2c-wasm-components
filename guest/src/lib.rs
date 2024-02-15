wit_bindgen::generate!({
    path: "../wit",
    world: "i2c-app",
    exports: {
        world: GuestComponent
    }
});

struct GuestComponent;


// This address is the default for the used i2c interface
const I2C_ADDRESS: u8 = 0x27;

impl Guest for GuestComponent {
    fn start() -> Result<(), ()> {
       let _ = host::i2c_init(I2C_ADDRESS);
       let _ = host::write("Hello, world!");

       Ok(())
    }
}

