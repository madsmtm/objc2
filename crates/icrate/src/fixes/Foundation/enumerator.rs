use objc2::encode::Encoding;

use crate::common::*;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NSFastEnumerationState {
    pub state: c_ulong,
    pub itemsPtr: *mut *mut AnyObject,
    pub mutationsPtr: *mut c_ulong,
    pub extra: [c_ulong; 5],
}

impl_encode! {
    NSFastEnumerationState = Encoding::Struct(
        "?",
        &[
            Encoding::C_ULONG,
            Encoding::Pointer(&Encoding::Object),
            Encoding::Pointer(&Encoding::C_ULONG),
            Encoding::Array(5, &Encoding::C_ULONG),
        ],
    );
}
