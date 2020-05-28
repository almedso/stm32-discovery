#![no_main]
#![no_std]


// Note when app is into powered down mode flashing does not work
// one needs to keep the reset button pressed until firmware download
// this download will fail (because it cannot reset) - but after releasing
// the reset button all is fine.

extern crate panic_halt;

use cortex_m_rt::entry;
use stm32l0xx_hal::{
    exti::{Exti, TriggerEdge, GpioLine, ExtiLine}, pac,
    prelude::*,
    pwr::{self, PWR},
    rcc::Config,
    syscfg::SYSCFG,
};

#[entry]
fn main() -> ! {
    let cp = pac::CorePeripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    let mut rcc = dp.RCC.freeze(Config::hsi16());
    let gpioa = dp.GPIOA.split(&mut rcc);
    let mut exti = Exti::new(dp.EXTI);
    let mut pwr = PWR::new(dp.PWR, &mut rcc);
    let mut delay = cp.SYST.delay(rcc.clocks);
    let mut scb   = cp.SCB;

    // Those are the user button and red LED
    // board.
    let button = gpioa.pa0.into_floating_input();
    let mut led = gpioa.pa5.into_push_pull_output();

    let mut syscfg = SYSCFG::new(dp.SYSCFG, &mut rcc);

    let line = GpioLine::from_raw_line(button.pin_number()).unwrap();

    exti.listen_gpio(
        &mut syscfg,
        button.port(),
        line,
        TriggerEdge::Falling,
    );

    loop {
        exti.wait_for_irq(
            line,
            pwr.stop_mode(
                &mut scb,
                &mut rcc,
                pwr::StopModeConfig {
                    ultra_low_power: true,
                },
            ),
        );

        led.set_high().unwrap();
        delay.delay_ms(100u32);
        led.set_low().unwrap();
    }
}
