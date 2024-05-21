![License](https://img.shields.io/badge/license-MIT-green.svg)
[![Cargo](https://img.shields.io/crates/v/impl-for.svg)](https://crates.io/crates/impl-for)
[![Documentation](https://docs.rs/impl-for/badge.svg)](https://docs.rs/impl-for)

The `impl_for` macro is used for repeating implementations with type substitutions.

## Example

Before:
```rust
macro_rules! itoa_into_bytes {
    ($t:ty) => {
        impl IntoBytes for $t {
            fn into_bytes(self) -> Vec<u8> {
                let mut buf = ::itoa::Buffer::new();
                let s = buf.format(self);
                s.as_bytes().to_vec()
            }
        }
    };
}

itoa_into_bytes!(i8);
itoa_into_bytes!(u8);
itoa_into_bytes!(i16);
itoa_into_bytes!(u16);
itoa_into_bytes!(i32);
itoa_into_bytes!(u32);
itoa_into_bytes!(i64);
itoa_into_bytes!(u64);
itoa_into_bytes!(isize);
itoa_into_bytes!(usize);
```

After:
```rust
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
```