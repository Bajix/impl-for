![License](https://img.shields.io/badge/license-MIT-green.svg)
[![Cargo](https://img.shields.io/crates/v/impl-for.svg)](https://crates.io/crates/impl-for)
[![Documentation](https://docs.rs/impl-for/badge.svg)](https://docs.rs/impl-for)

Macros for repeating implementations with type substitutions.

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
#[impl_for_each(i8, u8, i16, u16, i32, u32, i64, isize, usize)]
impl IntoBytes for T {
    fn into_bytes(self) -> Vec<u8> {
        let mut buf = ::itoa::Buffer::new();
        let s = buf.format(self);
        s.as_bytes().to_vec()
    }
}
```