#![no_std]

#[cfg(feature = "raspberry-pi-4")]
const PERIPHERALS_BASE: usize = 0xFE00_0000;

#[cfg(feature = "raspberry-pi-3")]
const PERIPHERALS_BASE: usize = 0x3F00_0000;
