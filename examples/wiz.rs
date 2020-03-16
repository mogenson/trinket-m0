#![no_std]
#![no_main]

use apa102_spi::Apa102;
use heapless::{
    consts::U64,
    i::Queue as ConstQueue,
    spsc::{Producer, Queue},
    Vec,
};
use panic_halt as _;
use serde::Deserialize;
use smart_leds::{SmartLedsWrite, RGB8};
use trinket_m0 as hal;
use trinket_m0::bindings as asf;

#[derive(Deserialize)]
struct Params {
    r: u8,
    g: u8,
    b: u8,
}

#[allow(non_camel_case_types)]
#[derive(Deserialize, PartialEq)]
enum Method {
    setPilot,
}

#[derive(Deserialize)]
struct Message {
    method: Method,
    params: Params,
}

/* shared buffer to hold data received over USB */
static mut BUFFER: [u8; 64] = [0; 64];
static mut PRODUCER: Option<Producer<u8, U64, u8>> = None;

unsafe extern "C" fn read_callback(_ep: u8, _rc: asf::usb_xfer_code, count: u32) -> bool {
    if let Some(producer) = &mut PRODUCER {
        for i in 0..count {
            producer.enqueue(BUFFER[i as usize]).ok();
        }
    }
    asf::cdcdf_acm_read(BUFFER.as_mut_ptr(), BUFFER.len() as u32); // start next read
    false // no error
}

#[no_mangle]
pub unsafe extern "C" fn main() {
    asf::system_init();
    asf::usb_init();

    /* items to move data out of USB interrupt */
    let mut vec: Vec<u8, U64> = Vec::new();
    static mut Q: Queue<u8, U64, u8> = Queue(ConstQueue::u8());
    let (producer, mut consumer) = Q.split();
    PRODUCER = Some(producer);

    /* make spi instance and dotstar, disable postamble for one led */
    let spi = hal::Spi::new(&mut asf::SPI_0);
    let mut dotstar = Apa102::new_with_custom_postamble(spi, 0, false);

    /* wait for USB enumeration */
    while !asf::cdcdf_acm_is_enabled() {}

    /* Rust wants an Option<fn()> and C wants a void (*fn)(void) */
    asf::cdcdf_acm_register_callback(
        asf::cdcdf_acm_cb_type_CDCDF_ACM_CB_READ as u32,
        Some(core::mem::transmute(read_callback as usize)),
    );

    /* start first read */
    asf::cdcdf_acm_read(BUFFER.as_mut_ptr(), BUFFER.len() as u32);

    loop {
        while let Some(byte) = consumer.dequeue() {
            if byte == '\n' as u8 {
                if let Ok(message) = serde_json_core::from_slice::<Message>(&vec) {
                    if message.method == Method::setPilot {
                        let color = RGB8 {
                            r: message.params.r,
                            g: message.params.g,
                            b: message.params.b,
                        };
                        dotstar.write(core::iter::once(color)).ok();
                    }
                }
                vec.clear();
            } else {
                vec.push(byte).ok();
            }
        }
    }
}
