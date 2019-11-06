#![no_std]
#![no_main]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

extern crate panic_halt;

use cortex_m::asm;
use cortex_m_rt::entry;

extern crate cty;
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[entry]
fn main() -> ! {
    asm::nop();
    unsafe {
        atmel_start_init();
    }

    loop {
        unsafe {
            delay_ms(500);
            toggle_led();
        }
    }
}
