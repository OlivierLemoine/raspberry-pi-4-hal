#![no_std]
#![feature(asm)]

pub mod aux;
pub mod gpio;
pub mod time;
pub mod uart;

#[cfg(feature = "raspberry-pi-4")]
const PERIPHERALS_BASE: usize = 0xFE00_0000;

#[cfg(feature = "raspberry-pi-3")]
const PERIPHERALS_BASE: usize = 0x3F00_0000;
