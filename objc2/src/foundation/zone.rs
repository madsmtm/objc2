use core::fmt;
use core::panic::{RefUnwindSafe, UnwindSafe};

#[cfg(feature = "gnustep-1-7")]
use objc2::Encode;
use objc2::{ffi, Encoding, RefEncode};

/// A type used to identify and manage memory zones.
///
/// Zones are ignored on all newer platforms, you should very rarely need to
/// use this, but may be useful if you need to implement `copyWithZone:` or
/// `allocWithZone:`.
///
/// See [Apple's documentation](https://developer.apple.com/documentation/foundation/nszone?language=objc).
pub struct NSZone {
    // Use `objc_object` to mark the types as !Send, !Sync and UnsafeCell.
    //
    // This works since `objc_object` is a ZST
    _inner: ffi::objc_object,
}

impl fmt::Debug for NSZone {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<NSZone {:p}>", self)
    }
}

// Note: We don't know anything about the internals of `NSZone`, so best not
// to make it `Send` and `Sync` for now.

impl UnwindSafe for NSZone {}
impl RefUnwindSafe for NSZone {}

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
    use crate::foundation::NSObject;
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
