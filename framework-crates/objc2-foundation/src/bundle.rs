use core::fmt;
use core::panic::{RefUnwindSafe, UnwindSafe};

use crate::NSBundle;

impl UnwindSafe for NSBundle {}
impl RefUnwindSafe for NSBundle {}

impl NSBundle {
    #[cfg(feature = "NSString")]
    #[cfg(feature = "NSDictionary")]
    pub fn name(&self) -> Option<objc2::rc::Retained<crate::NSString>> {
        let info = self.infoDictionary()?;
        let name = info.objectForKey(crate::ns_string!("CFBundleName"))?;
        Some(name.downcast().expect("CFBundleName to be NSString"))
    }
}

impl fmt::Debug for NSBundle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Delegate to NSObject
        (**self).fmt(f)
    }
}
