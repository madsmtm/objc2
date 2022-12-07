//! Ensure that implementing `OptionEncode` wrongly results in an error
use objc2::encode::{OptionEncode, Encode, RefEncode, Encoding};

#[repr(transparent)]
struct MyType(usize);

unsafe impl Encode for MyType {
    const ENCODING: Encoding = usize::ENCODING;
}

unsafe impl RefEncode for MyType {
    const ENCODING_REF: Encoding = usize::ENCODING_REF;
}

unsafe impl OptionEncode for MyType {}

fn main() {
    assert_eq!(<Option<MyType>>::ENCODING, MyType::ENCODING);
    assert_eq!(<Option<MyType>>::ENCODING_REF, MyType::ENCODING_REF);

    // TODO: trybuild runs with `cargo check`, which doesn't catch all const
    // errors.
    const TODO: () = {
        panic!("todo");
    };
}
