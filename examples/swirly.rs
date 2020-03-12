#![no_std]
#![no_main]

use embedded_hal::blocking::delay::DelayMs;
use panic_halt as _;
use trinket_m0 as hal;
use trinket_m0::bindings as asf;

#[no_mangle]
pub unsafe extern "C" fn main() {
    asf::system_init();

    // half brightness
    let mut buf: [u8; 8] = [0, 0, 0, 0, 0xEA, 0, 0, 0];
    let mut io: *mut asf::io_descriptor = core::ptr::null_mut();

    asf::spi_m_sync_get_io_descriptor(
        &mut asf::SPI_0 as *mut asf::spi_m_sync_descriptor,
        &mut io as *mut *mut asf::io_descriptor,
    );

    asf::spi_m_sync_enable(&mut asf::SPI_0 as *mut asf::spi_m_sync_descriptor);

    loop {
        for i in 5..8 {
            buf[5] = 0;
            buf[6] = 0;
            buf[7] = 0;
            buf[i] = 255;
            hal::Delay.delay_ms(1000u16);
            asf::io_write(io, buf.as_ptr(), buf.len() as u16);
        }
    }
}
