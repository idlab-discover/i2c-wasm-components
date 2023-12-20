use crate::my::project::types;

use wasmtime::component::{Component, Linker};
use wasmtime::{Config, Engine, Store};
use wasmtime_wasi::preview2::{Table, WasiCtx, WasiView, WasiCtxBuilder};
use futures::executor::block_on;
use async_trait::async_trait;

use hd44780_driver::{Cursor, CursorBlink, Display, DisplayMode, HD44780, bus};
use rppal::hal::Delay;
use rppal::i2c::I2c;

// Imports will be async functions through #[async_trait] and exports
// are also invoked as async functions. Requires `Config::async_support`
// to be `true`.
wasmtime::component::bindgen!({
    path: "../wit",
    world: "i2c-app",
    async: true,    // wasmtime-wasi currently only has an async implementation
});

struct MyState {
    lcd: Option<HD44780<bus::I2CBus<I2c>>>,
    delay: Option<Delay>,
    table: Table,
    wasi: WasiCtx
}

#[async_trait]
impl types::Host for MyState {
    async fn i2c_init(&mut self, address: u8) -> std::result::Result<std::result::Result<u32, u32>, wasmtime::Error> {
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
        ).unwrap();

        self.lcd = Some(lcd);
        self.delay = Some(delay);

        Ok(Ok(1))
    }
    
    async fn write(&mut self, message: String) -> std::result::Result<std::result::Result<u32, u32>, wasmtime::Error> {
        self.lcd.as_mut().expect("lcd is not initiated").write_str(&message, self.delay.as_mut().expect("lcd is not initiated")).unwrap();

        Ok(Ok(1))
    }
}

// Needed for wasmtime_wasi::preview2
impl WasiView for MyState {
    fn table(&self) -> &Table {
        &self.table
    }
    fn table_mut(&mut self) -> &mut Table {
        &mut self.table
    }
    fn ctx(&self) -> &WasiCtx {
        &self.wasi
    }
    fn ctx_mut(&mut self) -> &mut WasiCtx {
        &mut self.wasi
    }
}

fn main() -> wasmtime::Result<()> {
    // Configure an `Engine` and compile the `Component` that is being run for
    // the application.
    // Async support is needed for wasmtime linker
    let mut config = Config::new();
    config.wasm_component_model(true)
          .async_support(true);
    let engine = Engine::new(&config)?;
    let component = Component::from_file(&engine, "../guest/guest.component.wasm")?;

    // Instantiation of bindings always happens through a `Linker`.
    // Configuration of the linker is done through a generated `add_to_linker`
    // method on the bindings structure.
    //
    // Note that the closure provided here is a projection from `T` in
    // `Store<T>` to `&mut U` where `U` implements the `HelloWorldImports`
    // trait. In this case the `T`, `MyState`, is stored directly in the
    // structure so no projection is necessary here.
    let mut linker = Linker::new(&engine);

    // Bind wasi commmand world
    wasmtime_wasi::preview2::command::add_to_linker(&mut linker)?;
    // Binding host
    I2cApp::add_to_linker(&mut linker, |state: &mut MyState| state)?;

    // As with the core wasm API of Wasmtime instantiation occurs within a
    // `Store`. The bindings structure contains an `instantiate` method which
    // takes the store, component, and linker. This returns the `bindings`
    // structure which is an instance of `HelloWorld` and supports typed access
    // to the exports of the component.
    let table = Table::new();

    let wasi = WasiCtxBuilder::new()
            // .inherit_stderr()
            // .inherit_stdin()
            .inherit_stdout()
            .inherit_stdio()
            .build();

    let mut store = Store::new(
        &engine,
        MyState {
            lcd: None,
            delay: None,
            table,
            wasi
        },
    );

    block_on(async {
        let (bindings, _) = I2cApp::instantiate_async(&mut store, &component, &linker).await?;

        let _ = bindings.call_start(&mut store).await?;

        Ok::<(), anyhow::Error>(())
    })?;

    Ok(())
}
