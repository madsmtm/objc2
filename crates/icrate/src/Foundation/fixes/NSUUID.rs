#![cfg(feature = "Foundation_NSUUID")]
use objc2::encode::{Encode, Encoding, RefEncode};

use crate::common::*;
use crate::Foundation;

/// The headers describe `initWithUUIDBytes:` and `getUUIDBytes:` as
/// taking `uuid_t`, but something fishy is going on, in reality they
/// expect a reference to these!
///
/// Hence we create this newtype to change the encoding.
#[repr(transparent)]
pub(crate) struct UuidBytes(pub(crate) [u8; 16]);

unsafe impl RefEncode for UuidBytes {
    const ENCODING_REF: Encoding = Encoding::Array(16, &u8::ENCODING);
}

extern_methods!(
    unsafe impl Foundation::NSUUID {
        #[method_id(initWithUUIDBytes:)]
        pub(crate) fn initWithUUIDBytes(
            this: Option<Allocated<Self>>,
            bytes: &UuidBytes,
        ) -> Id<Self>;

        #[method(getUUIDBytes:)]
        pub(crate) fn getUUIDBytes(&self, bytes: &mut UuidBytes);
    }
);
