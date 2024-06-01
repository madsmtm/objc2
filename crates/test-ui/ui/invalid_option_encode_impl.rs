//! Ensure that implementing `OptionEncode` wrongly results in an error
use objc2::encode::{Encode, Encoding, OptionEncode};

#[repr(transparent)]
struct MyType(usize);

unsafe impl Encode for MyType {
    const ENCODING: Encoding = usize::ENCODING;
}

unsafe impl OptionEncode for MyType {}

fn main() {
    assert_eq!(<Option<MyType>>::ENCODING, MyType::ENCODING);
}
