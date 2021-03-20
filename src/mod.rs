pub mod aux;
pub mod gpio;
pub mod mailbox;
// pub mod sd;
pub mod time;
pub mod uart;

#[cfg(feature = "RaspberryPi4")]
const PERIPHERALS_BASE: usize = 0xFE00_0000;

#[cfg(feature = "RaspberryPi3")]
const PERIPHERALS_BASE: usize = 0x3F00_0000;
