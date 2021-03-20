use register::*;

const AUX_BASE: usize = super::PERIPHERALS_BASE + 0x21_5000;

#[allow(non_snake_case)]
#[repr(packed)]
pub struct MiniUartStruct {
    pub IO: Register<u32>,
    pub IER: Register<u32>,
    pub IIR: Register<u32>,
    pub LCR: Register<u32>,
    pub MCR: Register<u32>,
    pub LSR: Register<u32>,
    pub MSR: Register<u32>,
    pub scratch: Register<u32>,
    pub control: Register<u32>,
    pub status: Register<u32>,
    pub baud_rate: Register<u32>,
}

#[allow(non_snake_case)]
#[repr(packed)]
pub struct AUXStruct {
    pub IRQ: Register<u32>,
    pub ENABLES: Register<u32>,
    _reserved: [u32; 14],
    pub MU: MiniUartStruct,
}

#[allow(non_upper_case_globals)]
const AUXPtr: *mut AUXStruct = AUX_BASE as *mut AUXStruct;
pub unsafe fn aux<'a>() -> &'a mut AUXStruct {
    &mut *AUXPtr
}
