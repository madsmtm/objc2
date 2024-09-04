use core::fmt;
use core::panic::{RefUnwindSafe, UnwindSafe};

use crate::Foundation::NSBundle;

impl UnwindSafe for NSBundle {}
impl RefUnwindSafe for NSBundle {}

impl NSBundle {
    #[cfg(feature = "NSString")]
    #[cfg(feature = "NSDictionary")]
    pub fn name(&self) -> Option<objc2::rc::Retained<crate::Foundation::NSString>> {
        use crate::{ns_string, Foundation::NSString};
        use objc2::rc::Retained;

        let info = self.infoDictionary()?;
        let name = info.objectForKey(ns_string!("CFBundleName"))?;
        let name: Retained<NSString> = unsafe { Retained::cast(name) };
        Some(name)
    }
}

impl fmt::Debug for NSBundle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Delegate to NSObject
        (**self).fmt(f)
    }
}
