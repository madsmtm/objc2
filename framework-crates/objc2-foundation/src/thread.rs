use core::fmt;
use core::panic::{RefUnwindSafe, UnwindSafe};

use crate::Foundation::NSThread;

use objc2::MainThreadMarker;

unsafe impl Send for NSThread {}
unsafe impl Sync for NSThread {}

impl UnwindSafe for NSThread {}
impl RefUnwindSafe for NSThread {}

impl fmt::Debug for NSThread {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Use -[NSThread description] since that includes the thread number
        let obj: &crate::Foundation::NSObject = self;
        fmt::Debug::fmt(obj, f)
    }
}

/// Whether the application is multithreaded according to Cocoa.
pub fn is_multi_threaded() -> bool {
    NSThread::isMultiThreaded()
}

/// Whether the current thread is the main thread.
///
/// Deprecated. Prefer `MainThreadMarker::new().is_some()` or
/// `NSThread::isMainThread_class()` instead.
#[deprecated = "use `objc2::MainThreadMarker::new().is_some()`"]
pub fn is_main_thread() -> bool {
    MainThreadMarker::new().is_some()
}

#[allow(unused)]
fn make_multithreaded() {
    let thread = unsafe { NSThread::new() };
    unsafe { thread.start() };
    // Don't bother waiting for it to complete!
}
