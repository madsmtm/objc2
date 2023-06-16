use objc2::encode::{Encoding, RefEncode};

// We choose in this case to represent `NSDecimal` as an opaque struct because
// we don't know much about the internals.
//
// Therefore we do not implement `Encode`, but when implementing `RefEncode`,
// the type-encoding still has to be correct.
#[repr(C)]
struct NSDecimal {
    // Note: This should be an [extern type][rfc-1861] instead, when that
    // becomes possible, for now we use this as a workaround.
    //
    // [rfc-1861]: https://rust-lang.github.io/rfcs/1861-extern-types.html
    _priv: [u8; 0],
}

// SAFETY: `&NSDecimal` is a valid pointer, and the encoding is correct.
unsafe impl RefEncode for NSDecimal {
    // Running `@encode` on `NSDecimal*` on my 64-bit system gives
    // `^{?=cCCC[38C]}`. On other systems it may be something else!
    const ENCODING_REF: Encoding = Encoding::Pointer(&Encoding::Struct(
        "?",
        &[
            Encoding::Char,
            Encoding::UChar,
            Encoding::UChar,
            Encoding::UChar,
            Encoding::Array(38, &Encoding::UChar),
        ],
    ));
}

fn main() {
    assert!(NSDecimal::ENCODING_REF.equivalent_to_str("^{?=cCCC[38C]}"));
    // Does not compile:
    // println!("{:?}", NSDecimal::ENCODING);
}
