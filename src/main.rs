#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

use panic_halt as _;

use rtic::app;
#[allow(unused_imports)]

use stm32l0xx_hal::{
    exti::{TriggerEdge, Exti, GpioLine, ExtiLine},
    gpio::*,
    pac,
    prelude::*,
    rcc::Config,
    syscfg::SYSCFG,
    timer::Timer,
};


#[app(device = stm32l0xx_hal::pac, peripherals = true)]
const APP: () = {
    struct Resources {
        led: gpiob::PB4<Output<PushPull>>,
        exti: Exti,
        timer: Timer<pac::TIM2>,
    }

    #[init]
    fn init(ctx: init::Context) -> init::LateResources {
        let device = ctx.device;

        // Configure the clock.
        let mut rcc = device.RCC.freeze(Config::hsi16());

        // Acquire the GPIO peripherals.
        // This also enables the clock for GPIO in the RCC register.
        let gpioa = device.GPIOA.split(&mut rcc);
        let gpiob = device.GPIOB.split(&mut rcc);

        // Configure the green user LED.
        let mut led = gpiob.pb4.into_push_pull_output();
        led.set_high().unwrap();

        // Configure the user button as interrupt
        let button = gpioa.pa0.into_pull_up_input();
        let mut syscfg = SYSCFG::new(device.SYSCFG, &mut rcc);
        let mut exti = Exti::new(device.EXTI);

        // Configure the timer.
        let mut timer = device.TIM2.timer(1.hz(), &mut rcc);
        timer.listen();

        // Configure the external interrupt on the falling edge for the pin 0.
        let line = GpioLine::from_raw_line(button.pin_number()).unwrap();
        exti.listen_gpio(&mut syscfg, button.port(), line, TriggerEdge::Falling);

        // Return the initialized resources.
        init::LateResources { led, exti, timer }
    }


    #[idle]
    fn idle(_: idle::Context) -> ! {
        loop {}
    }

    #[task(binds=EXTI0_1, resources = [led, exti])]
    fn EXTI0_1(ctx: EXTI0_1::Context) {
        static mut STATE: bool = false;

        // Clear the interrupt flag.
        Exti::unpend(GpioLine::from_raw_line(0).unwrap());

        // Change the LED state on each interrupt.
        if *STATE {
            ctx.resources.led.set_low().unwrap();
            *STATE = false;
        } else {
            ctx.resources.led.set_high().unwrap();
            *STATE = true;
        }
    }

    #[task(binds = TIM2, resources = [led, timer])]
    fn TIM2(ctx: TIM2::Context) {
        static mut STATE: bool = false;
        static mut message: [ char: 80 ];

        // Clear the interrupt flag.
        ctx.resources.timer.clear_irq();

        // Change the LED state on each interrupt.
        if *STATE {
            ctx.resources.led.set_low().unwrap();
            *STATE = false;
        } else {
            ctx.resources.led.set_high().unwrap();
            *STATE = true;
        }
    }

};
