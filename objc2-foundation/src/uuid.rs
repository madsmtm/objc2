use core::panic::{RefUnwindSafe, UnwindSafe};

use objc2::rc::{Id, Shared};
use objc2::{msg_send, msg_send_id, Encode, Encoding, RefEncode};

use super::{extern_class, NSCopying, NSObject};

extern_class! {
    /// A universally unique value.
    ///
    /// Can be used to identify types, interfaces, and other items.
    ///
    /// See [Apple's documentation](https://developer.apple.com/documentation/foundation/nsuuid?language=objc).
    #[derive(Debug, PartialEq, Eq, Hash)]
    unsafe pub struct NSUUID: NSObject;
}

/// The headers describe `initWithUUIDBytes:` and `getUUIDBytes:` as
/// taking `uuid_t`, but something fishy is going on, in reality they
/// expect a reference to these!
///
/// Hence we create this newtype to change the encoding.
#[repr(transparent)]
struct UuidBytes([u8; 16]);

unsafe impl RefEncode for UuidBytes {
    const ENCODING_REF: Encoding<'static> = Encoding::Array(16, &u8::ENCODING);
}

// SAFETY: `NSUUID` is immutable.
unsafe impl Sync for NSUUID {}
unsafe impl Send for NSUUID {}

impl UnwindSafe for NSUUID {}
impl RefUnwindSafe for NSUUID {}

impl NSUUID {
    // TODO: `nil` method?

    pub fn new_v4() -> Id<Self, Shared> {
        unsafe { msg_send_id![Self::class(), new].unwrap() }
    }

    pub fn from_bytes(bytes: [u8; 16]) -> Id<Self, Shared> {
        let bytes = UuidBytes(bytes);
        unsafe {
            let obj = msg_send_id![Self::class(), alloc];
            msg_send_id![obj, initWithUUIDBytes: &bytes].unwrap()
        }
    }

    // TODO: `parse_str` using initWithUUIDString:

    pub fn as_bytes(&self) -> [u8; 16] {
        let mut bytes = UuidBytes([0; 16]);
        let _: () = unsafe { msg_send![self, getUUIDBytes: &mut bytes] };
        bytes.0
    }
}

/// Conversion methods to/from `uuid` crate.
#[cfg(feature = "uuid")]
// TODO: Research how stable the `uuid` crate is (or if we'll have to update
// it constantly).
impl NSUUID {}

unsafe impl NSCopying for NSUUID {
    type Ownership = Shared;
    type Output = NSUUID;
}

impl alloc::borrow::ToOwned for NSUUID {
    type Owned = Id<NSUUID, Shared>;
    fn to_owned(&self) -> Self::Owned {
        self.copy()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let uuid1 = NSUUID::new_v4();
        let uuid2 = NSUUID::new_v4();
        assert_ne!(uuid1, uuid2, "Statistically very unlikely");
    }

    #[test]
    fn test_bytes() {
        let uuid = NSUUID::from_bytes([10; 16]);
        assert_eq!(uuid.as_bytes(), [10; 16]);
    }
}
