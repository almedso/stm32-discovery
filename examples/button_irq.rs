#![no_main]
#![no_std]

extern crate panic_halt;

use core::cell::RefCell;
use core::ops::DerefMut;

use cortex_m::asm;
use cortex_m::interrupt::Mutex;
use cortex_m::peripheral::NVIC;
use cortex_m_rt::entry;
use stm32l0xx_hal::{
    exti::{TriggerEdge, GpioLine, ExtiLine, Exti},
    gpio::*,
    pac::{self, interrupt, Interrupt},
    prelude::*,
    rcc::Config,
    syscfg::SYSCFG,
};

// USER button (B1) is on pin PA0
// LED(3 green) on pin PB4

static LED: Mutex<RefCell<Option<gpiob::PB4<Output<PushPull>>>>> = Mutex::new(RefCell::new(None));

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    // Configure the clock.
    let mut rcc = dp.RCC.freeze(Config::hsi16());

    // Acquire the GPIOB peripheral. This also enables the clock for GPIOB in
    // the RCC register.
    let gpiob = dp.GPIOB.split(&mut rcc);

    // Configure PB4 as output.
    let led = gpiob.pb4.into_push_pull_output();

    // switch led on to show that there is action
    // led.set_high().unwrap();

    // Acquire the GPIOA peripheral. This also enables the clock for GPIOA in
    // the RCC register.
    let gpioa = dp.GPIOA.split(&mut rcc);

    // Configure PA0 as input.
    let button = gpioa.pa0.into_pull_up_input();

    let mut syscfg = SYSCFG::new(dp.SYSCFG, &mut rcc);
    let mut exti = Exti::new(dp.EXTI);

    // Configure the external interrupt on the falling edge for the pin 0.
    let line = GpioLine::from_raw_line(button.pin_number()).unwrap();
    exti.listen_gpio(&mut syscfg, button.port(), line, TriggerEdge::Falling);

    // Store the external interrupt and LED in mutex reffcells to make them
    // available from the interrupt.
    cortex_m::interrupt::free(|cs| {
        *LED.borrow(cs).borrow_mut() = Some(led);
    });

    // Enable the external interrupt in the NVIC.
    unsafe { NVIC::unmask(Interrupt::EXTI2_3); }

    loop {
        asm::wfi();
    }
}

#[interrupt]
fn EXTI2_3() {
    // Keep the LED state.
    static mut STATE: bool = false;

    cortex_m::interrupt::free(|cs| {
        // Clear the interrupt flag.
        Exti::unpend(GpioLine::from_raw_line(2).unwrap());

        // Change the LED state on each interrupt.
        if let Some(ref mut led) = LED.borrow(cs).borrow_mut().deref_mut() {
            if *STATE {
                led.set_low().unwrap();
                *STATE = false;
            } else {
                led.set_high().unwrap();
                *STATE = true;
            }
        }
    });
}
