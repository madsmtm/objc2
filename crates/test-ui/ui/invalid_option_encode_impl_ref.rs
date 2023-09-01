//! Ensure that implementing `OptionEncode` wrongly results in an error
use objc2::encode::{Encoding, OptionEncode, RefEncode};

#[repr(transparent)]
struct MyType(usize);

unsafe impl RefEncode for MyType {
    const ENCODING_REF: Encoding = usize::ENCODING_REF;
}

unsafe impl OptionEncode for MyType {}

fn main() {
    assert_eq!(<Option<MyType>>::ENCODING_REF, MyType::ENCODING_REF);
}
