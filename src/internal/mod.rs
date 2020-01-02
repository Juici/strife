#[macro_use]
mod macros;

pub mod prelude;

/// From core::fmt::num [impl_Display] macro.
/// 
/// [impl_Display]: https://doc.rust-lang.org/src/core/fmt/num.rs.html#192-238
#[rustfmt::skip]
pub static DEC_DIGITS_LUT: &[u8; 200] =
    b"0001020304050607080910111213141516171819\
      2021222324252627282930313233343536373839\
      4041424344454647484950515253545556575859\
      6061626364656667686970717273747576777879\
      8081828384858687888990919293949596979899";
