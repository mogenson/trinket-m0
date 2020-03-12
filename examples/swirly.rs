#![no_std]
#![no_main]

use apa102_spi::Apa102;
use embedded_hal::blocking::delay::DelayMs;
use panic_halt as _;
use smart_leds::hsv::{hsv2rgb, Hsv};
use smart_leds::SmartLedsWrite;
use trinket_m0 as hal;
use trinket_m0::bindings as asf;

#[no_mangle]
pub unsafe extern "C" fn main() {
    asf::system_init();

    /* make spi instance and dotstar, disable postamble for one led */
    let spi = hal::Spi::new(&mut asf::SPI_0);
    let mut dotstar = Apa102::new_with_custom_postamble(spi, 0, false);

    loop {
        /* cycle through all hue values */
        for i in 0..=255 {
            let hsv = Hsv {
                hue: i,
                sat: 255,
                val: 255,
            };
            let rgb = hsv2rgb(hsv);
            let color = core::iter::once(rgb);
            dotstar.write(color).ok();
            hal::Delay.delay_ms(25u16);
        }
    }
}
