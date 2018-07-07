//! A platform agnostic driver to interface with the TSL256X (Lux)
//!
//! This driver was built using [`embedded-hal`] traits.
//!
//! [`embedded-hal`]: https://docs.rs/embedded-hal

#![deny(missing_docs)]
#![deny(warnings)]
#![no_std]

extern crate embedded_hal as hal;
use hal::blocking::i2c;

/// TSL2561 driver
pub struct Tsl2561<I2C> {
    i2c: I2C,
    address: u8,
}

impl<I2C, E> Tsl2561<I2C>
    where
        I2C: i2c::WriteRead<Error = E> + i2c::Write<Error = E>,
{
    /// Creates a new driver from a I2C peripheral
    pub fn new(i2c: I2C) -> Result<Self, E> {
        let mut tsl2561 = Tsl2561 { i2c, address: 0x39 };

        tsl2561.power_on()?;

        Ok(tsl2561)
    }

    /// Power on the device
    /// Sensor readings are initialized to zero
    /// Actual sensor readings are not available until one integration period has passed (default 400ms)
    pub fn power_on(&mut self) -> Result<(), E> {
        let command = Command::new(Register::CONTROL).value();
        let power_on = 0x03;
        self.i2c.write(self.address, &[command, power_on])
    }

    /// Raw value from channel 0 ADC measuring IR + Visible spectrum
    pub fn visible_and_ir_raw(&mut self) -> Result<u16, E> {
        let command = Command::new(Register::DATA0LOW)
            .enable_word_protocol().value();

        let mut buffer : [u8;2] = [0;2];
        self.i2c.write_read(self.address, &[command], &mut buffer)?;
        let result = ((buffer[1] as u16) << 8) | buffer[0] as u16;

        Ok(result)
    }

    /// Raw value from channel 1 ADC measuring IR spectrum only
    pub fn ir_raw(&mut self) -> Result<u16, E> {
        let command = Command::new(Register::DATA1LOW)
            .enable_word_protocol().value();

        let mut buffer : [u8;2] = [0;2];
        self.i2c.write_read(self.address, &[command], &mut buffer)?;
        let result = ((buffer[1] as u16) << 8) | buffer[0] as u16;

        Ok(result)
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
#[derive(Copy, Clone)]
enum Register {
    CONTROL = 0x0,
    TIMING = 0x1,
    THRESHLOWLOW = 0x2,
    THRESHLOWHIGH = 0x3,
    THRESHHIGHLOW = 0x4,
    THRESHHIGHHIGH = 0x5,
    INTERRUPT = 0x6,
    CRC = 0x8,
    ID = 0xA,
    DATA0LOW = 0xC,
    DATA0HIGH = 0xD,
    DATA1LOW = 0xE,
    DATA1HIGH = 0xF,
}

impl Register {
    pub fn addr(&self) -> u8 {
        *self as u8
    }
}

struct Command {
    cmd: bool,
    clear: bool,
    word: bool,
    block: bool,
    register: Register,
}

impl Command {
    fn new(register: Register) -> Self {
        Self {
            cmd: true,
            clear: false,
            word: false,
            block: false,
            register,
        }
    }

    fn enable_word_protocol(mut self) -> Self {
        self.word = true;
        self
    }

    fn value(self) -> u8 {
        self.register.addr() |
            ((self.cmd as u8) << 7) |
            ((self.clear as u8) << 6) |
            ((self.word as u8) << 5) |
            ((self.block as u8) << 4)
    }
}
