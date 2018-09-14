#![no_main]
#![no_std]

#[macro_use]
extern crate cortex_m_rt;
#[macro_use]
extern crate nb;

extern crate nrf52840_mdk as device;
extern crate panic_abort;

use device::hal::{prelude::*, timer::Timer};
use device::nrf52840::Peripherals;
use device::Pins;

entry!(main);

fn main() -> ! {
    let p = Peripherals::take().unwrap();
    let pins = Pins::new(p.P0.split(), p.P1.split());

    let mut red_led = pins.red_led.into_push_pull_output();
    let mut blue_led = pins.blue_led.into_push_pull_output();
    let mut green_led = pins.green_led.into_push_pull_output();

    green_led.set_high();
    red_led.set_high();
    blue_led.set_high();

    let mut timer = p.TIMER0.constrain();

    // Alternately flash the red, green and blue leds
    loop {
        green_led.set_high();
        red_led.set_low();
        blue_led.set_high();
        delay(&mut timer, 250_000); // 250ms
        red_led.set_high();
        blue_led.set_low();
        delay(&mut timer, 1_000_000); // 1s
        green_led.set_low();
        blue_led.set_high();
        red_led.set_high();
        delay(&mut timer, 250_000); // 250ms
    }
}

fn delay<T>(timer: &mut Timer<T>, cycles: u32)
where
    T: TimerExt,
{
    timer.start(cycles);
    block!(timer.wait());
}
