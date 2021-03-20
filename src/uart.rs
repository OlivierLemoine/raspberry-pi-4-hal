use core::fmt::Write;

use super::{
    aux::aux,
    gpio::{Function, GPIOTuple, Resistor, GPIO},
};

pub unsafe fn init_uart_0() {
    #[cfg(feature = "raspberry-pi-4")]
    const BAUD_RATE: u32 = 542; // Base clock = 500MHz
    #[cfg(feature = "raspberry-pi-3")]
    const BAUD_RATE: u32 = 270; // Base clock = 250MHz

    (GPIO(14), GPIO(15))
        .set_function(Function::Alternate5)
        .set_pullup_pulldown(Resistor::No);

    aux().ENABLES.write(1);
    aux().MU.control.write(0);
    aux().MU.IER.write(0);
    aux().MU.IIR.write(0b1100_0110);
    aux().MU.LCR.write(0b11);
    aux().MU.MCR.write(0);
    aux().MU.baud_rate.write(BAUD_RATE);

    aux().MU.control.write(0b11);

    Uart1.write_u8_blocking('\n' as u8);
    Uart1.write_u8_blocking('\r' as u8);
}

pub struct Uart1;
impl Uart1 {
    pub fn write_str_blocking(&mut self, s: &str) {
        for c in s.chars() {
            self.write_u8_blocking(c as u8)
        }
    }
    pub fn write_u8_blocking(&mut self, c: u8) {
        loop {
            if unsafe { aux().MU.status.get(1) } {
                break;
            }
        }
        unsafe { aux().MU.IO.write(c as u32) };
    }
    pub fn try_read_u8(&self) -> Option<u8> {
        if unsafe { aux().MU.status.get(0) } {
            Some(unsafe { aux().MU.IO.read() & 0xFF } as u8)
        } else {
            None
        }
    }
    #[inline]
    pub fn try_read_char(&self) -> Option<char> {
        self.try_read_u8().map(|v| v as char)
    }
}

impl core::fmt::Write for Uart1 {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.write_str_blocking(s);
        Ok(())
    }
}

#[doc(hidden)]
pub fn _print_internals(args: core::fmt::Arguments) {
    Uart1.write_fmt(args).unwrap();
}
