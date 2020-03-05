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
    asf::usb_init();

    let mut led = hal::Pin::from(hal::PIN_PA10 as u8).into_output();
    led.set_high().unwrap();

    let hello = "Hello, World\r\n"; // a statically allocated str type

    while !asf::cdcdf_acm_is_enabled() {} // wait for USB enumeration

    // asf::cdcd_acm_example(); // a C echo example that doesn't return

    loop {
        // casting a const pointer into a mutable pointer is *bad*
        // but we know it won't be modified, the C function is missing 'const'
        asf::cdcdf_acm_write(hello.as_ptr() as *mut u8, hello.len() as u32);

        hal::Delay.delay_ms(1000u16);
        for _ in 0..4 {
            led.toggle().unwrap();
            hal::Delay.delay_ms(100u16);
        }
    }
}
