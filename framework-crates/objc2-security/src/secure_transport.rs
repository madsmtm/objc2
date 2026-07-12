use core::mem::ManuallyDrop;

use objc2_core_foundation::CFRetained;

use crate::{errSecSuccess, OSStatus, SSLContext};

impl SSLContext {
    #[doc(alias = "SSLDisposeContext")]
    #[deprecated = "No longer supported. Use Network.framework."]
    #[inline]
    pub fn dispose(this: CFRetained<Self>) -> OSStatus {
        let this = ManuallyDrop::new(this);
        #[allow(deprecated)]
        let res = unsafe { Self::__dispose(CFRetained::as_ptr(&this)) };
        // `SSLDisposeContext` releases if successful. So only if it isn't do
        // we want to deallocate.
        if res != errSecSuccess {
            let _ = ManuallyDrop::into_inner(this);
        }
        res
    }
}
