#![no_std]
#![no_main]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

extern crate cty;
extern crate panic_halt;
use cortex_m::asm;
use cortex_m_rt::entry;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[entry]
fn main() -> ! {
    let led: u8 = 10;

    unsafe {
        atmel_start_init();
    }

    loop {
        asm::nop(); // To not have main optimize to abort in release mode, remove when you add code
        // unsafe {
        //     delay_ms(1000);
        //     gpio_toggle_pin_level(led);
        // }
    }
}
