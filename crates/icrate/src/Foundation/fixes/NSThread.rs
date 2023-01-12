use crate::common::*;
use crate::Foundation;

extern_methods!(
    #[cfg(feature = "Foundation_NSThread")]
    unsafe impl Foundation::NSThread {
        /// Returns `true` if the thread is the main thread.
        #[method(isMainThread)]
        pub fn isMainThread(&self) -> bool;

        #[method(isMainThread)]
        pub(crate) fn class_isMainThread() -> bool;
    }
);
