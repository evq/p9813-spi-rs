#![no_std]

extern crate embedded_hal as hal;

use hal::{
    blocking::spi::Write,
    spi::{Mode, Phase, Polarity},
};

use smart_leds_trait::{SmartLedsWrite, RGB8};

/// SPI mode that can be used for this crate
///
/// Provided for convenience
/// Doesn't really matter
pub const MODE: Mode = Mode {
    polarity: Polarity::IdleLow,
    phase: Phase::CaptureOnFirstTransition,
};

pub struct P9813<SPI> {
    spi: SPI,
}

impl<SPI, E> P9813<SPI>
where
    SPI: Write<u8, Error = E>,
{
    pub fn new(spi: SPI) -> P9813<SPI> {
        Self { spi }
    }

    fn flush(&mut self) -> Result<(), E> {
        self.spi.write(&[0; 20])?;
        Ok(())
    }
}

impl<SPI, E> SmartLedsWrite for P9813<SPI>
where
    SPI: Write<u8, Error = E>,
{
    type Pixel = RGB8;
    type Error = E;

    /// Write all the items of an iterator to a p9813 strip
    fn write<T, I>(&mut self, iterator: T) -> Result<(), E>
    where
        T: Iterator<Item = I>,
        I: Into<RGB8>,
    {
        self.flush()?;

        for item in iterator {
            let item = item.into();
            let r = item.r;
            let g = item.g;
            let b = item.b;
            let top = 0xC0 | ((!b & 0xC0) >> 2) | ((!g & 0xC0) >> 4) | ((!r & 0xC0) >> 6);

            self.spi.write(&[top, b, g, r])?;
        }

        self.flush()?;

        Ok(())
    }
}
