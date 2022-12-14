use core::fmt;
use core::panic::{RefUnwindSafe, UnwindSafe};

use crate::Foundation::NSProcessInfo;

// SAFETY: The documentation explicitly states:
// > NSProcessInfo is thread-safe in macOS 10.7 and later.
unsafe impl Send for NSProcessInfo {}
unsafe impl Sync for NSProcessInfo {}

impl UnwindSafe for NSProcessInfo {}
impl RefUnwindSafe for NSProcessInfo {}

// TODO: Remove this
impl NSProcessInfo {
    pub fn process_name(&self) -> objc2::rc::Id<crate::Foundation::NSString, objc2::rc::Shared> {
        unsafe { self.processName() }
    }
}

impl fmt::Debug for NSProcessInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("NSProcessInfo")
            .field("processName", &self.process_name())
            .finish_non_exhaustive()
    }
}

#[cfg(test)]
mod tests {
    use alloc::format;

    use super::*;

    #[test]
    fn test_debug() {
        let info = NSProcessInfo::processInfo();
        let expected = format!(
            "NSProcessInfo {{ process_name: {:?}, .. }}",
            info.process_name()
        );
        assert_eq!(format!("{info:?}"), expected);
    }
}
