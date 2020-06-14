#![no_main]
#![no_std]


extern crate panic_halt;

use cortex_m_rt::entry;
use stm32l0xx_hal::{
    pac,
    gpio::*,
    prelude::*,
    spi::*,
    rcc::{Config,RccExt},
};

use epd_gde021a1;


extern crate embedded_graphics;
use embedded_graphics::{
    pixelcolor::BinaryColor,
    style::{PrimitiveStyle, TextStyle},
    primitives::Circle,
    fonts::{Font12x16, Font6x8, Text},
    prelude::*,
};

use epd_gde021a1::GDE021A1;

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
    let chip_sel = gpioa.pa15.into_push_pull_output();
    let data_cmd = gpiob.pb11.into_push_pull_output();
    let reset = gpiob.pb2.into_push_pull_output();
    let busy = gpiob.pb8.into_pull_up_input();
    let mut power = gpiob.pb10.into_push_pull_output();

    // The SPI
    let mosi = gpiob.pb5;
    let clk = gpiob.pb3;
    let spi = dp.SPI1.spi((clk, NoMiso, mosi),
                            MODE_0, 1_000_000.hz(), &mut rcc);

    // the time delay
    let mut delay = cp.SYST.delay(rcc.clocks);

    // and finally the display structure
    let mut disp =  GDE021A1::new(spi, reset, Some(chip_sel), data_cmd, busy);

    // power on the on the display chip
    power.set_low().unwrap();

    // initialize the display
    disp.init(&mut delay).expect("could not init display");
    // disp.alt_init(&mut delay).expect("could not init display");

    // all pixels turn white
    disp.clear();

    // draw some fancy stuff
    let elem =  Circle::new(Point::new(140, 36), 25)
         .into_styled(PrimitiveStyle::with_fill(BinaryColor::On));
    elem.draw(&mut disp);

    // Draw some text
    let elem = Text::new("Power minimized ", Point::new(1, 8))
        .into_styled(TextStyle::new(Font6x8, BinaryColor::On));
    elem.draw(&mut disp);
    let elem = Text::new("and safely", Point::new(1, 20))
        .into_styled(TextStyle::new(Font6x8, BinaryColor::On));
        elem.draw(&mut disp);
    let elem = Text::new("implemented in", Point::new(1, 32))
        .into_styled(TextStyle::new(Font6x8, BinaryColor::On));
    elem.draw(&mut disp);
    let elem = Text::new("Rust", Point::new(1, 44))
        .into_styled(TextStyle::new(Font12x16, BinaryColor::On));
    elem.draw(&mut disp);

    disp.refresh(&mut delay).expect("could not flush display");

    loop {
        continue;
    }
}
