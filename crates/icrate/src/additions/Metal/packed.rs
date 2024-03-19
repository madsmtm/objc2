use core::fmt;
use std::os::raw::c_float;

#[cfg(feature = "objc2")]
use objc2::encode::{Encode, Encoding, RefEncode};

// TODO: Investigate the need for this helper struct?
//
// I'm fairly sure that just replacing `MTLPackedFloat3` with this struct has
// the same effect in Rust as doing the trick with putting it in a union?
//
// Or is the intention actually to create something similar to `#[packed]`?
//
// <https://users.rust-lang.org/t/mapping-nested-packed-union-from-c-to-rust/87334>

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct __MTLPackedFloat3 {
    x: c_float,
    y: c_float,
    z: c_float,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union MTLPackedFloat3 {
    struct_: __MTLPackedFloat3,
    elements: [c_float; 3],
}

impl PartialEq for MTLPackedFloat3 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { self.struct_ == other.struct_ }
    }
}

impl fmt::Debug for MTLPackedFloat3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unsafe { self.struct_.fmt(f) }
    }
}

#[cfg(feature = "objc2")]
unsafe impl Encode for MTLPackedFloat3 {
    const ENCODING: Encoding = Encoding::Struct(
        "_MTLPackedFloat3",
        &[Encoding::Union(
            "?",
            &[
                Encoding::Struct(
                    "?",
                    &[c_float::ENCODING, c_float::ENCODING, c_float::ENCODING],
                ),
                Encoding::Array(3, &c_float::ENCODING),
            ],
        )],
    );
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for MTLPackedFloat3 {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

#[cfg(test)]
mod tests {
    use alloc::string::ToString;
    use objc2::encode::Encode;

    use crate::Metal::MTLPackedFloat4x3;

    #[test]
    fn test_packed_float() {
        assert_eq!(
            MTLPackedFloat4x3::ENCODING.to_string(),
            "{_MTLPackedFloat4x3=[4{_MTLPackedFloat3=(?={?=fff}[3f])}]}",
        );
    }
}
