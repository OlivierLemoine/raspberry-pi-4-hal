#![no_std]

use core::{
    ops::{BitAnd, BitOr, Not, Shl},
    ptr::{read_volatile, write_volatile},
};

macro_rules! impl_reg_op {
    ($t:ty) => {
        impl RegOp for $t {
            #[inline]
            fn zero() -> Self {
                0
            }
            #[inline]
            fn one() -> Self {
                1
            }
        }
    };
}

pub trait RegOp {
    fn zero() -> Self;
    fn one() -> Self;
}

impl_reg_op!(u64);
impl_reg_op!(u32);
impl_reg_op!(u16);
impl_reg_op!(u8);
impl_reg_op!(i64);
impl_reg_op!(i32);
impl_reg_op!(i16);
impl_reg_op!(i8);
impl_reg_op!(usize);
impl_reg_op!(isize);

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Register<T>(T);
impl<T> Register<T> {
    #[inline]
    pub unsafe fn read(&self) -> T {
        read_volatile(self as *const Self as *const T)
    }
    #[inline]
    pub unsafe fn write(&mut self, v: T) {
        write_volatile(self as *mut Self as *mut T, v)
    }
    #[inline]
    pub unsafe fn read_with_mask(&self, m: T) -> T
    where
        T: BitAnd<Output = T>,
    {
        self.read() & m
    }
    #[inline]
    pub unsafe fn read_with_mask_at(&self, m: T, at: T) -> T
    where
        T: BitAnd<Output = T> + Shl<Output = T>,
    {
        self.read_with_mask(m << at)
    }
    #[inline]
    pub unsafe fn write_with_mask(&mut self, v: T, m: T)
    where
        T: BitAnd<Output = T> + BitOr<Output = T> + Not<Output = T>,
    {
        self.write(self.read_with_mask(!m) | v)
    }
    #[inline]
    pub unsafe fn write_with_mask_at(&mut self, v: T, m: T, at: T)
    where
        T: BitAnd<Output = T> + BitOr<Output = T> + Not<Output = T> + Shl<Output = T> + Clone,
    {
        self.write_with_mask(v << at.clone(), m << at)
    }
    #[inline]
    pub unsafe fn get(&self, b: T) -> bool
    where
        T: BitAnd<Output = T> + Shl<Output = T> + RegOp + PartialEq,
    {
        self.read_with_mask_at(T::one(), b) != T::zero()
    }
    #[inline]
    pub unsafe fn set(&mut self, b: T, v: bool)
    where
        T: BitOr<Output = T>
            + BitAnd<Output = T>
            + Shl<Output = T>
            + Not<Output = T>
            + Clone
            + RegOp,
    {
        self.write_with_mask_at(if v { T::one() } else { T::zero() }, T::one(), b)
    }
}
