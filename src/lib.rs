//! Board support crate for the STM32L0 Discovery
//!
//! # Usage
//!
//! - Trying out the examples
//!
//! ``` text
//! # on another terminal
//! $ openocd -f interface/stlink-v2-1.cfg -f target/stm32l0.cfg
//!
//! # flash and debug the "Hello, world" example
//! $ rustup target add thumbv6em-none-eabi
//! $ cargo run --example hello
//! ```
//!
//! You'll need to have both OpenOCD and arm-none-eabi-gcc installed.
//!
//! - Building an application that depends on this crate
//!
//! # Examples
//!
//! See the [examples] module.
//!
//! [examples]: examples/index.html


#![deny(missing_docs)]
#![deny(warnings)]
#![no_std]
/*
pub extern crate stm32l0xx_hal as hal;

use hal::gpio::gpiob::PB13;  // green user led
use hal::gpio::gpioc::PE3;  // user button
use hal::gpio::{Output, PushPull};
*/

