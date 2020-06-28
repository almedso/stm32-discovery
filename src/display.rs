#![deny(unsafe_code)]
#![allow(warnings)]

#[cfg(debug_assertions)]
use cortex_m_semihosting::hprintln;

use epd_gde021a1::Error;
use stm32l0xx_hal::delay::Delay;
use embedded_hal::digital::v2::{InputPin, OutputPin};

use crate::board::{EpaperDevice, PowerPin};


/// the display encapsulates a graphic display and includes power management
///
/// Power management does not belong to the display driver, so it has to be
/// added and encapsulated somewhere. - Here is the perfect location.
pub struct Display<'e, 'pwr, 'dly> {
    pub epd: &'e mut EpaperDevice,
    power: &'pwr mut PowerPin,
    delay: &'dly mut Delay,
}

impl<'e, 'pwr, 'dly> Display<'e, 'pwr, 'dly> {

    pub fn new(
        epd: &'e mut EpaperDevice,
        power: &'pwr mut PowerPin,
        delay: &'dly mut Delay
    ) -> Display<'e, 'pwr, 'dly> {
        power.set_low().unwrap();
        epd.init(delay).unwrap();
        epd.clear();
        epd.refresh(delay).unwrap();
        Display { epd, power, delay }
    }

    /// Actually initialize the external device chip
    ///
    /// - power the chip on
    /// - run the init sequence (requires spi communication)
    pub fn power_on_and_init(&mut self)
        -> Result<(), Error<(), ()>>
    {
        // power on the on the display chip
        self.power.set_low().unwrap();

        // initialize the display make sure borrowing works by element separation
        // let &mut delay = self.delay;
        // let &mut epd = self.epd;
        // epd.init(&mut delay).unwrap();
        Ok (())
    }

    /// Refersh the display with corrent content from cache
    ///
    /// The call is forwarded to the display driver
    pub fn refresh(&mut self)
        -> Result<(), Error<(), ()>>
    {
    //     let mut delay = self.delay;
    //     let mut epd = self.epd;
    //     epd.refresh(&mut delay).unwrap();
        Ok (())
    }

    /// Power off the chip
    ///
    /// It means power off for ePaper display
    pub fn power_off(&mut self)
        -> Result<(), Error<(), ()>>
    {
        // power off the on the display chip
        self.power.set_high().unwrap();
        Ok (())
    }
}

