use core::num::{
    NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize, NonZeroU16, NonZeroU32,
    NonZeroU64, NonZeroU8, NonZeroUsize,
};

use impl_for::impl_for;

/// Extension trait for converting into `Vec<u8>`
pub trait IntoBytes {
    fn into_bytes(self) -> Vec<u8>;
}

#[impl_for(T = "i8")]
#[impl_for(T = "u8")]
#[impl_for(T = "i16")]
#[impl_for(T = "u16")]
#[impl_for(T = "i32")]
#[impl_for(T = "u32")]
#[impl_for(T = "i64")]
#[impl_for(T = "u64")]
#[impl_for(T = "isize")]
#[impl_for(T = "usize")]
impl IntoBytes for T {
    fn into_bytes(self) -> Vec<u8> {
        let mut buf = ::itoa::Buffer::new();
        let s = buf.format(self);
        s.as_bytes().to_vec()
    }
}

#[impl_for(T = "NonZeroU8")]
#[impl_for(T = "NonZeroI8")]
#[impl_for(T = "NonZeroU16")]
#[impl_for(T = "NonZeroI16")]
#[impl_for(T = "NonZeroU32")]
#[impl_for(T = "NonZeroI32")]
#[impl_for(T = "NonZeroU64")]
#[impl_for(T = "NonZeroI64")]
#[impl_for(T = "NonZeroUsize")]
#[impl_for(T = "NonZeroIsize")]
impl IntoBytes for T {
    fn into_bytes(self) -> Vec<u8> {
        let mut buf = ::itoa::Buffer::new();
        let s = buf.format(self.get());
        s.as_bytes().to_vec()
    }
}

#[impl_for(T = "f32")]
#[impl_for(T = "f64")]
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
            vec![1]
        } else {
            vec![0]
        }
    }
}

impl IntoBytes for String {
    fn into_bytes(self) -> Vec<u8> {
        self.into_bytes()
    }
}

impl<'a> IntoBytes for &'a str {
    fn into_bytes(self) -> Vec<u8> {
        self.as_bytes().to_vec()
    }
}

impl IntoBytes for Vec<u8> {
    fn into_bytes(self) -> Vec<u8> {
        self
    }
}

impl<'a> IntoBytes for &'a [u8] {
    fn into_bytes(self) -> Vec<u8> {
        self.to_vec()
    }
}
