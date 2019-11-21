#![no_std]
#![no_main]

extern crate panic_halt;
mod hal;
use cortex_m_rt::entry;
use embedded_hal::blocking::delay::DelayMs;
use embedded_hal::digital::v2::OutputPin;

#[entry]
fn main() -> ! {
    hal::init();
    let mut led = hal::Pin::from(hal::PIN_PA10 as u8).into_output();

    loop {
        hal::Delay.delay_ms(1000u16);
        led.set_high().unwrap();
        hal::Delay.delay_ms(500u16);
        led.set_low().unwrap();
    }
}
