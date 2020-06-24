#![deny(unsafe_code)]
#![allow(warnings)]
#![no_main]
#![no_std]

// use panic_halt as _;
use panic_semihosting as _;
use cortex_m_semihosting::hprintln;

use rtic::app;

use embedded_hal::digital::v2::{InputPin, OutputPin};

use stm32l053c8t6_discovery::board::{
    Board,
    TimerInterrupt,
    Button,
    Led,
    board_init,
};

use debounced_pin::prelude::*;
use debounced_pin::ActiveHigh;


// #[app(device = stm32l0::stm32l0x3, peripherals = true)]
#[app(device = stm32l0xx_hal::pac, peripherals = true)]
const APP: () = {
    struct Resources {
        // morse: MorseDevice,
        timer: TimerInterrupt,
        button: Button,
        led: Led
    }

    #[init(spawn = [toggle_led])]
    fn init(ctx: init::Context) -> init::LateResources {
        let device = ctx.device;
        let b = board_init(device);
        hprintln!("init").unwrap();

        init::LateResources { timer: b.timer, button: b.button, led: b.led }
    }


    #[idle]
    fn idle(_: idle::Context) -> ! {
        hprintln!("idle").unwrap();
        loop {}
    }

    #[task(binds = TIM2, resources = [button, timer, led ], spawn = [toggle_led])]
    fn TIM2(ctx: TIM2::Context) {
        static mut STATE: bool = false;
        static mut LAST_STATE: bool = false;
        match ctx.resources.button.update().unwrap() {
            // Pin is not active.
            DebounceState::NotActive => *LAST_STATE = false,
            // Pin was reset or is not active in general.
            DebounceState::Reset => return,
            // Pin is active but still debouncing.
            DebounceState::Debouncing => return,
            // Pin is active and debounced.
            DebounceState::Active => {
                if ! *LAST_STATE {
                    *LAST_STATE = true;
                    hprintln!("Button pressed").unwrap();
                    if *STATE {
                        ctx.resources.led.set_low().unwrap();
                    } else {
                        ctx.resources.led.set_high().unwrap();
                    }
                    *STATE = ! *STATE;
                }
            },
        }
    }

    #[task(resources = [led])]
    fn toggle_led(ctx: toggle_led::Context) {
        static mut STATE: bool = false;
        hprintln!("Toggle LED").unwrap();
        if *STATE {
            ctx.resources.led.set_low().unwrap();
        } else {
            ctx.resources.led.set_high().unwrap();
        }
        *STATE = ! *STATE;
    }

    // Interrupt handlers used to dispatch software tasks
    extern "C" {
        fn USART1();
        // fn LCD();
    }

};
