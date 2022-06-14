use objc2::rc::{Id, Shared};
use objc2::{msg_send, msg_send_bool, msg_send_id};

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
    pub fn current() -> Id<Self, Shared> {
        // TODO: currentThread is @property(strong), what does that mean?
        // TODO: Always available?
        unsafe { msg_send_id![Self::class(), currentThread].unwrap() }
    }

    /// Returns the [`NSThread`] object representing the main thread.
    pub fn main() -> Id<NSThread, Shared> {
        // TODO: mainThread is @property(strong), what does that mean?
        // The main thread static may not have been initialized
        // This can at least fail in GNUStep!
        unsafe { msg_send_id![Self::class(), mainThread] }.expect("Could not retrieve main thread.")
    }

    /// Returns `true` if the thread is the main thread.
    pub fn is_main(&self) -> bool {
        unsafe { msg_send_bool![self, isMainThread] }
    }

    /// The name of the thread.
    pub fn name(&self) -> Option<Id<NSString, Shared>> {
        unsafe { msg_send_id![self, name] }
    }

    fn new() -> Id<Self, Shared> {
        unsafe { msg_send_id![Self::class(), new] }.unwrap()
    }

    fn start(&self) {
        unsafe { msg_send![self, start] }
    }
}

/// Whether the application is multithreaded according to Cocoa.
pub fn is_multi_threaded() -> bool {
    unsafe { msg_send_bool![NSThread::class(), isMultiThreaded] }
}

/// Whether the current thread is the main thread.
pub fn is_main_thread() -> bool {
    unsafe { msg_send_bool![NSThread::class(), isMainThread] }
}

#[allow(unused)]
fn make_multithreaded() {
    let thread = NSThread::new();
    thread.start();
    // Don't bother waiting for it to complete!
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg_attr(
        feature = "gnustep-1-7",
        ignore = "Retrieving main thread is weirdly broken, only works with --test-threads=1"
    )]
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
