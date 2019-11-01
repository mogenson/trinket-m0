#![no_std]
#![no_main]

// pick a panicking behavior
extern crate panic_halt; // you can put a breakpoint on `rust_begin_unwind` to catch panics

use cortex_m::asm;
use cortex_m_rt::entry;

// from: http://nercury.github.io/rust/embedded/experiments/2019/01/27/rust-embedded-02-measuring-the-clock.html
extern crate cty;
extern "C" {
    fn atmel_start_init();
    fn delay_ms(ms: cty::uint16_t);
    fn gpio_toggle_pin_level(pin: cty::uint8_t);
}

#[entry]
fn main() -> ! {
    let led: u8 = 10;
    asm::nop(); // To not have main optimize to abort in release mode, remove when you add code

    unsafe {
        atmel_start_init();
    }

    loop {
        unsafe {
            delay_ms(1000);
            gpio_toggle_pin_level(led);
        }
    }
}
