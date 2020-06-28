#![no_main]
#![no_std]


use panic_semihosting as _;
use cortex_m_semihosting::hprintln;

use cortex_m_rt::entry;
use embedded_hal::digital::v2::OutputPin;
use epd_gde021a1::GDE021A1;
use stm32l053c8t6_discovery::board::board_init;

extern crate embedded_graphics;
use embedded_graphics::{
    pixelcolor::BinaryColor,
    style::{PrimitiveStyle, TextStyle},
    primitives::Circle,
    fonts::{Font6x8, Text},
    prelude::*,
};


#[entry]
fn main() -> ! {
    hprintln!("** Started").unwrap();

    // let device = pac::Peripherals::take().unwrap();
    let b = board_init(None, None);

    hprintln!("** Board init done").unwrap();

    let mut disp =  GDE021A1::new(b.spi, b.reset, Some(b.chip_sel), b.data_cmd, b.busy);

    hprintln!("** Display created").unwrap();

    let mut delay = b.delay;

    // init_chip(&display).expect("could not init display");
    let mut power = b.power;
    power.set_low().unwrap();

    // initialize the display
    disp.init(&mut delay).unwrap();

    hprintln!("** Display initialized").unwrap();

    // all pixels turn white
    disp.clear();

    // draw some fancy stuff
    let elem =  Circle::new(Point::new(140, 36), 25)
         .into_styled(PrimitiveStyle::with_fill(BinaryColor::On));
    elem.draw(&mut disp).unwrap();

    // Draw some text
    let elem = Text::new("Power minimized ", Point::new(1, 8))
        .into_styled(TextStyle::new(Font6x8, BinaryColor::On));
    elem.draw(&mut disp).unwrap();
    let elem = Text::new("and safely", Point::new(1, 20))
        .into_styled(TextStyle::new(Font6x8, BinaryColor::On));
        elem.draw(&mut disp).unwrap();
    let elem = Text::new("implemented in", Point::new(1, 32))
        .into_styled(TextStyle::new(Font6x8, BinaryColor::On));
    elem.draw(&mut disp).unwrap();
    let elem = Text::new("Rust", Point::new(1, 44))
        .into_styled(TextStyle::new(Font6x8, BinaryColor::On));
    elem.draw(&mut disp).unwrap();

    hprintln!("** Refresh display").unwrap();

    disp.refresh(&mut delay).unwrap();

    hprintln!("** Enter loop").unwrap();

    loop {
        continue;
    }
}
