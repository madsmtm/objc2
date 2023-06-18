#![cfg(feature = "Foundation_NSBundle")]
use core::panic::{RefUnwindSafe, UnwindSafe};

use crate::common::*;
use crate::Foundation::{self, NSBundle};

// SAFETY: Bundles are documented as thread-safe.
unsafe impl Sync for NSBundle {}
unsafe impl Send for NSBundle {}

impl UnwindSafe for NSBundle {}
impl RefUnwindSafe for NSBundle {}

impl NSBundle {
    #[cfg(feature = "Foundation_NSString")]
    #[cfg(feature = "Foundation_NSDictionary")]
    pub fn name(&self) -> Option<Id<Foundation::NSString>> {
        use Foundation::{NSCopying, NSString};

        let info = self.infoDictionary()?;
        // TODO: Use ns_string!
        let name = info.get(&NSString::from_str("CFBundleName"))?;
        let ptr: *const AnyObject = name;
        let ptr: *const NSString = ptr.cast();
        // SAFETY: TODO
        let name = unsafe { ptr.as_ref().unwrap_unchecked() };
        Some(name.copy())
    }
}
