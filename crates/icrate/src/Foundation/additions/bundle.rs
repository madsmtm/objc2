use core::panic::{RefUnwindSafe, UnwindSafe};

use objc2::rc::{Id, Shared};
use objc2::runtime::Object;

use crate::Foundation::{NSBundle, NSCopying, NSString};

// SAFETY: Bundles are documented as thread-safe.
unsafe impl Sync for NSBundle {}
unsafe impl Send for NSBundle {}

impl UnwindSafe for NSBundle {}
impl RefUnwindSafe for NSBundle {}

impl NSBundle {
    pub fn name(&self) -> Option<Id<NSString, Shared>> {
        let info = self.infoDictionary()?;
        // TODO: Use ns_string!
        let name = info.get(&NSString::from_str("CFBundleName"))?;
        let ptr: *const Object = name;
        let ptr: *const NSString = ptr.cast();
        // SAFETY: TODO
        let name = unsafe { ptr.as_ref().unwrap_unchecked() };
        Some(name.copy())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::format;
    use std::println;

    #[test]
    #[cfg_attr(not(target_os = "macos"), ignore = "varies between platforms")]
    fn try_running_functions() {
        // This is mostly empty since cargo doesn't bundle the application
        // before executing.
        let bundle = NSBundle::mainBundle();
        println!("{bundle:?}");
        assert_eq!(format!("{:?}", bundle.infoDictionary().unwrap()), "{}");
        assert_eq!(bundle.name(), None);
    }
}
