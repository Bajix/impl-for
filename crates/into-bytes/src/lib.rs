#![no_std]

extern crate alloc;

use alloc::{string::String, vec::Vec};
use core::num::{
    NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize, NonZeroU16, NonZeroU32,
    NonZeroU64, NonZeroU8, NonZeroUsize,
};

use impl_for::impl_for_each;

/// Extension trait for converting into `Vec<u8>`
pub trait IntoBytes {
    fn into_bytes(self) -> Vec<u8>;
}

#[impl_for_each(i8, u8, i16, u16, i32, u32, i64, u64, isize, usize)]
impl IntoBytes for T {
    fn into_bytes(self) -> Vec<u8> {
        let mut buf = ::itoa::Buffer::new();
        let s = buf.format(self);
        s.as_bytes().to_vec()
    }
}

#[impl_for_each(
    NonZeroU8,
    NonZeroI8,
    NonZeroU16,
    NonZeroI16,
    NonZeroU32,
    NonZeroI32,
    NonZeroU64,
    NonZeroI64,
    NonZeroUsize,
    NonZeroIsize
)]
impl IntoBytes for T {
    fn into_bytes(self) -> Vec<u8> {
        let mut buf = ::itoa::Buffer::new();
        let s = buf.format(self.get());
        s.as_bytes().to_vec()
    }
}

#[impl_for_each(f32, f64)]
impl IntoBytes for T {
    fn into_bytes(self) -> Vec<u8> {
        let mut buf = ::ryu::Buffer::new();
        let s = buf.format(self);
        s.as_bytes().to_vec()
    }
}

impl IntoBytes for bool {
    fn into_bytes(self) -> Vec<u8> {
        if self {
            Vec::from([1])
        } else {
            Vec::from([0])
        }
    }
}

impl IntoBytes for String {
    fn into_bytes(self) -> Vec<u8> {
        self.into_bytes()
    }
}

impl IntoBytes for &str {
    fn into_bytes(self) -> Vec<u8> {
        self.as_bytes().to_vec()
    }
}

impl IntoBytes for Vec<u8> {
    fn into_bytes(self) -> Vec<u8> {
        self
    }
}

impl IntoBytes for &[u8] {
    fn into_bytes(self) -> Vec<u8> {
        self.to_vec()
    }
}
