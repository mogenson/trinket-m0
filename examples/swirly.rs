#![no_std]
#![no_main]

use embedded_hal::blocking::delay::DelayMs;
use embedded_hal::blocking::spi::Write;
use panic_halt as _;
use trinket_m0 as hal;
use trinket_m0::bindings as asf;

#[no_mangle]
pub unsafe extern "C" fn main() {
    asf::system_init();

    // half brightness
    let mut buf: [u8; 8] = [0, 0, 0, 0, 0xEA, 0, 0, 0];
    let mut spi = hal::Spi::new();

    loop {
        for i in 5..8 {
            buf[5] = 0;
            buf[6] = 0;
            buf[7] = 0;
            buf[i] = 255;
            hal::Delay.delay_ms(1000u16);
            spi.write(&buf).ok();
        }
    }
}
