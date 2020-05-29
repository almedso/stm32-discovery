#![deny(unsafe_code)]
#![no_main]
#![no_std]

extern crate panic_halt;

use cortex_m_rt::entry;
use stm32l0xx_hal::{pac, prelude::*, rcc::Config};
// use embedded_hal;
use epd_gde021a1;

/*
fn foo<T : embedded_hal::digital::v2::OutputPin>(mut l: T) {
    l.set_high();
}
*/
#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    // Configure the clock.
    let mut rcc = dp.RCC.freeze(Config::hsi16());

    // Acquire the GPIOx peripheral.
    // This also enables the clock for GPIOx in the RCC register.
    let gpioa = dp.GPIOA.split(&mut rcc);
    let gpiob = dp.GPIOB.split(&mut rcc);

    // The GPIO's
    let mut chip_sel = gpioa.pa15.into_push_pull_output();
    let mut dev_ctl = gpiob.pb11.into_push_pull_output();
    let mut reset = gpiob.pb2.into_push_pull_output();
    let mut busy = gpiob.pb8.into_pull_up_input();
    let mut power = gpiob.pb10.into_push_pull_output();

    // The SPI
    let mosi_pin = gpiob.pb5;
    let clk_pin = gpiob.pb3;
    let mut spi = dp.SPI1.spi(
        (clk_pin, stm32l0xx_hal::spi::NoMiso, mosi_pin),
        spi::MODE_0,
        1_000_000.hz(),
        &mut rcc
      );

    // the time delay
    let mut delay = cp.SYST.delay(rcc.clocks);

    // and finally the display structure
    let mut epd =  EpaperDisp {
        chip_sel, dev_ctl, reset, busy, power, spi, delay
    };

    epd.init();
    epd.clear(epd::Color.White);
    // epd.display_string_at_line(5,"foo");
    // epd.display_string_at_line(6,"bar");
    edp.refresh();
    loop {

    }
}