#![no_std]
#![no_main]

use core::mem::transmute;
use embedded_hal::blocking::delay::DelayMs;
use embedded_hal::digital::v2::{OutputPin, ToggleableOutputPin};
use panic_halt as _;
use trinket_m0 as hal;
use trinket_m0::bindings as asf;

/* USB callback types */
#[repr(u32)]
enum CbType {
    Read = asf::cdcdf_acm_cb_type_CDCDF_ACM_CB_READ,
    Write = asf::cdcdf_acm_cb_type_CDCDF_ACM_CB_WRITE,
    StateChange = asf::cdcdf_acm_cb_type_CDCDF_ACM_CB_STATE_C,
}

/* shared buffer to hold data received over USB */
static mut BUFFER: [u8; 64] = [0; 64];

unsafe extern "C" fn read_callback(_ep: u8, _rc: asf::usb_xfer_code, count: u32) -> bool {
    for i in 0..count {
        BUFFER[i as usize].make_ascii_uppercase(); // convert ascii chars to uppercase
    }
    asf::cdcdf_acm_write(BUFFER.as_mut_ptr(), count); // write buffer back
    false // no error
}

unsafe extern "C" fn write_callback(_ep: u8, _rc: asf::usb_xfer_code, _count: u32) -> bool {
    asf::cdcdf_acm_read(BUFFER.as_mut_ptr(), BUFFER.len() as u32); // do next read
    false // no error
}

unsafe extern "C" fn state_change_callback(state: asf::usb_cdc_control_signal_t) -> bool {
    /* this ugly type is a C struct with unions of inner structs */
    if state.__bindgen_anon_1.rs232.DTR() == 1 {
        asf::cdcdf_acm_register_callback(
            CbType::Read as u32,
            Some(transmute(read_callback as usize)),
        );
        asf::cdcdf_acm_register_callback(
            CbType::Write as u32,
            Some(transmute(write_callback as usize)),
        );
        asf::cdcdf_acm_read(BUFFER.as_mut_ptr(), BUFFER.len() as u32);
    }

    false // no error
}

#[no_mangle]
pub unsafe extern "C" fn main() {
    asf::system_init();
    asf::usb_init();

    let mut led = hal::Pin::from(hal::PIN_PA10 as u8).into_output();
    led.set_high().unwrap();

    /* wait for USB enumeration */
    while !asf::cdcdf_acm_is_enabled() {}

    /* Rust wants an Option<fn()> and C wants a void (*fn)(void) */
    asf::cdcdf_acm_register_callback(
        CbType::StateChange as u32,
        Some(transmute(state_change_callback as usize)),
    );

    /* do blinky */
    loop {
        hal::Delay.delay_ms(1000u16);
        for _ in 0..4 {
            led.toggle().unwrap();
            hal::Delay.delay_ms(100u16);
        }
    }
}
