#![deny(unsafe_code)]
#![allow(warnings)]

use stm32l0xx_hal::{
    delay::Delay,
    gpio::*,
    pac,
    prelude::*,
    rcc::Config,
    timer::Timer,
};

use debounced_pin::prelude::*;
use debounced_pin::ActiveHigh;

pub type TimerInterrupt = Timer<pac::TIM2>;
pub type Button = DebouncedInputPin<gpioa::PA0<Input<PullUp>>, ActiveHigh>;
pub type Led = gpiob::PB4<Output<PushPull>>;

pub struct Board {
    // pub morse: my_morse::MorseDevice,
    pub delay: Delay,
    pub timer: TimerInterrupt,
    pub button: Button,
    pub led: Led,
}

pub fn board_init(device: pac::Peripherals) -> Board {
    // let device = pac::Peripherals::take().unwrap();
    let mcu = cortex_m::Peripherals::take().unwrap();


    // Configure the clock.
    let mut rcc = device.RCC.freeze(Config::hsi16());

    // Configure the timer.
    let mut timer = device.TIM2.timer(1_000.hz(), &mut rcc);
    timer.listen();

    // the time delay
    let delay = mcu.SYST.delay(rcc.clocks);

    // Configure the button
    let gpioa = device.GPIOA.split(&mut rcc);
    let button = gpioa.pa0.into_pull_up_input();
    let button = DebouncedInputPin::new(button, ActiveHigh);

    // Configure the user LED
    let gpiob = device.GPIOB.split(&mut rcc);
    let led = gpiob.pb4.into_push_pull_output();

    Board { delay, timer, button , led }
}


/*

mod display {

    use epd_gde021a1::{ GDE021A1, Error};

    use stm32l0xx_hal::{
        delay::Delay,
        exti::{TriggerEdge, Exti, GpioLine, ExtiLine},
        gpio::*,
        pac,
        prelude::*,
        rcc::Config,
        spi::*,
        syscfg::SYSCFG,
        timer::Timer,
    };
    use stm32l0::stm32l0x3::{SPI1, PINS};


    pub type EpaperDevice = GDE021A1<
        //Spi<stm32l0xx_hal::spi::Spi<stm32l0::stm32l0x3::SPI1, PINS>>,
        //Spi<stm32l0::stm32l0x3::SPI1,( gpiob::PB3<Analog>, NoMiso, gpiob::PB3<Analog>)>,
        Spi<SPI1, Pins( gpiob::PB3<Output<PushPull>>, NoMiso, gpiob::PB3<Output<PushPull>>)>,
        //Spi<SPI1, PINS>,
        // Spi<
        //     spi,
        //     Pins<
        //         gpiob::PB3<spi>, NoMiso, gpiob::PB5<spi>
        //     >
        // >,
        gpiob::PB2<Output<PushPull>>, // reset
        gpioa::PA15<Output<PushPull>>, // chip-sel (some)
        gpiob::PB11<Output<PushPull>>, // data_cmd
        gpiob::PB8<Input<PullUp>>, // busy
    >;
    pub type PowerPin = gpiob::PB10<Output<PushPull>>;

    pub struct Display {
        pub epd: EpaperDevice,
        pub power: PowerPin,
        pub delay: Delay,
    }

    /// Configure all pins, timers and spi
    ///
    /// Do not communicate with the display chip yet
    pub fn init(device: pac::Peripherals, mcu: cortex_m::Peripherals) -> Display {

        // Configure the clock. - maybe this is unique resource already
        let mut rcc = device.RCC.freeze(Config::hsi16());

        let gpioa = device.GPIOA.split(&mut rcc);
        let gpiob = device.GPIOB.split(&mut rcc);

        // The GPIO's
        let chip_sel = gpioa.pa15.into_push_pull_output();
        let data_cmd = gpiob.pb11.into_push_pull_output();
        let reset = gpiob.pb2.into_push_pull_output();
        let busy = gpiob.pb8.into_pull_up_input();
        let power = gpiob.pb10.into_push_pull_output();

        // The SPI
        let mosi = gpiob.pb5;
        let clk = gpiob.pb3;
        let spi = device.SPI1.spi((clk, NoMiso, mosi),
                                MODE_0, 1_000_000.hz(), &mut rcc);

        // the time delay
        let delay = mcu.SYST.delay(rcc.clocks);

        // and finally the display structure
        let epd =  GDE021A1::new(spi, reset, Some(chip_sel), data_cmd, busy);

        // return the display structure
        Display { epd, power, delay}
    }

    /// Actually initialize the external device chip
    ///
    /// - power the chip on
    /// - run the init sequence (requires spi communication)
    pub fn init_chip(display: &mut Display)
        -> Result<(), Error<(), ()>>
    {
        // power on the on the display chip
        display.power.set_low().unwrap();
        // initialize the display
        display.epd.init(&mut display.delay).unwrap();
        Ok (())
    }

    /// Deinit the chip
    ///
    /// It means power off for ePaper display
    pub fn deinit_chip(display: &mut Display)
        -> Result<(), Error<(), ()>>
    {
        // power off the on the display chip
        display.power.set_high().unwrap();
        Ok (())
    }
}
*/