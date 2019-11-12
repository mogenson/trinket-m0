#![no_std]
#![no_main]

extern crate panic_halt;
mod hal;
use cortex_m_rt::entry;
use embedded_hal::blocking::delay::DelayMs;

#[entry]
fn main() -> ! {
    let led: u8 = 10;
    hal::init(led);

    loop {
        hal::Delay.delay_ms(1000u16);
    }
}
