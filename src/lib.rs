#![no_std]
#![feature(asm)]
#![feature(const_generics)]
#![feature(const_fn)]
#![feature(const_evaluatable_checked)]

pub mod aux;
pub mod gpio;
mod macros;
pub mod mailbox;
pub mod time;
pub mod uart;

#[cfg(feature = "raspberry-pi-4")]
const PERIPHERALS_BASE: usize = 0xFE00_0000;

#[cfg(feature = "raspberry-pi-3")]
const PERIPHERALS_BASE: usize = 0x3F00_0000;

#[inline(always)]
pub fn core_id() -> usize {
    let res: usize;
    unsafe {
        asm!(
            "mrs {0}, mpidr_el1",
            "and {0}, {0}, #0x3",
            out(reg) res,
        );
    }
    res
}

#[inline(always)]
pub fn execption_level() -> u8 {
    let res: u64;
    unsafe { asm!("mrs {}, CurrentEL", out(reg) res) }
    ((res >> 2) & 0b11) as u8
}

pub unsafe fn init() {
    uart::init_uart_0();
}

pub fn firmware_version() -> u32 {
    mailbox::Message::new()
        .with(mailbox::tag::GetFirmwareVersion)
        .commit()
        .unwrap()
}

pub fn memory() -> mailbox::tag_res::Ptr {
    mailbox::Message::new()
        .with(mailbox::tag::GetArmMemory)
        .commit()
        .unwrap()
}
