use objc2::msg_send;
use objc2::rc::{Id, Shared};

use crate::{NSObject, NSString};

object! {
    /// A collection of information about the current process.
    ///
    /// See [Apple's documentation](https://developer.apple.com/documentation/foundation/nsprocessinfo?language=objc).
    unsafe pub struct NSProcessInfo: NSObject;

    // TODO: This contains a lot more important functionality!
}

// The documentation explicitly states:
// > NSProcessInfo is thread-safe in macOS 10.7 and later.
unsafe impl Send for NSProcessInfo {}
unsafe impl Sync for NSProcessInfo {}

impl NSProcessInfo {
    pub fn process_info() -> Id<NSProcessInfo, Shared> {
        // currentThread is @property(strong), what does that mean?
        let obj: *mut Self = unsafe { msg_send![Self::class(), processInfo] };
        // TODO: Always available?
        unsafe { Id::retain_autoreleased(obj).unwrap() }
    }

    pub fn process_name(&self) -> Id<NSString, Shared> {
        let obj: *mut NSString = unsafe { msg_send![Self::class(), processName] };
        unsafe { Id::retain_autoreleased(obj).unwrap() }
    }
}
