//! Test that compilation fails when the encoding is too long.
//!
//! Ideally, this should be tested by `trybuild`, but it doesn't work at the
//! moment (`cargo check` doesn't catch the error).
use objc2::{Encode, Encoding};

struct X;

const S: &str = unsafe { std::str::from_utf8_unchecked(&[b'a'; 1020]) };

unsafe impl Encode for X {
    const ENCODING: Encoding<'static> = Encoding::Struct(S, &[]);
}

fn main() {
    let _ = X::ENCODING_CSTR;
}
