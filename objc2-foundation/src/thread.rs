use objc2::msg_send;
use objc2::rc::{Id, Shared};
use objc2::runtime::Bool;

use crate::{NSObject, NSString};

object! {
    /// A thread of execution.
    ///
    /// See [Apple's documentation](https://developer.apple.com/documentation/foundation/nsthread?language=objc).
    unsafe pub struct NSThread: NSObject;
}

unsafe impl Send for NSThread {}
unsafe impl Sync for NSThread {}

impl NSThread {
    /// Returns the [`NSThread`] object representing the current thread.
    pub fn current() -> Id<NSThread, Shared> {
        // TODO: currentThread is @property(strong), what does that mean?
        let obj: *mut Self = unsafe { msg_send![Self::class(), currentThread] };
        // TODO: Always available?
        unsafe { Id::retain_autoreleased(obj).unwrap() }
    }

    /// Returns the [`NSThread`] object representing the main thread.
    pub fn main() -> Id<NSThread, Shared> {
        // TODO: mainThread is @property(strong), what does that mean?
        let obj: *mut Self = unsafe { msg_send![Self::class(), mainThread] };
        // The main thread static may not have been initialized
        // This can at least fail in GNUStep!
        unsafe { Id::retain_autoreleased(obj).expect("Could not retrieve main thread.") }
    }

    /// Returns `true` if the thread is the main thread.
    pub fn is_main(&self) -> bool {
        let res: Bool = unsafe { msg_send![self, isMainThread] };
        res.is_true()
    }

    /// The name of the thread.
    pub fn name(&self) -> Option<Id<NSString, Shared>> {
        let obj: *mut NSString = unsafe { msg_send![self, name] };
        unsafe { Id::retain_autoreleased(obj) }
    }
}

/// Whether the application is multithreaded according to Cocoa.
pub fn is_multi_threaded() -> bool {
    let res: Bool = unsafe { msg_send![NSThread::class(), isMultiThreaded] };
    res.is_true()
}

/// Whether the current thread is the main thread.
pub fn is_main_thread() -> bool {
    let res: Bool = unsafe { msg_send![NSThread::class(), isMainThread] };
    res.is_true()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg_attr(gnustep, should_panic = "Could not retrieve main thread")]
    fn test_main_thread() {
        let current = NSThread::current();
        let main = NSThread::main();

        assert!(main.is_main());

        if main == current {
            assert!(current.is_main());
            assert!(is_main_thread());
        } else {
            assert!(!current.is_main());
            assert!(!is_main_thread());
        }
    }

    #[test]
    fn test_not_main_thread() {
        let res = std::thread::spawn(|| (is_main_thread(), NSThread::current().is_main()))
            .join()
            .unwrap();
        assert_eq!(res, (false, false));
    }
}
