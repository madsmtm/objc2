use objc2::msg_send_id;
use objc2::rc::{Id, Shared};

use crate::{NSObject, NSString};

extern_class! {
    /// A collection of information about the current process.
    ///
    /// See [Apple's documentation](https://developer.apple.com/documentation/foundation/nsprocessinfo?language=objc).
    #[derive(Debug, PartialEq, Eq, Hash)]
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
        // TODO: Always available?
        unsafe { msg_send_id![Self::class(), processInfo].unwrap() }
    }

    pub fn process_name(&self) -> Id<NSString, Shared> {
        unsafe { msg_send_id![self, processName].unwrap() }
    }
}
