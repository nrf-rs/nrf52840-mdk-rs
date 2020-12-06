#![no_main]
#![no_std]

extern crate panic_halt;
use cortex_m_rt::entry;

use nrf52840_mdk::hal::gpio::{p0, p1, Level};
use nrf52840_mdk::hal::prelude::*;
use nrf52840_mdk::hal::timer::Timer;
use nrf52840_mdk::pac::Peripherals;
use nrf52840_mdk::Pins;

#[entry]
fn main() -> ! {
    let p = Peripherals::take().unwrap();
    let pins = Pins::new(p0::Parts::new(p.P0), p1::Parts::new(p.P1));

    let mut red_led = pins.red_led.into_push_pull_output(Level::Low);
    let mut blue_led = pins.blue_led.into_push_pull_output(Level::Low);
    let mut green_led = pins.green_led.into_push_pull_output(Level::Low);

    let _ = green_led.set_high();
    let _ = red_led.set_high();
    let _ = blue_led.set_high();

    let mut timer = Timer::new(p.TIMER0);

    // Alternately flash the red, green and blue leds
    loop {
        let _ = green_led.set_high();
        let _ = red_led.set_low();
        let _ = blue_led.set_high();
        timer.delay(250_000); // 250ms
        let _ = red_led.set_high();
        let _ = blue_led.set_low();
        timer.delay(1_000_000); // 1s
        let _ = green_led.set_low();
        let _ = blue_led.set_high();
        let _ = red_led.set_high();
        timer.delay(250_000); // 250ms
    }
}
