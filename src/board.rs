#![deny(unsafe_code)]
#![allow(warnings)]

use stm32l0xx_hal::{
    delay::Delay,
    gpio::*,
    pac,
    prelude::*,
    rcc::Config,
    spi::*,
    timer::Timer,
};

use debounced_pin::prelude::*;
use debounced_pin::ActiveHigh;

use epd_gde021a1::{ GDE021A1, Error};

#[cfg(debug_assertions)]
use cortex_m_semihosting::hprintln;
use panic_semihosting as _;

pub type TimerInterrupt = Timer<pac::TIM2>;
pub type Button = DebouncedInputPin<gpioa::PA0<Input<PullUp>>, ActiveHigh>;
pub type LedGreen = gpiob::PB4<Output<PushPull>>;
pub type LedRed = gpioa::PA5<Output<PushPull>>;
pub type PowerPin = gpiob::PB10<Output<PushPull>>;
pub type ResetPin = gpiob::PB2<Output<PushPull>>;
pub type ChipSelectPin = gpioa::PA15<Output<PushPull>>;
pub type DataCommandPin = gpiob::PB11<Output<PushPull>>;
pub type BusyPin = gpiob::PB8<Input<PullUp>>;
pub type DisplaySpi = Spi<pac::SPI1,
            (gpiob::PB3<Analog>, NoMiso, gpiob::PB5<Analog> )>;

// board wired pins uninitialized so far
pub type UsbDmPin = gpioa::PA11<Analog>;
pub type UsbDPPin = gpioa::PA12<Analog>;
pub type Spi2 = Spi<pac::SPI2, (
    gpiob::PB13<Analog>,  // CLK
    gpiob::PB14<Analog>,  // Miso
    gpiob::PB15<Analog>,  // Mosi
)>;
pub type Spi2IrqIn = gpiob::PB6<Analog>;
pub type Spi2IrqOut = gpioa::PA15<Output<PushPull>>;
pub type LinearSensor1 = ( gpiob::PB0<Output<PushPull>>, // to sensor
     gpiob::PB1<Output<PushPull>> ); // to ground
pub type LinearSensor2 = ( gpioa::PA6<Output<PushPull>>, // to sensor
    gpioa::PA7<Output<PushPull>> ); // to ground
pub type LinearSensor3 = ( gpioa::PA2<Output<PushPull>>, // to sensor
    gpioa::PA3<Output<PushPull>> ); // to ground

pub type I2cSda = gpiob::PB7<Analog>;
pub type I2cScl = gpiob::PB6<Analog>;

pub type UsartRx = gpiob::PB10<Analog>;
pub type UsartTx = gpiob::PB11<Analog>;

pub type MfxWakeup = gpioa::PA0<Analog>;

// Extended types
pub type EpaperDevice = GDE021A1< DisplaySpi,
            ResetPin, ChipSelectPin, DataCommandPin, BusyPin>;


pub struct Board {
    pub timer: TimerInterrupt,
    pub button: Button,
    pub led_green: LedGreen,
    pub led_red: LedRed,
    pub spi: DisplaySpi,
    pub reset: ResetPin,
    pub chip_sel: ChipSelectPin,
    pub data_cmd: DataCommandPin,
    pub busy: gpiob::PB8<Input<PullUp>>,
    pub power: PowerPin,
    pub delay: Delay,
}

pub fn board_init(
    device: Option<pac::Peripherals>,
    mcu: Option<cortex_m::Peripherals>
) -> Board {

    #[cfg(debug_assertions)]
    hprintln!("** Start board config").unwrap();

    let device = match(device) {
        None => pac::Peripherals::take().unwrap(),
        Some(device)  => device,
    };
    let mcu = match(mcu) {
        None => cortex_m::Peripherals::take().unwrap(),
        Some(mcu) => mcu,
    };

    // Configure the clock.
    let mut rcc = device.RCC.freeze(Config::hsi16());

    // Configure the timer.
    let mut timer = device.TIM2.timer(1_000.hz(), &mut rcc);
    timer.listen();

    // Configure the button
    let gpioa = device.GPIOA.split(&mut rcc);
    let button = gpioa.pa0.into_pull_up_input();
    let button = DebouncedInputPin::new(button, ActiveHigh);

    // Configure the user LED's
    let gpiob = device.GPIOB.split(&mut rcc);
    let led_green = gpiob.pb4.into_push_pull_output();
    let led_red = gpioa.pa5.into_push_pull_output();

    // *** Start of the display section  ***
    // The GPIO's
    let chip_sel = gpioa.pa15.into_push_pull_output();
    let data_cmd = gpiob.pb11.into_push_pull_output();
    let reset = gpiob.pb2.into_push_pull_output();
    let busy = gpiob.pb8.into_pull_up_input();
    let mut power = gpiob.pb10.into_push_pull_output();

    // The SPI
    let mosi = gpiob.pb5;
    let clk = gpiob.pb3;

    // Alternative way of init
    // let spi = Spi::spi1(
    //     device.SPI1,
    //     (clk, NoMiso, mosi),
    //     MODE_0,
    //     1u32.mhz(),
    //     &mut rcc
    // );
    let spi = device.SPI1.spi((clk, NoMiso, mosi),
                            MODE_0, 1_000_000.hz(), &mut rcc);

    // the time delay
    let mut delay = mcu.SYST.delay(rcc.clocks);

    #[cfg(debug_assertions)]
    hprintln!("** Low level board configuration done").unwrap();

    Board {
        timer,
        button,
        led_green,
        led_red,
        spi,
        reset,
        chip_sel,
        data_cmd,
        busy,
        power,
        delay,
    }
}

