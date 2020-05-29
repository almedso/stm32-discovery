//! Board support crate for the STM32L0 Discovery
//!

#![deny(missing_docs)]
#![deny(warnings)]
#![no_std]

#[allow(unused_extern_crates)] // NOTE(allow) bug rust-lang/rust#53964
extern crate panic_halt; // panic handler

pub use cortex_m_rt::entry;

