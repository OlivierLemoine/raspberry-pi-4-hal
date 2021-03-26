use register::*;

pub const GPIO_BASE: usize = super::PERIPHERALS_BASE + 0x20_0000;

#[repr(packed)]
pub struct GPIOAccess {
    _reserved: u32,
    value: [Register<u32>; 2],
}

#[allow(non_snake_case)]
#[derive(Clone, Copy)]
#[repr(packed)]
pub struct PupdnClk {
    _reserved: u32,
    pub GPPUD: Register<u32>,
    pub GPPUDCLK: [Register<u32>; 2],
}

#[allow(non_snake_case)]
#[derive(Clone, Copy)]
#[repr(packed)]
pub struct PupdnOff {
    _reserved: [u32; 0x15],
    pub GPIO_PUP_PDN_CNTRL: [Register<u32>; 4],
}

#[allow(non_snake_case)]
#[derive(Clone, Copy)]
#[repr(packed)]
pub struct Pupdn {
    _reserved: u32,
    pub GPIO_PUP_PDN_CNTRL: [Register<u32>; 4],
}

#[allow(non_snake_case)]
#[repr(packed)]
pub union PupdnUnion {
    pub CNTRL: Pupdn,
    pub CLK: PupdnClk,
    pub CNTRL_OFF: PupdnOff,
}

#[allow(non_snake_case)]
#[repr(packed)]
pub struct GPIOStruct {
    pub GPFSEL: [Register<u32>; 6],
    pub GPSET: GPIOAccess,
    pub GPCLR: GPIOAccess,
    pub GPLEV: GPIOAccess,
    pub GPEDS: GPIOAccess,
    pub GPREN: GPIOAccess,
    pub GPFEN: GPIOAccess,
    pub GPHEN: GPIOAccess,
    pub GPLEN: GPIOAccess,
    pub GPAREN: GPIOAccess,
    pub GPAFEN: GPIOAccess,
    pub GPPUD: PupdnUnion,
}

#[allow(non_upper_case_globals)]
const GPIOPtr: *mut GPIOStruct = GPIO_BASE as *mut GPIOStruct;
unsafe fn gpio<'a>() -> &'a mut GPIOStruct {
    &mut *GPIOPtr
}

#[derive(Clone, Copy)]
pub enum Function {
    Input = 0b000,
    Output = 0b001,
    Alternate0 = 0b100,
    Alternate1 = 0b101,
    Alternate2 = 0b110,
    Alternate3 = 0b111,
    Alternate4 = 0b011,
    Alternate5 = 0b010,
}

#[derive(Clone, Copy)]
pub enum Resistor {
    No = 0b00,
    PullUp = 0b01,
    PullDown = 0b10,
}

pub struct GPIO(pub usize);
impl GPIO {
    pub unsafe fn set(&mut self, v: bool) -> &mut Self {
        if v {
            &mut gpio().GPSET
        } else {
            &mut gpio().GPCLR
        }
        .value
        .get_unchecked_mut(self.0 / 32)
        .set((self.0 % 32) as u32, true);
        self
    }
    pub unsafe fn get(&self) -> bool {
        gpio()
            .GPLEV
            .value
            .get_unchecked_mut(self.0 / 32)
            .get((self.0 % 32) as u32)
    }
    pub unsafe fn set_function(&mut self, function: Function) -> &mut Self {
        gpio()
            .GPFSEL
            .get_unchecked_mut(self.0 / 10)
            .write_with_mask_at(function as u32, 0b111, ((self.0 % 10) * 3) as u32);
        self
    }
    pub unsafe fn set_pullup_pulldown(&mut self, pup_pdn: Resistor) -> &mut Self {
        gpio()
            .GPPUD
            .CNTRL_OFF
            .GPIO_PUP_PDN_CNTRL
            .get_unchecked_mut(self.0 / 16)
            .write_with_mask_at(pup_pdn as u32, 0x3, ((self.0 % 16) * 2) as u32);

        // gpio().GPPUD.CLK.GPPUD.write(pup_pdn.value());
        // crate::asm::delay(150);
        // gpio()
        //     .GPPUD
        //     .CLK
        //     .GPPUDCLK
        //     .get_unchecked_mut(self.0 / 32)
        //     .write(1 << (self.0 % 32));
        // crate::asm::delay(150);
        // // gpio().GPPUD.CLK.GPPUD.write(0);
        // gpio()
        //     .GPPUD
        //     .CLK
        //     .GPPUDCLK
        //     .get_unchecked_mut(self.0 / 32)
        //     .write(0);

        self
    }
    pub unsafe fn rising_edge_detect(&mut self, v: bool) -> &mut Self {
        gpio()
            .GPREN
            .value
            .get_unchecked_mut(self.0 / 32)
            .set((self.0 % 32) as u32, v);
        self
    }
    pub unsafe fn falling_edge_detect(&mut self, v: bool) -> &mut Self {
        gpio()
            .GPFEN
            .value
            .get_unchecked_mut(self.0 / 32)
            .set((self.0 % 32) as u32, v);
        self
    }
    pub unsafe fn high_edge_detect(&mut self, v: bool) -> &mut Self {
        gpio()
            .GPHEN
            .value
            .get_unchecked_mut(self.0 / 32)
            .set((self.0 % 32) as u32, v);
        self
    }
    pub unsafe fn low_edge_detect(&mut self, v: bool) -> &mut Self {
        gpio()
            .GPLEN
            .value
            .get_unchecked_mut(self.0 / 32)
            .set((self.0 % 32) as u32, v);
        self
    }
}

pub trait GPIOTuple {
    unsafe fn set_function(&mut self, function: Function) -> &mut Self;
    unsafe fn set_pullup_pulldown(&mut self, pup_pdn: Resistor) -> &mut Self;
}
impl GPIOTuple for (GPIO, GPIO) {
    unsafe fn set_function(&mut self, function: Function) -> &mut Self {
        self.0.set_function(function);
        self.1.set_function(function);
        self
    }
    unsafe fn set_pullup_pulldown(&mut self, pup_pdn: Resistor) -> &mut Self {
        self.0.set_pullup_pulldown(pup_pdn);
        self.1.set_pullup_pulldown(pup_pdn);
        self
    }
}
