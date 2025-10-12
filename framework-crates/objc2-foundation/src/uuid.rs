use core::fmt;
use core::panic::{RefUnwindSafe, UnwindSafe};

use objc2::rc::Retained;
use objc2::runtime::NSObject;
use objc2::{msg_send, AnyThread};

use crate::{util, NSUUID};

impl UnwindSafe for NSUUID {}
impl RefUnwindSafe for NSUUID {}

impl NSUUID {
    /// The 'nil UUID'.
    pub fn nil() -> Retained<Self> {
        Self::initWithUUIDBytes(Self::alloc(), None)
    }

    /// Create a new `NSUUID` from the given bytes.
    ///
    ///
    /// # Example
    ///
    /// Create a new `NSUUID` from the `uuid` crate.
    ///
    /// ```ignore
    /// use uuid::Uuid;
    /// use objc2_foundation::NSUUID;
    ///
    /// let uuid: Uuid;
    /// # uuid = todo!();
    /// let obj = NSUUID::from_bytes(uuid.into_bytes());
    /// assert_eq!(obj.as_bytes(), uuid.into_bytes());
    /// ```
    pub fn from_bytes(bytes: [u8; 16]) -> Retained<Self> {
        Self::initWithUUIDBytes(Self::alloc(), Some(&bytes))
    }

    #[cfg(feature = "NSString")]
    pub fn from_string(string: &crate::NSString) -> Option<Retained<Self>> {
        Self::initWithUUIDString(Self::alloc(), string)
    }

    /// Convert the UUID to an array.
    pub fn as_bytes(&self) -> [u8; 16] {
        let mut bytes = [0; 16];
        self.getUUIDBytes(&mut bytes);
        bytes
    }
}

impl fmt::Display for NSUUID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string: Retained<NSObject> = unsafe { msg_send![self, UUIDString] };
        // SAFETY: `UUIDString` returns `NSString`.
        unsafe { util::display_string(&string, f) }
    }
}

impl fmt::Debug for NSUUID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // The `uuid` crate does `Debug` and `Display` equally, and so do we.

        let string: Retained<NSObject> = unsafe { msg_send![self, UUIDString] };
        // SAFETY: `UUIDString` returns `NSString`.
        unsafe { util::display_string(&string, f) }
    }
}

// UUID `compare:` is broken for some reason?

// impl PartialOrd for NSUUID {
//     #[inline]
//     fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
//         Some(self.cmp(other))
//     }
// }

// impl Ord for NSUUID {
//     fn cmp(&self, other: &Self) -> cmp::Ordering {
//         let res: NSComparisonResult = unsafe { msg_send![self, compare: other] };
//         res.into()
//     }
// }
