#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused)]
extern crate cty;
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub fn init(pin: u8) {
    unsafe {
        system_init();
        hal_gpio_set_pin_direction(pin, gpio_direction_GPIO_DIRECTION_OUT);
        hal_gpio_set_pin_level(pin, true);
    }
}

pub fn work(pin: u8) {
    unsafe {
        delay_ms(1000);
        hal_gpio_toggle_pin_level(pin);
    }
}

// implement Delay trait from embedded_hal
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
