#![cfg(feature = "WKApplicationMain")]
use core::ffi::{c_char, c_int};
use core::ptr::NonNull;

use objc2::MainThreadMarker;
use objc2_foundation::NSString;

use crate::WKApplication;

// These functions are in crt_externs.h.
extern "C" {
    fn _NSGetArgc() -> *mut c_int;
    fn _NSGetArgv() -> *mut *mut *mut c_char;
}

impl WKApplication {
    /// The entry point to WatchKit applications.
    ///
    /// Creates the application object and the application delegate, and sets
    /// up the app’s event cycle.
    ///
    /// See [Apple's documentation][apple-doc] for more details.
    ///
    /// [apple-doc]: https://developer.apple.com/documentation/appkit/nsapplicationmain(_:_:)
    #[doc(alias = "WKApplicationMain")]
    pub fn main(application_delegate_class_name: Option<&NSString>, mtm: MainThreadMarker) -> ! {
        // WKApplicationMain must be called on the main thread.
        let _ = mtm;

        // NOTE: `WKApplicationMain` ignores `argc` and `argv`, so we choose
        // to not expose those in our API.
        // We pass correct values anyhow though, just to be certain.
        let argc = unsafe { *_NSGetArgc() };
        let argv = unsafe { NonNull::new(*_NSGetArgv()).unwrap().cast() };

        // SAFETY: `argc` and `argv` are correct.
        // `WKApplicationMain` is safely re-entrant, just weird to do so.
        let ret = unsafe { Self::__main(argc, argv, application_delegate_class_name) };

        // WKApplicationMain is documented to never return, so whatever we do
        // here is just for show really.
        #[cfg(feature = "std")]
        {
            std::process::exit(ret as i32)
        }
        #[cfg(not(feature = "std"))]
        {
            unreachable!("WKApplicationMain should not have returned")
        }
    }
}
