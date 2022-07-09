#[cfg(feature = "gnustep-1-7")]
use objc2::Encode;
use objc2::{Encoding, RefEncode};

/// A type used to identify and manage memory zones.
///
/// Zones are ignored on all newer platforms, you should very rarely need to
/// use this.
///
/// See [Apple's documentation](https://developer.apple.com/documentation/foundation/nszone?language=objc).
#[derive(Debug)]
pub struct NSZone {
    _inner: [u8; 0],
}

unsafe impl RefEncode for NSZone {
    #[cfg(feature = "apple")]
    const ENCODING_REF: Encoding<'static> = Encoding::Pointer(&Encoding::Struct("_NSZone", &[]));
    #[cfg(feature = "gnustep-1-7")]
    const ENCODING_REF: Encoding<'static> = Encoding::Pointer(&Encoding::Struct(
        "_NSZone",
        &[
            // Functions
            Encoding::Pointer(&Encoding::Unknown),
            Encoding::Pointer(&Encoding::Unknown),
            Encoding::Pointer(&Encoding::Unknown),
            Encoding::Pointer(&Encoding::Unknown),
            Encoding::Pointer(&Encoding::Unknown),
            Encoding::Pointer(&Encoding::Unknown),
            // Stats
            Encoding::Pointer(&Encoding::Unknown),
            // Zone granularity
            usize::ENCODING,
            // Name of zone
            Encoding::Object,
            // Next zone
            Encoding::Pointer(&Encoding::Struct("_NSZone", &[])),
        ],
    ));
}

#[cfg(test)]
mod tests {
    use crate::NSObject;
    use core::ptr;
    use objc2::msg_send_id;
    use objc2::rc::{Allocated, Id, Owned};

    use super::*;

    #[test]
    #[cfg_attr(
        feature = "gnustep-1-7",
        ignore = "The encoding is not really correct yet!"
    )]
    fn alloc_with_zone() {
        let zone: *const NSZone = ptr::null();
        let _obj: Id<Allocated<NSObject>, Owned> =
            unsafe { msg_send_id![NSObject::class(), allocWithZone: zone].unwrap() };
    }
}
