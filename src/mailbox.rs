use alloc::{vec, vec::Vec};
use core::marker::PhantomData;
use register::*;

pub const MAILBOX_BASE: usize = super::PERIPHERALS_BASE + 0xB880;

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub enum Channel {
    PowerManagement = 0,
    FrameBuffer = 1,
    VirtualUART = 2,
    VCHIQ = 3,
    Leds = 4,
    Buttons = 5,
    Touchscreen = 6,
    // Unsused = 7,
    TagsArmToVC = 8,
    TagsVCToARM = 9,
}

#[repr(packed)]
pub struct MailboxParam {
    _reserved: [u32; 3],
    pub peek: Register<u32>,
    pub sender: Register<u32>,
    pub status: Register<u32>,
    pub config: Register<u32>,
}
#[repr(packed)]
pub struct MailboxRead {
    read: Register<u32>,
    param: MailboxParam,
}
#[repr(packed)]
pub struct MailboxWrite {
    write: Register<u32>,
    param: MailboxParam,
}
#[repr(packed)]
pub struct MailboxStruct {
    read: MailboxRead,
    write: MailboxWrite,
}

#[allow(non_upper_case_globals)]
const MailboxPtr: *mut MailboxStruct = MAILBOX_BASE as *mut MailboxStruct;
unsafe fn mailbox<'a>() -> &'a mut MailboxStruct {
    &mut *MailboxPtr
}

#[derive(Clone, Copy)]
pub struct Mailbox;
impl Mailbox {
    pub fn write_message(&mut self, channel: Channel, value: u32) -> &mut Self {
        let mut message = value << 4;
        message |= channel as u32;

        while unsafe { mailbox().write.param.status.get(31) } {}

        unsafe { mailbox().write.write.write(message) }

        self
    }
    pub fn read_message(&mut self, channel: Channel) -> u32 {
        let message = loop {
            while unsafe { mailbox().read.param.status.get(30) } {}

            let message = unsafe { mailbox().read.read.read() };

            if (message & 0xF) == (channel as u32) {
                break message;
            }
        };

        message >> 4
    }
}

const END_REQUEST: u32 = 0;
const RESPONSE: u32 = 0;

pub mod tag_res {
    #[derive(Debug, Clone, Copy)]
    pub struct Size {
        pub width: u32,
        pub height: u32,
    }
    #[derive(Debug, Clone, Copy)]
    pub struct Ptr {
        pub ptr: *mut u8,
        pub bytes: usize,
    }
    #[derive(Debug, Clone, Copy)]
    pub struct Handle(pub(crate) u32);
}

pub mod tag {
    use super::*;

    pub struct GetFirmwareVersion;
    impl super::Tag for GetFirmwareVersion {
        #[inline]
        fn tag() -> u32 {
            0x1
        }
    }
    impl super::SerializableTag for GetFirmwareVersion {
        fn serialize(self, res: &mut Vec<u32>) {
            res.extend_from_slice(&[Self::tag(), 4, END_REQUEST, RESPONSE]);
        }
    }
    impl super::DeserializableTag for GetFirmwareVersion {
        type Output = u32;
        fn deserialize(input: &[u32]) -> Self::Output {
            input[0]
        }
    }
    pub struct AllocateBuffer;
    impl super::Tag for AllocateBuffer {
        #[inline]
        fn tag() -> u32 {
            0x40001
        }
    }
    impl super::SerializableTag for AllocateBuffer {
        fn serialize(self, res: &mut Vec<u32>) {
            res.extend_from_slice(&[Self::tag(), 8, END_REQUEST, 16, RESPONSE]);
        }
    }
    impl super::DeserializableTag for AllocateBuffer {
        type Output = tag_res::Ptr;
        fn deserialize(input: &[u32]) -> Self::Output {
            tag_res::Ptr {
                ptr: input[0] as *mut u8,
                bytes: input[1] as usize,
            }
        }
    }
    pub struct SetPhysicalSize {
        pub width: u32,
        pub height: u32,
    }
    impl super::Tag for SetPhysicalSize {
        #[inline]
        fn tag() -> u32 {
            0x48003
        }
    }
    impl super::SerializableTag for SetPhysicalSize {
        fn serialize(self, res: &mut Vec<u32>) {
            res.extend_from_slice(&[Self::tag(), 8, END_REQUEST, self.width, self.height]);
        }
    }
    pub struct GetPhysicalSize;
    impl super::Tag for GetPhysicalSize {
        #[inline]
        fn tag() -> u32 {
            0x40003
        }
    }
    impl super::SerializableTag for GetPhysicalSize {
        fn serialize(self, res: &mut Vec<u32>) {
            res.extend_from_slice(&[Self::tag(), 8, END_REQUEST, RESPONSE, RESPONSE]);
        }
    }
    impl super::DeserializableTag for GetPhysicalSize {
        type Output = tag_res::Size;
        fn deserialize(input: &[u32]) -> Self::Output {
            tag_res::Size {
                width: input[0],
                height: input[1],
            }
        }
    }
    pub struct SetVirtualSize {
        pub width: u32,
        pub height: u32,
    }
    impl super::Tag for SetVirtualSize {
        #[inline]
        fn tag() -> u32 {
            0x48004
        }
    }
    impl super::SerializableTag for SetVirtualSize {
        fn serialize(self, res: &mut Vec<u32>) {
            res.extend_from_slice(&[Self::tag(), 8, END_REQUEST, self.width, self.height]);
        }
    }
    pub struct GetVirtualSize;
    impl super::Tag for GetVirtualSize {
        #[inline]
        fn tag() -> u32 {
            0x40004
        }
    }
    impl super::SerializableTag for GetVirtualSize {
        fn serialize(self, res: &mut Vec<u32>) {
            res.extend_from_slice(&[Self::tag(), 8, END_REQUEST, RESPONSE, RESPONSE]);
        }
    }
    impl super::DeserializableTag for GetVirtualSize {
        type Output = tag_res::Size;
        fn deserialize(input: &[u32]) -> Self::Output {
            tag_res::Size {
                width: input[0],
                height: input[1],
            }
        }
    }
    pub struct GetDepth;
    impl super::Tag for GetDepth {
        #[inline]
        fn tag() -> u32 {
            0x48005
        }
    }
    impl super::SerializableTag for GetDepth {
        fn serialize(self, res: &mut Vec<u32>) {
            res.extend_from_slice(&[Self::tag(), 4, END_REQUEST, RESPONSE]);
        }
    }
    impl super::DeserializableTag for GetDepth {
        type Output = u32;
        fn deserialize(input: &[u32]) -> Self::Output {
            input[0]
        }
    }
    pub struct SetDepth(pub u32);
    impl super::Tag for SetDepth {
        #[inline]
        fn tag() -> u32 {
            0x48005
        }
    }
    impl super::SerializableTag for SetDepth {
        fn serialize(self, res: &mut Vec<u32>) {
            res.extend_from_slice(&[Self::tag(), 4, END_REQUEST, self.0]);
        }
    }
    pub struct GetPitch;
    impl super::Tag for GetPitch {
        #[inline]
        fn tag() -> u32 {
            0x40008
        }
    }
    impl super::SerializableTag for GetPitch {
        fn serialize(self, res: &mut Vec<u32>) {
            res.extend_from_slice(&[Self::tag(), 8, END_REQUEST, RESPONSE, RESPONSE]);
        }
    }
    impl super::DeserializableTag for GetPitch {
        type Output = u32;
        fn deserialize(input: &[u32]) -> Self::Output {
            input[0]
        }
    }

    pub struct BlankScreen(pub bool);
    impl super::Tag for BlankScreen {
        #[inline]
        fn tag() -> u32 {
            0x40002
        }
    }
    impl super::SerializableTag for BlankScreen {
        fn serialize(self, res: &mut Vec<u32>) {
            res.extend_from_slice(&[Self::tag(), 4, END_REQUEST, if self.0 { 1 } else { 0 }]);
        }
    }
    pub struct GetArmMemory;
    impl super::Tag for GetArmMemory {
        #[inline]
        fn tag() -> u32 {
            0x10005
        }
    }
    impl super::SerializableTag for GetArmMemory {
        fn serialize(self, res: &mut Vec<u32>) {
            res.extend_from_slice(&[Self::tag(), 8, END_REQUEST, RESPONSE, RESPONSE]);
        }
    }
    impl super::DeserializableTag for GetArmMemory {
        type Output = tag_res::Ptr;
        fn deserialize(input: &[u32]) -> Self::Output {
            tag_res::Ptr {
                ptr: input[0] as *mut u8,
                bytes: input[1] as usize,
            }
        }
    }

    pub enum AllocateMemoryFlags {
        Discardable = 1 << 0,
        Normal = 0 << 2,
        Direct = 1 << 2,
        Coherent = 2 << 2,
        L1NonAllocating =
            AllocateMemoryFlags::Direct as isize | AllocateMemoryFlags::Coherent as isize,
        Zero = 1 << 4,
        NoInit = 1 << 5,
        HintPermalock = 1 << 6,
    }
    pub struct AllocateMemory {
        pub size: u32,
        pub alignment: u32,
        pub flags: u32,
    }
    impl super::Tag for AllocateMemory {
        fn tag() -> u32 {
            0x3000C
        }
    }
    impl super::SerializableTag for AllocateMemory {
        fn serialize(self, res: &mut Vec<u32>) {
            res.extend_from_slice(&[
                Self::tag(),
                12,
                END_REQUEST,
                self.size,
                self.alignment,
                self.flags,
            ])
        }
    }
    impl super::DeserializableTag for AllocateMemory {
        type Output = super::tag_res::Handle;
        fn deserialize(input: &[u32]) -> Self::Output {
            super::tag_res::Handle(input[0])
        }
    }

    pub struct ReleaseMemory(pub super::tag_res::Handle);
    impl Tag for ReleaseMemory {
        fn tag() -> u32 {
            0x3000f
        }
    }
    impl SerializableTag for ReleaseMemory {
        fn serialize(self, res: &mut Vec<u32>) {
            res.extend_from_slice(&[Self::tag(), 4, self.0 .0])
        }
    }
    impl DeserializableTag for ReleaseMemory {
        type Output = Result<(), ()>;
        fn deserialize(input: &[u32]) -> Self::Output {
            if input[0] == 0 {
                Ok(())
            } else {
                Err(())
            }
        }
    }
}

pub trait Tag {
    fn tag() -> u32;
}
pub trait SerializableTag: Tag {
    fn serialize(self, res: &mut Vec<u32>);
}
pub trait DeserializableTag: Tag {
    type Output;
    fn deserialize(input: &[u32]) -> Self::Output;
}

pub mod state {
    pub struct Draft;
    pub struct Sent;
}

pub struct Message<STATE> {
    buffer: Vec<u32>,
    _state: core::marker::PhantomData<STATE>,
}
impl Message<state::Draft> {
    pub fn new() -> Self {
        let buffer = vec![
            3 * 4, // size
            0,     // somthing I don't understand yet
            0,     // null terminated message
        ];

        Message {
            buffer,
            _state: PhantomData,
        }
    }
    pub fn with_tag<T: SerializableTag>(mut self, tag: T) -> Self {
        self.add_tag(tag);
        self
    }
    pub fn add_tag<T: SerializableTag>(&mut self, tag: T) -> &mut Self {
        self.buffer.pop(); // remove null terminated
        tag.serialize(&mut self.buffer); // write tag
        self.buffer.push(0); // null terminated

        self.buffer[0] = (self.buffer.len() as u32) << 2; // update message len
        self
    }
    pub fn commit(mut self) -> Result<Message<state::Sent>, ()> {
        self.buffer.reserve(4);
        for _ in 0..4 {
            self.buffer.push(0);
        }
        let v = (self.buffer.as_ptr() as u32) >> 4;
        Mailbox
            .write_message(Channel::TagsArmToVC, v)
            .read_message(Channel::TagsArmToVC);

        let Message { buffer, _state } = self;

        if buffer[1] != 0x80000000 {
            return Err(());
        }

        Ok(Message {
            buffer,
            _state: PhantomData,
        })
    }
}
impl Message<state::Sent> {
    pub fn get<T: DeserializableTag>(&self) -> Option<T::Output> {
        let mut ptr = 2;

        loop {
            if self.buffer[ptr] == 0 {
                break None;
            }

            let size = (self.buffer[ptr + 1] >> 2) + 3;
            let next_ptr = ptr + size as usize;

            if self.buffer[ptr] == T::tag() {
                break Some(T::deserialize(&self.buffer[ptr + 3..next_ptr]));
            };

            ptr = next_ptr;
        }
    }
}

// impl Mailbox {
//     pub fn send_to_vc(&mut self) {
//         let _res = unsafe {
//             MailboxInterface::write(
//                 self.buffer.as_mut_ptr() as *mut u32,
//                 Mailbox0Channel::TagsArmToVC,
//             );
//             MailboxInterface::read(Mailbox0Channel::TagsArmToVC)
//         };
//     }
// }

// pub trait Getter: Sized {
//     type Output;
//     fn tag() -> Tag;
//     fn parse(buffer: &[U32Alligned16]) -> Self::Output;
// }

// pub mod tag {
//     pub struct FirmwareVersion;
//     pub struct AllocateBuffer;
//     pub struct GetPhysicalSize {
//         pub w: usize,
//         pub h: usize,
//     }
//     pub struct GetPitch;
// }
// impl Getter for tag::FirmwareVersion {
//     type Output = u32;
//     #[inline]
//     fn tag() -> Tag {
//         Tag::GetFirmwareVersion
//     }
//     #[inline]
//     fn parse(buffer: &[U32Alligned16]) -> Self::Output {
//         *buffer[0]
//     }
// }
// impl Getter for tag::AllocateBuffer {
//     type Output = *mut ();
//     #[inline]
//     fn tag() -> Tag {
//         Tag::AllocateBuffer(0)
//     }
//     #[inline]
//     fn parse(buffer: &[U32Alligned16]) -> Self::Output {
//         (*buffer[0] & 0x3FFFFFFF) as *mut ()
//     }
// }
// impl Getter for tag::GetPhysicalSize {
//     type Output = tag::GetPhysicalSize;
//     #[inline]
//     fn tag() -> Tag {
//         Tag::GetPhysicalSize
//     }
//     #[inline]
//     fn parse(buffer: &[U32Alligned16]) -> Self::Output {
//         tag::GetPhysicalSize {
//             w: *buffer[0] as usize,
//             h: *buffer[1] as usize,
//         }
//     }
// }
// impl Getter for tag::GetPitch {
//     type Output = u32;
//     #[inline]
//     fn tag() -> Tag {
//         Tag::GetPitch
//     }
//     #[inline]
//     fn parse(buffer: &[U32Alligned16]) -> Self::Output {
//         *buffer[0]
//     }
// }

//\////

//

//

// pub enum Tag {
//     GetFirmwareVersion,

//     /* Hardware */
//     GetBoardModel,
//     GetBoardRevision,
//     GetBoardMacAddress,
//     GetBoardSerial,
//     GetArmMemory,
//     GetVcMemory,
//     GetClocks,

//     /* Config */
//     GetCommandLine,

//     /* Shared resource management */
//     GetDMAChannels,

//     /* Power */
//     GetPowerState,
//     GetTIMING,
//     SetPowerState,

//     /* Clocks */
//     GetClockState,
//     SetClockState,
//     GetClockRate(u32),
//     SetClockRate {
//         clk_id: u32,
//         rate_hz: u32,
//         skip_turbo: bool,
//     },
//     GetMaxClockRate(u32),
//     GetMinClockRate(u32),
//     GetTurbo,
//     SetTurbo,

//     /* Voltage */
//     GetVoltage,
//     SetVoltage,
//     GetMaxVoltage,
//     GetMinVoltage,
//     GetTemperature,
//     GetMaxTemperature,
//     ALLOCATE_MEMORY,
//     LOCK_MEMORY,
//     UNLOCK_MEMORY,
//     RELEASE_MEMORY,
//     EXECUTE_CODE,
//     GetDISPMANX_MEM_HANDLE,
//     GetEDID_BLOCK,

//     /* Framebuffer */
//     AllocateBuffer(u32),
//     RELEASE_BUFFER,
//     BLANK_SCREEN,
//     GetPhysicalSize,
//     TestPhysicalSize {
//         w: u32,
//         h: u32,
//     },
//     SetPhysicalSize {
//         w: u32,
//         h: u32,
//     },
//     GetVirtualSize,
//     TestVirtualSize {
//         w: u32,
//         h: u32,
//     },
//     SetVirtualSize {
//         w: u32,
//         h: u32,
//     },
//     GetDepth,
//     TestDEPTH,
//     SetDepth(u32),
//     GetPIXEL_ORDER,
//     TestPIXEL_ORDER,
//     SetPixelOrder(u32),
//     GetALPHA_MODE,
//     TestALPHA_MODE,
//     SetALPHA_MODE(u32),
//     GetPitch,
//     GetVirtualOffset,
//     TestVirtualOffset,
//     SetVirtualOffset {
//         w: u32,
//         h: u32,
//     },
//     GetOVERSCAN,
//     TestOVERSCAN,
//     SetOVERSCAN {
//         top: u32,
//         bottom: u32,
//         left: u32,
//         right: u32,
//     },
//     GetPALETTE,
//     TestPALETTE,
//     SetPALETTE,
//     SetCURSOR_INFO,
//     SetCURSORState,
// }
// impl Tag {
//     const fn tag_code(&self) -> u32 {
//         match self {
//             Tag::GetFirmwareVersion => 0x1,
//             Tag::GetBoardModel => 0x10001,
//             Tag::GetBoardRevision => 0x10002,
//             Tag::GetBoardMacAddress => 0x10003,
//             Tag::GetBoardSerial => 0x10004,
//             Tag::GetArmMemory => 0x10005,
//             Tag::GetVcMemory => 0x10006,
//             Tag::GetClocks => 0x10007,
//             Tag::GetCommandLine => 0x50001,
//             Tag::GetDMAChannels => 0x60001,
//             Tag::GetPowerState => 0x20001,
//             Tag::GetTIMING => 0x20002,
//             Tag::SetPowerState => 0x28001,
//             Tag::GetClockState => 0x30001,
//             Tag::SetClockState => 0x38001,
//             Tag::GetClockRate(..) => 0x30002,
//             Tag::SetClockRate { .. } => 0x38002,
//             Tag::GetMaxClockRate(..) => 0x30004,
//             Tag::GetMinClockRate(..) => 0x30007,
//             Tag::GetTurbo => 0x30009,
//             Tag::SetTurbo => 0x38009,
//             Tag::GetVoltage => 0x30003,
//             Tag::SetVoltage => 0x38003,
//             Tag::GetMaxVoltage => 0x30005,
//             Tag::GetMinVoltage => 0x30008,
//             Tag::GetTemperature => 0x30006,
//             Tag::GetMaxTemperature => 0x3000A,
//             Tag::ALLOCATE_MEMORY => 0x3000C,
//             Tag::LOCK_MEMORY => 0x3000D,
//             Tag::UNLOCK_MEMORY => 0x3000E,
//             Tag::RELEASE_MEMORY => 0x3000F,
//             Tag::EXECUTE_CODE => 0x30010,
//             Tag::GetDISPMANX_MEM_HANDLE => 0x30014,
//             Tag::GetEDID_BLOCK => 0x30020,
//             Tag::AllocateBuffer(..) => 0x40001,
//             Tag::RELEASE_BUFFER => 0x48001,
//             Tag::BLANK_SCREEN => 0x40002,
//             Tag::GetPhysicalSize => 0x40003,
//             Tag::TestPhysicalSize { .. } => 0x44003,
//             Tag::SetPhysicalSize { .. } => 0x48003,
//             Tag::GetVirtualSize => 0x40004,
//             Tag::TestVirtualSize { .. } => 0x44004,
//             Tag::SetVirtualSize { .. } => 0x48004,
//             Tag::GetDepth => 0x40005,
//             Tag::TestDEPTH => 0x44005,
//             Tag::SetDepth(..) => 0x48005,
//             Tag::GetPIXEL_ORDER => 0x40006,
//             Tag::TestPIXEL_ORDER => 0x44006,
//             Tag::SetPixelOrder(..) => 0x48006,
//             Tag::GetALPHA_MODE => 0x40007,
//             Tag::TestALPHA_MODE => 0x44007,
//             Tag::SetALPHA_MODE(..) => 0x48007,
//             Tag::GetPitch => 0x40008,
//             Tag::GetVirtualOffset => 0x40009,
//             Tag::TestVirtualOffset => 0x44009,
//             Tag::SetVirtualOffset { .. } => 0x48009,
//             Tag::GetOVERSCAN => 0x4000A,
//             Tag::TestOVERSCAN => 0x4400A,
//             Tag::SetOVERSCAN { .. } => 0x4800A,
//             Tag::GetPALETTE => 0x4000B,
//             Tag::TestPALETTE => 0x4400B,
//             Tag::SetPALETTE => 0x4800B,
//             Tag::SetCURSOR_INFO => 0x8011,
//             Tag::SetCURSORState => 0x8010,
//         }
//     }
//     fn serialize_into(&self, buffer: &mut alloc::vec::Vec<U32Alligned16>) {
//         match self {
//             Tag::GetFirmwareVersion
//             | Tag::GetBoardModel
//             | Tag::GetBoardRevision
//             | Tag::GetBoardMacAddress
//             | Tag::GetBoardSerial
//             | Tag::GetArmMemory
//             | Tag::GetVcMemory
//             | Tag::GetDMAChannels => buffer.extend_from_slice(&[
//                 U32Alligned16(self.tag_code()),
//                 U32Alligned16(8),
//                 REQUEST,
//                 RES,
//                 RES,
//             ]),
//             Tag::GetClocks | Tag::GetCommandLine => {
//                 unimplemented!()
//                 // buffer.extend_from_slice(&[self.tag_code(), 256, REQUEST]);
//                 // buffer.resize(buffer.len() + (256 >> 2), RES);
//             }
//             Tag::AllocateBuffer(x)
//             | Tag::GetClockRate(x)
//             | Tag::GetMaxClockRate(x)
//             | Tag::GetMinClockRate(x) => {
//                 buffer.extend_from_slice(&[
//                     U32Alligned16(self.tag_code()),
//                     U32Alligned16(8),
//                     REQUEST,
//                     U32Alligned16(*x),
//                     RES,
//                 ]);
//             }
//             Tag::SetClockRate {
//                 clk_id,
//                 rate_hz,
//                 skip_turbo,
//             } => {
//                 buffer.extend_from_slice(&[
//                     U32Alligned16(self.tag_code()),
//                     U32Alligned16(12),
//                     REQUEST,
//                     U32Alligned16(*clk_id),
//                     U32Alligned16(*rate_hz),
//                     U32Alligned16(if *skip_turbo { 1 } else { 0 }),
//                 ]);
//             }
//             Tag::SetPhysicalSize { w, h }
//             | Tag::SetVirtualSize { w, h }
//             | Tag::SetVirtualOffset { w, h }
//             | Tag::TestPhysicalSize { w, h }
//             | Tag::TestVirtualSize { w, h } => {
//                 buffer.extend_from_slice(&[
//                     U32Alligned16(self.tag_code()),
//                     U32Alligned16(8),
//                     REQUEST,
//                     U32Alligned16(*w),
//                     U32Alligned16(*h),
//                 ]);
//             }
//             Tag::GetPhysicalSize | Tag::GetVirtualSize | Tag::GetVirtualOffset => {
//                 buffer.extend_from_slice(&[
//                     U32Alligned16(self.tag_code()),
//                     U32Alligned16(8),
//                     REQUEST,
//                     RES,
//                     RES,
//                 ]);
//             }
//             Tag::GetALPHA_MODE | Tag::GetDepth | Tag::GetPitch | Tag::GetPIXEL_ORDER => {
//                 buffer.extend_from_slice(&[
//                     U32Alligned16(self.tag_code()),
//                     U32Alligned16(4),
//                     REQUEST,
//                     RES,
//                 ]);
//             }
//             Tag::SetALPHA_MODE(x) | Tag::SetDepth(x) | Tag::SetPixelOrder(x) => {
//                 buffer.extend_from_slice(&[
//                     U32Alligned16(self.tag_code()),
//                     U32Alligned16(4),
//                     REQUEST,
//                     U32Alligned16(*x),
//                 ]);
//             }
//             Tag::GetOVERSCAN => {
//                 buffer.extend_from_slice(&[
//                     U32Alligned16(self.tag_code()),
//                     U32Alligned16(16),
//                     REQUEST,
//                     RES,
//                     RES,
//                     RES,
//                     RES,
//                 ]);
//             }
//             Tag::SetOVERSCAN {
//                 top,
//                 bottom,
//                 left,
//                 right,
//             } => {
//                 buffer.extend_from_slice(&[
//                     U32Alligned16(self.tag_code()),
//                     U32Alligned16(16),
//                     REQUEST,
//                     U32Alligned16(*top),
//                     U32Alligned16(*bottom),
//                     U32Alligned16(*left),
//                     U32Alligned16(*right),
//                 ]);
//             }
//             _ => {
//                 panic!("Not supported");
//             }
//         }
//     }
// }

// // #[repr(C, align(16))]
// #[repr(transparent)]
// #[derive(Clone, Copy, Debug)]
// pub struct U32Alligned16(u32);
// impl core::ops::Deref for U32Alligned16 {
//     type Target = u32;
//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }
// impl core::ops::DerefMut for U32Alligned16 {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.0
//     }
// }
