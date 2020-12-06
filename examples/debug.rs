#![no_main]
#![no_std]

use cortex_m_rt::entry;
use panic_probe as _;

use nrf52840_mdk::hal::prelude::*;
use rtt_target::{rprintln, rtt_init_print};

#[entry]
fn main() -> ! {
    // allocate the rtt machinery for printing
    // rtt_init_print!(); //non blocking but 1024 buffer and lose data if full
    rtt_init_print!(BlockIfFull, 128); //NOTE! will block if not hooked up to debugger

    rprintln!("this is what debugging looks like");

    loop {
        panic!("This is what a panic looks like")
    }
}
