#![no_std]
#![no_main]

extern crate panic_halt;
extern crate trinket_m0 as hal;
use embedded_hal::blocking::delay::DelayMs;
use embedded_hal::digital::v2::{OutputPin, ToggleableOutputPin};
use hal::bindings;

#[no_mangle]
pub unsafe extern "C" fn main() {
    hal::init();
    bindings::usb_init();

    let mut led = hal::Pin::from(hal::PIN_PA10 as u8).into_output();
    led.set_low().unwrap();

    bindings::cdcd_acm_example(); // doesn't return

    loop {
        hal::Delay.delay_ms(1000u16);
        for _ in 0..4 {
            led.toggle().unwrap();
            hal::Delay.delay_ms(100u16);
        }
    }
}
