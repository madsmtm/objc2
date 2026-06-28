use core::ffi::c_short;

#[cfg(feature = "objc2")]
use objc2::encode::{Encode, Encoding, RefEncode};

/// [Apple's documentation](https://developer.apple.com/documentation/coreservices/qelem?language=objc)
#[repr(C, packed(2))]
#[derive(Clone, Copy, Debug, PartialEq)]
#[allow(non_snake_case)]
pub struct QElem {
    pub qLink: *mut QElem,
    pub qType: c_short,
    pub qData: [c_short; 1],
}

#[cfg(feature = "objc2")]
unsafe impl Encode for QElem {
    const ENCODING: Encoding = Encoding::Struct(
        "QElem",
        &[
            Encoding::Struct("QElem", &[]),
            <c_short>::ENCODING,
            <[c_short; 1]>::ENCODING,
        ],
    );
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for QElem {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
