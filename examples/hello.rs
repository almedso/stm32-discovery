//! Prints "Hello, world!" on the host console using semihosting
#![no_main]
#![no_std]

extern crate panic_semihosting;

use cortex_m_rt::entry;
use cortex_m_semihosting::{hprintln};

// include default NVIC
#[allow(unused_imports)]
use stm32l0xx_hal::{prelude::*};

#[entry]
fn main() -> ! {
    hprintln!("Hello, world!").unwrap();

    loop {}
}
