#![no_std]
#![no_main]

use embedded_hal::blocking::delay::DelayMs;
use embedded_hal::digital::v2::{OutputPin, ToggleableOutputPin};
use panic_halt as _;
use trinket_m0 as hal;
use trinket_m0::bindings as asf;

#[no_mangle]
pub unsafe extern "C" fn main() {
    asf::system_init();
    let mut led = hal::Pin::from(hal::PIN_PA10 as u8).into_output();
    led.set_low().unwrap();

    loop {
        hal::Delay.delay_ms(1000u16);
        for _ in 0..4 {
            led.toggle().unwrap();
            hal::Delay.delay_ms(100u16);
        }
    }
}
