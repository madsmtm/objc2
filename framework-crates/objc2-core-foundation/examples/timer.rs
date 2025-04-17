//! Creating a `CFRunLoopTimer` and attaching it with a runloop.

use std::sync::Arc;
use std::{cell::Cell, ffi::c_void};

use objc2_core_foundation::{
    kCFAllocatorDefault, kCFRunLoopCommonModes, CFAbsoluteTime, CFAbsoluteTimeGetCurrent, CFIndex,
    CFRetained, CFRunLoop, CFRunLoopTimer, CFRunLoopTimerContext, CFTimeInterval,
};

fn main() {
    let rl = CFRunLoop::main().unwrap();

    let iterations = Cell::new(0);

    let callback = {
        let rl = rl.clone();
        move |timer: &CFRunLoopTimer| {
            println!("Timer called: {}", iterations.get());

            if 10 <= iterations.get() {
                // Remove the timer.
                rl.remove_timer(Some(timer), unsafe { kCFRunLoopCommonModes });

                // Stop the run loop explicitly
                // (the main run-loop won't stop otherwise).
                println!("Stopping run loop");
                rl.stop();
            }

            iterations.set(iterations.get() + 1);
        }
    };

    // SAFETY: The timer is added to a run loop on the same thread.
    let timer = unsafe { create_timer_unchecked(0.0, 0.1, 0, callback) };
    rl.add_timer(Some(&timer), unsafe { kCFRunLoopCommonModes });

    // Add a single-shot timer from another thread.
    let fire_date = CFAbsoluteTimeGetCurrent() + 0.5;
    let timer = create_timer(fire_date, 0.0, 0, |_| {
        println!("Fired one-shot timer after 0.5 seconds");
        // Still runs on the main thread.
        assert_eq!(CFRunLoop::current().unwrap(), CFRunLoop::main().unwrap());
    });
    rl.add_timer(Some(&timer), unsafe { kCFRunLoopCommonModes });

    println!("Starting run loop");
    CFRunLoop::run();
}

/// Create a new `CFRunLoopTimer`.
fn create_timer<F: Fn(&CFRunLoopTimer) + Send + Sync + 'static>(
    fire_date: CFAbsoluteTime,
    interval: CFTimeInterval,
    order: CFIndex,
    callback: F,
) -> CFRetained<CFRunLoopTimer> {
    // SAFETY: The callback is `Send + Sync`.
    unsafe { create_timer_unchecked(fire_date, interval, order, callback) }
}

/// Create a new `CFRunLoopTimer`, without enforcing the callback to be
/// thread-safe.
///
/// # Safety
///
/// The callback must be either `Send` + `Sync`, or the timer must only be
/// added to a run loop that runs on the current thread.
unsafe fn create_timer_unchecked<F: Fn(&CFRunLoopTimer) + 'static>(
    fire_date: CFAbsoluteTime,
    interval: CFTimeInterval,
    order: CFIndex,
    callback: F,
) -> CFRetained<CFRunLoopTimer> {
    // We use an `Arc` here to make sure that the reference-counting of the
    // signal container is atomic (`Retained`/`CFRetained` would be valid
    // alternatives too).
    let callback = Arc::new(callback);

    unsafe extern "C-unwind" fn retain<F>(info: *const c_void) -> *const c_void {
        // SAFETY: The pointer was passed to `CFRunLoopTimerContext.info` below.
        unsafe { Arc::increment_strong_count(info.cast::<F>()) };
        info
    }
    unsafe extern "C-unwind" fn release<F>(info: *const c_void) {
        // SAFETY: The pointer was passed to `CFRunLoopTimerContext.info` below.
        unsafe { Arc::decrement_strong_count(info.cast::<F>()) };
    }

    unsafe extern "C-unwind" fn callout<F: Fn(&CFRunLoopTimer)>(
        timer: *mut CFRunLoopTimer,
        info: *mut c_void,
    ) {
        // SAFETY: The timer is valid for at least the duration of the callback.
        let timer = unsafe { &*timer };

        // SAFETY: The pointer was passed to `CFRunLoopTimerContext.info` below.
        let callback = unsafe { &*info.cast::<F>() };

        // Call the provided closure.
        callback(timer);
    }

    // This is marked `mut` to match the signature of `CFRunLoopTimerCreate`,
    // but the information is copied, and not actually mutated.
    let mut context = CFRunLoopTimerContext {
        version: 0,
        // This pointer is retained by CF on creation.
        info: Arc::as_ptr(&callback) as *mut c_void,
        retain: Some(retain::<F>),
        release: Some(release::<F>),
        copyDescription: None,
    };

    // SAFETY: The retain/release callbacks are thread-safe, and caller
    // upholds that the main callback is used in a thread-safe manner.
    //
    // `F: 'static`, so extending the lifetime of the closure is fine.
    unsafe {
        CFRunLoopTimer::new(
            kCFAllocatorDefault,
            fire_date,
            interval,
            0, // Documentation says to pass 0 for future compat.
            order,
            Some(callout::<F>),
            &mut context,
        )
    }
    .unwrap()
}
