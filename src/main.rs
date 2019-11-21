#![no_std]
#![no_main]

extern crate panic_halt;
mod hal;
use cortex_m_rt::entry;
use embedded_hal::blocking::delay::DelayMs;
use embedded_hal::digital::v2::InputPin;
use embedded_hal::digital::v2::OutputPin;

#[entry]
fn main() -> ! {
    hal::init();
    let mut led = hal::Pin::new(hal::gpio_port_GPIO_PORTA, 10u8).into_output();
    let input = hal::Pin::from(11u8).into_pull_down_input();
    led.set_high().unwrap();

    loop {
        hal::Delay.delay_ms(1000u16);
        hal::Delay.delay_ms(input.is_high().unwrap() as u8);
    }
}
