//! A platform agnostic driver to interface with the TSL256X (Lighting Intensity)
//!
//! This driver was built using [`embedded-hal`] traits.
//!
//! [`embedded-hal`]: https://docs.rs/embedded-hal

#![deny(missing_docs)]
#![deny(warnings)]
#![no_std]

extern crate embedded_hal as hal;
use hal::blocking::i2c;

use core::marker::PhantomData;

/// TSL2561 driver
pub struct Tsl2561<I2C> {
    i2c: PhantomData<I2C>,
    address: u8,
}

impl<I2C, E> Tsl2561<I2C>
    where
        I2C: i2c::WriteRead<Error = E> + i2c::Write<Error = E>,
{
    /// Creates a new sensor driver associated with an I2C peripheral
    /// Phantom I2C ensures whichever I2C bus the device was created on is the one that is used for all future interactions
    pub fn new(_i2c: &I2C, address: u8) -> Result<Self, E> {
        let tsl2561 = Tsl2561 {
            i2c: PhantomData,
            address,
        };

        Ok(tsl2561)
    }

    /// Power on the device
    /// Sensor readings are initialized to zero
    /// Actual sensor readings are not available until one integration period has passed (default 400ms)
    pub fn power_on(&self, i2c: &mut I2C) -> Result<(), E> {
        let command = Command::new(Register::CONTROL).value();
        let power_on = 0x03;
        i2c.write(self.address, &[command, power_on])
    }

    /// Power down the device
    /// Sensor readings are initialized to zero
    pub fn power_off(&self, i2c: &mut I2C) -> Result<(), E> {
        let command = Command::new(Register::CONTROL).value();
        let power_on = 0x00;
        i2c.write(self.address, &[command, power_on])
    }

    /// Raw value from channel 0 ADC measuring IR + Visible spectrum
    pub fn visible_and_ir_raw(&self, i2c: &mut I2C) -> Result<u16, E> {
        let command = Command::new(Register::DATA0LOW)
            .enable_word_protocol().value();

        let mut buffer : [u8;2] = [0;2];
        i2c.write_read(self.address, &[command], &mut buffer)?;
        let result = ((buffer[1] as u16) << 8) | buffer[0] as u16;

        Ok(result)
    }

    /// Raw value from channel 1 ADC measuring IR spectrum only
    pub fn ir_raw(&self, i2c: &mut I2C) -> Result<u16, E> {
        let command = Command::new(Register::DATA1LOW)
            .enable_word_protocol().value();

        let mut buffer : [u8;2] = [0;2];
        i2c.write_read(self.address, &[command], &mut buffer)?;
        let result = ((buffer[1] as u16) << 8) | buffer[0] as u16;

        Ok(result)
    }

    /// Set the sensor integration time and gain
    /// These settings share a register in the instrument so setting them both requires only a single write
    /// Changes to these settings do not take effect until the next integration period
    pub fn config_time_gain(&self, i2c: &mut I2C, integration_time: IntegrationTime, gain: Gain)
        -> Result<(), E>
    {
        let command = Command::new(Register::TIMING).value();
        let setting = (*&integration_time as u8) | ((*&gain as u8) << 4);
        i2c.write(self.address, &[command, setting])
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

#[allow(dead_code)]
#[allow(non_camel_case_types)]
#[derive(Copy, Clone)]
/// Possible slave addresses
pub enum SlaveAddr {
    /// Default slave address
    ADDR_0x39 = 0x39,
    /// Optional slave address
    ADDR_0x29 = 0x29,
    /// Optional slave address
    ADDR_0x49 = 0x49,
}

impl Default for SlaveAddr {
    fn default() -> Self {
        SlaveAddr::ADDR_0x39
    }
}

impl SlaveAddr {
    /// Get slave address as u8
    pub fn addr(&self) -> u8 {
        *self as u8
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
#[derive(Copy, Clone)]
/// Available integration times
/// Lux calculations must take this setting into account
/// Triggering integration start and stop manually is an option of the device but not supported by this driver
pub enum IntegrationTime {
    /// 13.7 milliseconds
    ms_13 = 0,
    /// 101 milliseconds
    ms_101 = 1,
    /// 402 milliseconds (default)
    ms_402 = 2,
}


#[allow(dead_code)]
#[derive(Copy, Clone)]
/// Available sensor gain settings
/// Lux calculations must take this setting into account
pub enum Gain {
    /// Low gain - 1x (default)
    Low = 0,
    /// High gain - 16x
    High = 1,
}
