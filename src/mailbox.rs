//use core::marker::PhantomData;
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
    impl Tag for GetFirmwareVersion {
        const ID: u32 = 0x1;
        const LEN: usize = 1;
        type Res = (u32,);
        fn deserialize(from: &[u32]) -> Self::Res {
            (from[0],)
        }
    }

    pub struct AllocateBuffer;
    impl Tag for AllocateBuffer {
        const ID: u32 = 0x40001;
        const LEN: usize = 2;
        type Res = (tag_res::Ptr,);
        fn serialize(self, buffer: &mut [u32]) {
            buffer[0] = 16; // Alignment
            buffer[1] = RESPONSE;
        }
        fn deserialize(from: &[u32]) -> Self::Res {
            (tag_res::Ptr {
                ptr: from[0] as *mut u8,
                bytes: from[1] as usize,
            },)
        }
    }

    pub struct SetPhysicalSize {
        pub width: u32,
        pub height: u32,
    }
    impl Tag for SetPhysicalSize {
        const ID: u32 = 0x48003;
        const LEN: usize = 2;
        type Res = ();
        fn serialize(self, buffer: &mut [u32]) {
            buffer[0] = self.width;
            buffer[1] = self.height;
        }
        fn deserialize(_: &[u32]) -> Self::Res {
            ()
        }
    }

    pub struct GetPhysicalSize;
    impl Tag for GetPhysicalSize {
        const ID: u32 = 0x40003;
        const LEN: usize = 2;
        type Res = (tag_res::Size,);
        fn deserialize(from: &[u32]) -> Self::Res {
            (tag_res::Size {
                width: from[0],
                height: from[1],
            },)
        }
    }

    pub struct SetVirtualSize {
        pub width: u32,
        pub height: u32,
    }
    impl Tag for SetVirtualSize {
        const ID: u32 = 0x48004;
        const LEN: usize = 2;
        type Res = ();
        fn serialize(self, buffer: &mut [u32]) {
            buffer[0] = self.width;
            buffer[1] = self.height;
        }
        fn deserialize(_: &[u32]) -> Self::Res {
            ()
        }
    }

    pub struct GetVirtualSize;
    impl Tag for GetVirtualSize {
        const ID: u32 = 0x40004;
        const LEN: usize = 2;
        type Res = (tag_res::Size,);
        fn deserialize(from: &[u32]) -> Self::Res {
            (tag_res::Size {
                width: from[0],
                height: from[1],
            },)
        }
    }

    pub struct SetDepth(pub u32);
    impl Tag for SetDepth {
        const ID: u32 = 0x48005;
        const LEN: usize = 1;
        type Res = ();
        fn serialize(self, buffer: &mut [u32]) {
            buffer[0] = self.0;
        }
        fn deserialize(_: &[u32]) -> Self::Res {
            ()
        }
    }

    pub struct GetDepth;
    impl Tag for GetDepth {
        const ID: u32 = 0x48005;
        const LEN: usize = 1;
        type Res = (u32,);
        fn deserialize(from: &[u32]) -> Self::Res {
            (from[0],)
        }
    }

    pub struct GetPitch;
    impl Tag for GetPitch {
        const ID: u32 = 0x40008;
        const LEN: usize = 2;
        type Res = (u32,);
        fn deserialize(from: &[u32]) -> Self::Res {
            (from[0],)
        }
    }

    pub struct GetArmMemory;
    impl Tag for GetArmMemory {
        const ID: u32 = 0x10005;
        const LEN: usize = 2;
        type Res = (tag_res::Ptr,);
        fn deserialize(from: &[u32]) -> Self::Res {
            (tag_res::Ptr {
                ptr: from[0] as *mut u8,
                bytes: from[1] as usize,
            },)
        }
    }
}

pub trait TagsHolder: Sized {
    const LEN: usize;
    type Res;
    fn serialize(self, buffer: &mut [u32]);
    fn deserialize(buffer: &[u32]) -> Self::Res;
}
pub trait ResHolder: Sized {
    //
}
pub struct NoTag;
impl TagsHolder for NoTag {
    const LEN: usize = 0;
    type Res = ();
    fn serialize(self, buffer: &mut [u32]) {
        assert!(buffer.is_empty());
    }
    fn deserialize(buffer: &[u32]) -> Self::Res {
        assert!(buffer.is_empty());
        ()
    }
}
pub struct Tags<T, O>(T, O);
impl<T: Tag, O: TagsHolder> TagsHolder for Tags<T, O> {
    const LEN: usize = T::LEN + 3 + O::LEN;
    type Res = (T::Res, O::Res);
    fn serialize(self, buffer: &mut [u32]) {
        let len = T::LEN + 3;
        let split = buffer.len() - len;
        let (b_o, b_t) = buffer.split_at_mut(split);
        self.1.serialize(b_o);
        let buffer = b_t;
        buffer[0] = T::ID;
        buffer[1] = 4 * (T::LEN as u32);
        //buffer[2] = END_REQUEST;
        let buffer = &mut buffer[3..];
        self.0.serialize(buffer);
    }
    fn deserialize(buffer: &[u32]) -> Self::Res {
        let len = T::LEN + 3;
        let split = buffer.len() - len;
        let (b_o, b_t) = buffer.split_at(split);
        let other = O::deserialize(b_o);
        let tag_res = T::deserialize(&b_t[3..]);

        (tag_res, other)
    }
}

pub trait Flatten {
    type Out;
    fn flatten(self) -> Self::Out;
}
impl Flatten for () {
    type Out = ();
    fn flatten(self) -> Self::Out {
        ()
    }
}
impl Flatten for ((), ()) {
    type Out = ();
    fn flatten(self) -> Self::Out {
        ()
    }
}
impl<A> Flatten for ((A,), ()) {
    type Out = A;
    fn flatten(self) -> Self::Out {
        let ((a,), _) = self;
        a
    }
}
impl<A> Flatten for ((A,), ((), ())) {
    type Out = A;
    fn flatten(self) -> Self::Out {
        let ((a,), _) = self;
        a
    }
}
impl<A> Flatten for ((A,), ((), ((), ()))) {
    type Out = A;
    fn flatten(self) -> Self::Out {
        let ((a,), _) = self;
        a
    }
}
impl<A> Flatten for ((A,), ((), ((), ((), ())))) {
    type Out = A;
    fn flatten(self) -> Self::Out {
        let ((a,), _) = self;
        a
    }
}
impl<A, B, C, D> Flatten for ((A,), ((B,), ((C,), ((D,), ())))) {
    type Out = (D, C, B, A);
    fn flatten(self) -> Self::Out {
        let ((a,), ((b,), ((c,), ((d,), _)))) = self;
        (d, c, b, a)
    }
}

pub trait Tag: Sized {
    const ID: u32;
    const LEN: usize;
    type Res;
    fn serialize(self, buffer: &mut [u32]) {
        for i in 0..Self::LEN {
            //buffer[i] = RESPONSE
        }
    }
    fn deserialize(from: &[u32]) -> Self::Res;
}

#[repr(C, align(16))]
struct Align16Buffer<T, const LEN: usize>([T; LEN]);

pub struct Message<T> {
    tags: T,
}
impl Message<NoTag> {
    pub fn new() -> Self {
        Message { tags: NoTag }
    }
    pub fn with<T: Tag>(self, tag: T) -> Message<Tags<T, NoTag>> {
        Message {
            tags: Tags(tag, NoTag),
        }
    }
}
impl<O1, O2> Message<Tags<O1, O2>> {
    pub fn with<T: Tag>(self, tag: T) -> Message<Tags<T, Tags<O1, O2>>> {
        Message {
            tags: Tags(tag, self.tags),
        }
    }
    pub fn commit(self) -> Result<<<Tags<O1, O2> as TagsHolder>::Res as Flatten>::Out, ()>
    where
        O1: Tag,
        O2: TagsHolder,
        [u32; Tags::<O1, O2>::LEN + 3]: Sized,
        <Tags<O1, O2> as TagsHolder>::Res: Flatten,
    {
        let mut buffer = Align16Buffer([0u32; Tags::<O1, O2>::LEN + 3]); // size + request code + tags + zero terminated
        buffer.0[0] = (buffer.0.len() << 2) as u32; // Size
                                                    //buffer.0[1] = 0; // Req code

        let buffer_len = buffer.0.len();
        self.tags.serialize(&mut buffer.0[2..buffer_len - 1]);

        //buffer.0[buffer.0.len() - 1] = 0; // Zero terminated

        let v = (buffer.0.as_ptr() as u32) >> 4;
        let _ = Mailbox
            .write_message(Channel::TagsArmToVC, v)
            .read_message(Channel::TagsArmToVC);

        if buffer.0[1] != 0x80000000 {
            return Err(());
        }

        let res = Tags::<O1, O2>::deserialize(&buffer.0[2..buffer_len - 1]);

        Ok(res.flatten())
    }
}
