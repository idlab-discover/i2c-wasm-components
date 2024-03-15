#![no_std]
#![no_main]
#[allow(warnings)]
mod bindings;

use crate::bindings::exports::sketch::embedded::hts::Guest;
use crate::bindings::sketch::embedded::i2c::{ErrorCode, I2c, NoAcknowledgeSource};
use bindings::sketch::embedded::i2c;
use lol_alloc::{AssumeSingleThreaded, FreeListAllocator};

#[macro_use]
extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug)]
pub struct I2CError {
    err: ErrorCode,
}

// impl From<

impl embedded_hal::i2c::Error for I2CError {
    fn kind(&self) -> embedded_hal::i2c::ErrorKind {
        use embedded_hal::i2c::ErrorKind;

        let errno = self.err;

        match errno {
            ErrorCode::Bus => ErrorKind::Bus,
            ErrorCode::ArbitrationLoss => ErrorKind::ArbitrationLoss,
            ErrorCode::NoAcknowledge(sour) => match sour {
                NoAcknowledgeSource::Address => {
                    ErrorKind::NoAcknowledge(embedded_hal::i2c::NoAcknowledgeSource::Address)
                }
                NoAcknowledgeSource::Data => {
                    ErrorKind::NoAcknowledge(embedded_hal::i2c::NoAcknowledgeSource::Data)
                }
                NoAcknowledgeSource::Unknown => {
                    ErrorKind::NoAcknowledge(embedded_hal::i2c::NoAcknowledgeSource::Unknown)
                }
            },
            ErrorCode::Overrun => ErrorKind::Overrun,
            ErrorCode::Other => ErrorKind::Other,
        }
    }
}

// pub struct I2c0 {
//     inner: I2c,
// }

impl From<I2CError> for ErrorCode {
    fn from(value: I2CError) -> Self {
        value.err
    }
}

impl embedded_hal::i2c::ErrorType for I2c {
    type Error = I2CError;
}

impl embedded_hal::i2c::I2c for I2c {
    fn transaction(
        &mut self,
        address: u8,
        operations: &mut [embedded_hal::i2c::Operation<'_>],
    ) -> Result<(), Self::Error> {
        let opers = operations
            .iter_mut()
            .map(|a| match a {
                embedded_hal::i2c::Operation::Read(r) => {
                    i2c::Operation::Read(r.len().try_into().unwrap())
                }
                embedded_hal::i2c::Operation::Write(w) => i2c::Operation::Write(w.to_vec()),
            })
            .collect::<Vec<i2c::Operation>>();

        let _ = Self::transaction(self, u16::from(address), &opers);

        Ok(())
    }
}

// impl embedded_hal::i2c::I2c<embedded_hal::i2c::SevenBitAddress> for I2c0 {
//     fn transaction(
//         &mut self,
//         address: embedded_hal::i2c::SevenBitAddress,
//         mut operations: &mut [embedded_hal::i2c::Operation<'_>],
//     ) -> Result<(), Self::Error> {
//         embedded_hal::i2c::I2c::<embedded_hal::i2c::TenBitAddress>::transaction(
//             self,
//             u16::from(address),
//             &mut operations,
//         )
//     }
// }

// impl embedded_hal::i2c::I2c<embedded_hal::i2c::TenBitAddress> for I2c0 {
//     fn transaction(
//         &mut self,
//         address: embedded_hal::i2c::TenBitAddress,
//         mut operations: &mut [embedded_hal::i2c::Operation<'_>],
//     ) -> Result<(), Self::Error> {
//         self.inner.transaction(address, operations)
//     }
// }

// impl embedded_hal::i2c::I2c for I2c0 {
//     fn transaction(
//         &mut self,
//         address: embedded_hal::i2c::SevenBitAddress,
//         mut operations: &mut [embedded_hal::i2c::Operation],
//     ) -> Result<(), Self::Error> {
//         self.inner.transaction(u16::from(address), operations)
//     }
// }

struct Component {}

impl Guest for Component {
    fn get_humidity(mut connection: I2c) -> Result<String, ErrorCode> {
        let mut hts221 = hts221::Builder::new()
            .with_avg_t(hts221::AvgT::Avg256)
            .with_avg_h(hts221::AvgH::Avg512)
            .build(&mut connection)?;

        let humidity_x2 = hts221.humidity_x2(&mut connection)?;
        Ok(format!(
            "rH = {}.{}%",
            humidity_x2 >> 1,
            5 * (humidity_x2 & 0b1)
        ))
    }

    fn get_temperature(mut connection: I2c) -> Result<String, ErrorCode> {
        let mut hts221 = hts221::Builder::new()
            .with_avg_t(hts221::AvgT::Avg256)
            .with_avg_h(hts221::AvgH::Avg512)
            .build(&mut connection)?;

        let temperature_x8 = hts221.temperature_x8(&mut connection)?;
        Ok(format!(
            "Temp = {}.{} deg C",
            temperature_x8 >> 3,
            125 * (temperature_x8 & 0b111)
        ))
    }
}

/// Define a global allocator, since we're using `no_std`.
/// SAFETY: We're single-threaded.
#[global_allocator]
static GLOBAL_ALLOCATOR: AssumeSingleThreaded<FreeListAllocator> =
    unsafe { AssumeSingleThreaded::new(FreeListAllocator::new()) };

/// Define a panic handler, since we're using `no_std`. Just infloop for
/// now and hope we don't panic.
#[panic_handler]
fn panic(_panic: &core::panic::PanicInfo<'_>) -> ! {
    // Don't panic ;-).
    loop {}
}

bindings::export!(Component with_types_in bindings);
