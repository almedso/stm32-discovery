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
    board_init,
    Board,
    Button,
    EpaperDevice,
    LedGreen,
    TimerInterrupt,
};
// use stm32l053c8t6_discovery::display::Display;

use embedded_graphics::drawable::Drawable;

use stm32l053c8t6_discovery::view::{
    SpotTemperatureWidget,
    ButtonWidget,
};

use debounced_pin::prelude::*;
use debounced_pin::ActiveHigh;
use epd_gde021a1::GDE021A1;

use embedded_graphics::{
    // drawable::Drawable,
    // egrectangle, egtext,
    // text_style,
    // fonts::{Font6x8, Text},
    geometry::Point,
    pixelcolor::{Gray2, },
    prelude::*,
    // primitives::Line,
    // style::{PrimitiveStyle, TextStyle},
    // primitive_style,
};


#[app(device = stm32l0xx_hal::pac, peripherals = true)]
const APP: () = {
    struct Resources {
        timer: TimerInterrupt,
        button: Button,
        led: LedGreen,
        epd: EpaperDevice,
        // display: Display<'e, 'pwr, 'dly>,
    }

    #[init(spawn = [toggle_led])]
    fn init(ctx: init::Context) -> init::LateResources {
        let device = ctx.device;
        hprintln!("** Init started").unwrap();
        let b = board_init(Some(device), None);

        hprintln!("** Init done").unwrap();
        let mut epd = GDE021A1::new(b.spi, b.reset, Some(b.chip_sel), b.data_cmd, b.busy);
        let mut power = b.power;
        power.set_low().unwrap();

        // initialize the display
        let mut delay = b.delay;
        epd.init(&mut delay).unwrap();

        hprintln!("** Display initialized").unwrap();

        // all pixels turn white
        epd.clear();

        // draw some fancy stuff
        // let button = ButtonWidget {
        //     top_left: Point::new(1,1),
        //     bottom_right: Point::new(100, 50),
        //     bg_color: Gray2::WHITE,
        //     fg_color: Gray2::BLACK,
        //     text: "Click me!",
        // };
        // let button = ButtonWidget::<Gray2>::new();
        // button.draw(&mut epd).unwrap();

        let mut temp_widget = SpotTemperatureWidget::<Gray2>::new(23);
        temp_widget.draw(&mut epd).unwrap();

        epd.refresh(&mut delay).unwrap();

        init::LateResources {
            timer: b.timer,
            button: b.button,
            led: b.led_green,
            epd,
//            epd: GDE021A1::new(b.spi, b.reset, Some(b.chip_sel), b.data_cmd, b.busy),
        }
    }


    #[idle]
    fn idle(_: idle::Context) -> ! {
        hprintln!("** Idle").unwrap();
        loop {}
    }

    #[task(binds = TIM2, resources = [button, timer, led, epd ], spawn = [toggle_led])]
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
                    hprintln!("** Button pressed").unwrap();
                    if *STATE {
                        ctx.resources.led.set_low().unwrap();
                    } else {
                        ctx.resources.led.set_high().unwrap();
                    }
                    *STATE = ! *STATE;
                }
                // let mut temp_widget = SpotTemperatureWidget::new(23);
                // temp_widget.draw(ctx.resources.epd).unwrap();

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
