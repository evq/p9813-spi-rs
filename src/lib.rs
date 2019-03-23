#![no_std]

extern crate embedded_hal as hal;

use hal::spi::{FullDuplex, Mode, Phase, Polarity};

use smart_leds_trait::{Color, SmartLedsWrite};

use nb;
use nb::block;

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
    SPI: FullDuplex<u8, Error = E>,
{
    pub fn new(spi: SPI) -> P9813<SPI> {
        Self { spi }
    }

    fn flush(&mut self) -> Result<(), E> {
        for _ in 0..20 {
            block!(self.spi.send(0))?;
            block!(self.spi.read())?;
        }
        Ok(())
    }
}

impl<SPI, E> SmartLedsWrite for P9813<SPI>
where
    SPI: FullDuplex<u8, Error = E>,
{
    type Error = E;
    /// Write all the items of an iterator to a p9813 strip
    fn write<T>(&mut self, iterator: T) -> Result<(), E>
    where
        T: Iterator<Item = Color>,
    {
        self.flush()?;

        for item in iterator {
            let r = item.r;
            let g = item.g;
            let b = item.b;
            let top = 0xC0 | ((!b & 0xC0) >> 2) | ((!g & 0xC0) >> 4) | ((!r & 0xC0) >> 6);

            block!(self.spi.send(top))?;
            block!(self.spi.read())?;
            block!(self.spi.send(b))?;
            block!(self.spi.read())?;
            block!(self.spi.send(g))?;
            block!(self.spi.read())?;
            block!(self.spi.send(r))?;
            block!(self.spi.read())?;
        }

        self.flush()?;

        Ok(())
    }
}
