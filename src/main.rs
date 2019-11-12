#![no_std]
#![no_main]
//#![allow(non_upper_case_globals)]
//#![allow(non_camel_case_types)]
//#![allow(non_snake_case)]

extern crate panic_halt;

use cortex_m::asm;
use cortex_m_rt::entry;

// extern crate cty;
// include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

mod hal;
use embedded_hal::blocking::delay::DelayMs;

#[entry]
fn main() -> ! {
    let led: u8 = 10;
    asm::nop();

    hal::init(led);

    loop {
        hal::Delay.delay_ms(1000u16);
    }
}
