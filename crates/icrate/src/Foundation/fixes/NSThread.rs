use objc2::extern_methods;

use crate::Foundation::NSThread;

extern_methods!(
    unsafe impl NSThread {
        /// Returns `true` if the thread is the main thread.
        #[method(isMainThread)]
        pub fn isMainThread(&self) -> bool;

        #[method(isMainThread)]
        pub(crate) fn class_isMainThread() -> bool;
    }
);
