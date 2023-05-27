use objc2::encode::{Encode, Encoding, RefEncode};
use objc2::runtime::Object;

#[repr(transparent)]
struct NSString {
    // `NSString` has the same layout / works the same as `Object`.
    _priv: Object,
}

// We don't know the size of NSString, so we can only hold pointers to it.
//
// SAFETY: The string is `repr(transparent)` over `Object`.
unsafe impl RefEncode for NSString {
    const ENCODING_REF: Encoding = Encoding::Object;
}

fn main() {
    // The `RefEncode` implementation provide an `Encode` implementation for
    // pointers to the object.
    assert_eq!(<*const NSString>::ENCODING, Encoding::Object);
    assert_eq!(<*mut NSString>::ENCODING, Encoding::Object);
    assert_eq!(<&NSString>::ENCODING, Encoding::Object);
    assert_eq!(<&mut NSString>::ENCODING, Encoding::Object);
    assert_eq!(<Option<&NSString>>::ENCODING, Encoding::Object);
    assert_eq!(<Option<&mut NSString>>::ENCODING, Encoding::Object);
}
