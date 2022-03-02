use objc2::rc::{Id, Shared};
use objc2::{msg_send, Encode, Encoding, RefEncode};

use super::{NSCopying, NSObject};

object! {
    /// A universally unique value.
    ///
    /// Can be used to identify types, interfaces, and other items.
    ///
    /// See [Apple's documentation](https://developer.apple.com/documentation/foundation/nsuuid?language=objc).
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

// TODO: SAFETY
unsafe impl Sync for NSUUID {}
unsafe impl Send for NSUUID {}

impl NSUUID {
    // TODO: `nil` method?

    pub fn new_v4() -> Id<Self, Shared> {
        unsafe {
            let obj: *mut Self = msg_send![Self::class(), alloc];
            let obj: *mut Self = msg_send![obj, init];
            Id::new(obj).unwrap()
        }
    }

    pub fn from_bytes(bytes: [u8; 16]) -> Id<Self, Shared> {
        let bytes = UuidBytes(bytes);
        unsafe {
            let obj: *mut Self = msg_send![Self::class(), alloc];
            let obj: *mut Self = msg_send![obj, initWithUUIDBytes: &bytes];
            Id::new(obj).unwrap()
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
