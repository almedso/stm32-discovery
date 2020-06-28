/// Application specific view or widgets
///
#[macro_use]

// #![allow(warnings)]
use embedded_graphics::{
    drawable::Drawable,
    egrectangle, egtext,
    text_style,
    fonts::{Font6x8, Text},
    geometry::Point,
    pixelcolor::{BinaryColor, Gray2, PixelColor, },
    prelude::*,
    primitives::Line,
    style::{PrimitiveStyle, TextStyle},
    primitive_style,
};

use core::str::from_utf8_mut;
use numtoa::NumToA;
use core::marker::PhantomData;


pub struct SpotTemperatureWidget<C: PixelColor> {
    top_left: Point,
    temperature: isize,
    phantom: PhantomData<C>,  // To cover the PixelColor type
}

impl<C>  SpotTemperatureWidget<C>
where C : PixelColor
{
    pub fn new (temperature: isize) -> SpotTemperatureWidget<C> {
        SpotTemperatureWidget {
            top_left: Point::new(1,1),
            temperature,
            phantom: PhantomData,
        }
    }
}

impl<C> Drawable<C> for &SpotTemperatureWidget<C>
where C: PixelColor + From<Gray2>
{
    fn draw<D: DrawTarget<C>>(self, display: &mut D) -> Result<(), D::Error> {
        const TEXT_COLOR: Gray2 = Gray2::BLACK;
        let mut buf = [0u8; 4];
        self.temperature.numtoa_str(10, &mut buf);
        let text : &str = core::str::from_utf8_mut(&mut buf).unwrap();
        egtext!(
            text = text,
            top_left = self.top_left,
            style = text_style!(font = Font6x8, text_color = TEXT_COLOR.into())
        )
        .draw(display)?;
        // egtext!(
        //     text = self.unit,
        //     top_left = self.top_left + Point::new(0, 24),
        //     style = text_style!(font = Font6x8, text_color = self.fg_color)
        // )
        // .draw(display)?;
        Ok (())
    }

}

pub struct ButtonWidget<'a, C: PixelColor> {
    pub top_left: Point,
    pub bottom_right: Point,
    pub bg_color: C,
    pub fg_color: C,
    pub text: &'a str,
}

impl<'a, C: 'a> Drawable<C> for &ButtonWidget<'a, C>
where
    C: PixelColor + From<BinaryColor>,
{
    fn draw<D: DrawTarget<C>>(self, display: &mut D) -> Result<(), D::Error> {
        egrectangle!(
            top_left = self.top_left,
            bottom_right = self.bottom_right,
            style = primitive_style!(fill_color = self.fg_color)
        )
        .draw(display)?;
        egtext!(
            text = self.text,
            top_left = (20, 20),
            style = text_style!(font = Font6x8, text_color = self.bg_color)
        )
        .draw(display)
    }
}

impl<'a, C> ButtonWidget<'a, C>
where C : PixelColor + From<Gray2>
{

    pub fn new() -> ButtonWidget<'a, C> {
        ButtonWidget {
            top_left: Point::new(1,1),
            bottom_right: Point::new(100, 50),
            bg_color: Gray2::WHITE.into(),
            fg_color: Gray2::BLACK.into(),
            text: "Click me!",
        }
    }
}
