//! If this ever starts compiling on stable, we should use/expose it!

// Hack to make this work in CI (these features are only enabled when using
// the nightly compiler).
#![cfg(any(
    feature = "unstable-darwin-objc",
    feature = "unstable-autoreleasesafe",
    feature = "unstable-arbitrary-self-types",
    feature = "unstable-coerce-pointee",
))]
#![allow(incomplete_features)] // just a test
#![feature(generic_const_exprs)]

use objc2::Encode;

trait EncodingLen {
    const ENCODING_LEN: usize;
}

impl<T: ?Sized + Encode> EncodingLen for T {
    const ENCODING_LEN: usize = T::ENCODING.str_len();
}

pub trait EncodingExt: Encode
where
    [(); Self::ENCODING_LEN]:,
{
    const ENCODING_ARRAY: [u8; Self::ENCODING_LEN] = Self::ENCODING.str_array();
    const ENCODING_STR: &str = unsafe { core::str::from_utf8_unchecked(&Self::ENCODING_ARRAY) };
}

impl<T: ?Sized + Encode> EncodingExt for T where [(); Self::ENCODING_LEN]: {}

#[test]
fn encoding_str() {
    assert_eq!(<&i32>::ENCODING_STR, "^i");
}
