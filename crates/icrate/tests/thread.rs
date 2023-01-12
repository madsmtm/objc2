#![cfg(feature = "Foundation_NSThread")]
use icrate::Foundation::{is_main_thread, MainThreadMarker, NSThread};

#[test]
#[cfg_attr(
    feature = "gnustep-1-7",
    ignore = "Retrieving main thread is weirdly broken, only works with --test-threads=1"
)]
fn test_main_thread() {
    let current = NSThread::currentThread();
    let main = NSThread::mainThread();

    assert!(main.isMainThread());

    if main == current {
        assert!(current.isMainThread());
        assert!(is_main_thread());
    } else {
        assert!(!current.isMainThread());
        assert!(!is_main_thread());
    }
}

#[test]
fn test_not_main_thread() {
    let res = std::thread::spawn(|| (is_main_thread(), NSThread::currentThread().isMainThread()))
        .join()
        .unwrap();
    assert_eq!(res, (false, false));
}

#[test]
fn test_main_thread_auto_traits() {
    use std::panic::{RefUnwindSafe, UnwindSafe};

    fn assert_traits<T: Unpin + UnwindSafe + RefUnwindSafe + Sized>() {}

    assert_traits::<MainThreadMarker>()
}

#[test]
#[cfg_attr(
    feature = "gnustep-1-7",
    ignore = "Retrieving main thread is weirdly broken, only works with --test-threads=1"
)]
fn test_debug() {
    let thread = NSThread::mainThread();

    let actual = format!("{thread:?}");
    let expected = [
        // macOS 11
        format!("<NSThread: {thread:p}>{{number = 1, name = (null)}}"),
        format!("<NSThread: {thread:p}>{{number = 1, name = main}}"),
        // macOS 12
        format!("<_NSMainThread: {thread:p}>{{number = 1, name = (null)}}"),
        format!("<_NSMainThread: {thread:p}>{{number = 1, name = main}}"),
    ];
    assert!(
        expected.contains(&actual),
        "Expected one of {expected:?}, got {actual:?}",
    );

    // SAFETY: We don't use the marker for anything other than its Debug
    // impl, so this test doesn't actually need to run on the main thread!
    let marker = unsafe { MainThreadMarker::new_unchecked() };
    assert_eq!(format!("{marker:?}"), "MainThreadMarker");
}
