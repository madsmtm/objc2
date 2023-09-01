//! Test that compilation fails when the struct name is invalid.
//!
//! Ideally, this should be tested by `trybuild`, but it doesn't work at the
//! moment (`cargo check` doesn't catch the error).
use objc2::{Encode, Encoding};

struct X;

unsafe impl Encode for X {
    const ENCODING: Encoding<'static> = Encoding::Struct("-", &[]);
}

fn main() {
    let _ = X::ENCODING_CSTR;
}
