#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused)]
#![no_std]

pub use bindings::*;
pub mod bindings;

use core::convert::Infallible;

// implement digital traits from embedded_hal
use embedded_hal::digital::v2::{InputPin, OutputPin, ToggleableOutputPin};

pub struct Pin {
    pin: u8,
}

impl Pin {
    pub fn new(port: gpio_port, pin: u8) -> Self {
        Pin {
            pin: unsafe { pin_new(port, pin) },
        }
    }
    pub fn from(pin: u8) -> Self {
        Pin { pin: pin }
    }
    pub fn into_output(self) -> Self {
        unsafe {
            pin_into_output(self.pin);
        }
        self
    }
    pub fn into_input(self) -> Self {
        unsafe {
            pin_into_input(self.pin);
        }
        self
    }
    pub fn into_pull_down_input(self) -> Self {
        unsafe {
            pin_into_pull_down_input(self.pin);
        }
        self
    }
    pub fn into_pull_up_input(self) -> Self {
        unsafe {
            pin_into_pull_up_input(self.pin);
        }
        self
    }
}

impl OutputPin for Pin {
    type Error = Infallible;
    fn set_low(&mut self) -> Result<(), Self::Error> {
        unsafe {
            pin_set_low(self.pin);
        }
        Ok(())
    }
    fn set_high(&mut self) -> Result<(), Self::Error> {
        unsafe {
            pin_set_high(self.pin);
        }
        Ok(())
    }
}

impl InputPin for Pin {
    type Error = Infallible;
    fn is_low(&self) -> Result<bool, Self::Error> {
        Ok(unsafe { pin_is_low(self.pin) })
    }
    fn is_high(&self) -> Result<bool, Self::Error> {
        Ok(unsafe { pin_is_high(self.pin) })
    }
}

impl ToggleableOutputPin for Pin {
    type Error = Infallible;
    fn toggle(&mut self) -> Result<(), Self::Error> {
        unsafe {
            pin_toggle(self.pin);
        }
        Ok(())
    }
}

// implement delay traits from embedded_hal
// Note: <u32> unimplemented because atmel start hal uses uin16_t for delay time
use embedded_hal::blocking::delay::{DelayMs, DelayUs};

pub struct Delay; // empty struct

impl DelayMs<u16> for Delay {
    fn delay_ms(&mut self, ms: u16) {
        unsafe {
            delay_ms(ms);
        }
    }
}

impl DelayMs<u8> for Delay {
    fn delay_ms(&mut self, ms: u8) {
        self.delay_ms(ms as u16);
    }
}

impl DelayUs<u16> for Delay {
    fn delay_us(&mut self, us: u16) {
        unsafe {
            delay_us(us);
        }
    }
}

impl DelayUs<u8> for Delay {
    fn delay_us(&mut self, us: u8) {
        self.delay_us(us as u16);
    }
}

// implement blocking spi traits from embedded_hal
use embedded_hal::blocking::spi::Write;

pub struct Spi {
    spi: *mut bindings::spi_m_sync_descriptor,
    io: *mut bindings::io_descriptor,
}

impl Spi {
    pub fn new() -> Self {
        let spi = unsafe { &mut bindings::SPI_0 as *mut bindings::spi_m_sync_descriptor };
        let mut io = core::ptr::null_mut() as *mut bindings::io_descriptor;
        unsafe {
            bindings::spi_m_sync_get_io_descriptor(
                spi,
                &mut io as *mut *mut bindings::io_descriptor,
            );
            bindings::spi_m_sync_enable(spi);
        }

        Spi { spi: spi, io: io }
    }
}

impl Write<u8> for Spi {
    type Error = Infallible;

    fn write(&mut self, buffer: &[u8]) -> Result<(), Self::Error> {
        unsafe {
            bindings::io_write(self.io, buffer.as_ptr(), buffer.len() as u16);
        }
        Ok(())
    }
}
