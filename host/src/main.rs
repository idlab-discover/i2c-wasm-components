use wasmtime::{
    component::{bindgen, Component, Linker},
    Config, Engine, Result, Store,
};

use hd44780_driver::{bus, Cursor, CursorBlink, Display, DisplayMode, HD44780};
use rppal::hal::Delay;
use rppal::i2c::I2c;

bindgen!("i2c-app" in "../wit/my-component.wit");

struct HostComponent {
    lcd: Option<HD44780<bus::I2CBus<I2c>>>,
    delay: Option<Delay>,
}

impl host::Host for HostComponent {
    fn i2c_init(
        &mut self,
        address: u8,
    ) -> wasmtime::Result<std::result::Result<u32, u32>> {
        let i2c = I2c::new().unwrap();
        let mut delay = Delay::new();
        let mut lcd = HD44780::new_i2c(i2c, address, &mut delay).unwrap();

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
        )
        .unwrap();

        self.lcd = Some(lcd);
        self.delay = Some(delay);

        Ok(Ok(1))
    }

    fn write(
        &mut self,
        message: String,
    ) -> wasmtime::Result<std::result::Result<u32, u32>> {
        self.lcd
            .as_mut()
            .expect("lcd is not initiated")
            .write_str(&message, self.delay.as_mut().expect("lcd is not initiated"))
            .unwrap();

        Ok(Ok(1))
    }
}

struct MyState {
    host: HostComponent,
}

fn main() -> Result<()> {
    // Configure an `Engine` and compile the `Component` that is being run for
    // the application.
    // Async support is needed for wasmtime linker
    let engine = Engine::new(Config::new().wasm_component_model(true))?;
    let path = "../guest/target/wasm32-wasi/debug/i2c_app.wasm";
    // let component = convert_to_component("../guest/target/wasm32-wasi/debug/guest.wasm")?;

    // Create our component and call our generated host function.
    // let component = Component::from_binary(&engine, &component)?;
    let component = Component::from_file(&engine, path)?;
    let mut store = Store::new(
        &engine,
        MyState {
            host: HostComponent {
                lcd: None,
                delay: None,
            },
        },
    );
    let mut linker = Linker::new(&engine);
    host::add_to_linker(&mut linker, |state: &mut MyState| &mut state.host)?;
    let (i2c_app, _instance) = I2cApp::instantiate(&mut store, &component, &linker)?;

    let _ = i2c_app.call_start(&mut store)?;

    Ok(())
}
