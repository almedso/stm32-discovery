#![deny(unsafe_code)]
#![no_main]
#![no_std]

extern crate panic_halt;

use cortex_m_rt::entry;
use stm32l0xx_hal::{pac, prelude::*, rcc::Config};

// LED3 (green) at PB4
// LED4 (red) at PA5

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    // Configure the clock.
    let mut rcc = dp.RCC.freeze(Config::hsi16());

    // Acquire the GPIOA peripheral. This also enables the clock for GPIOA in
    // the RCC register.
    let gpioa = dp.GPIOA.split(&mut rcc);
    // Acquire the GPIOA peripheral. This also enables the clock for GPIOA in
    // the RCC register.
    let gpiob = dp.GPIOB.split(&mut rcc);

    // Configure PA5 as output.
    let mut led4 = gpioa.pa5.into_push_pull_output();
    // Configure PB4 as output.
    let mut led5 = gpiob.pb4.into_push_pull_output();

    loop {
        // Set the LEDs 100k times in a row.
        for _ in 0..100_000 {
            led4.set_high().unwrap();
            led5.set_low().unwrap();
        }

        // Set the LEDs reverse 100k times in a row.
        for _ in 0..100_000 {
            led4.set_low().unwrap();
            led5.set_high().unwrap();
        }
    }
}
