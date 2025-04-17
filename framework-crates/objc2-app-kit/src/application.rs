#![cfg(feature = "NSResponder")]
use core::ffi::{c_char, c_int};
use core::ptr::NonNull;

use objc2::rc::Retained;
use objc2::MainThreadMarker;

use crate::NSApplication;

pub fn NSApp(mtm: MainThreadMarker) -> Retained<NSApplication> {
    // TODO: Use the `NSApp` static
    NSApplication::sharedApplication(mtm)
}

// These functions are in crt_externs.h.
extern "C" {
    fn _NSGetArgc() -> *mut c_int;
    fn _NSGetArgv() -> *mut *mut *mut c_char;
}

impl NSApplication {
    /// An entry point to AppKit applications.
    ///
    /// See [Apple's documentation][apple-doc] for more details.
    ///
    /// [apple-doc]: https://developer.apple.com/documentation/appkit/nsapplicationmain(_:_:)
    #[doc(alias = "NSApplicationMain")]
    pub fn main(mtm: MainThreadMarker) -> ! {
        // NSApplicationMain must be called on the main thread.
        let _ = mtm;

        // NOTE: `NSApplicationMain` is explicitly documented to ignore the
        // `argc` and `argv` arguments, so we choose to not expose those in
        // our API.
        // We pass correct values anyhow though, just to be certain.
        let argc = unsafe { *_NSGetArgc() };
        let argv = unsafe { NonNull::new(*_NSGetArgv()).unwrap().cast() };

        // SAFETY: `argc` and `argv` are correct.
        // `NSApplicationMain` is safely re-entrant, just weird to do so.
        let ret = unsafe { Self::__main(argc, argv) };

        // NSApplicationMain is documented to never return, so whatever we do
        // here is just for show really.
        #[cfg(feature = "std")]
        {
            std::process::exit(ret as i32)
        }
        #[cfg(not(feature = "std"))]
        {
            unreachable!("NSApplicationMain should not have returned")
        }
    }
}
