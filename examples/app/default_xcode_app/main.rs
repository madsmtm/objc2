//! Xcode's default macOS application template, using `objc2`.
//!
//! Using a Storyboard outside of Xcode is quite involved, so instead, we set
//! up the entire UI (menubar and window) ourselves.

use objc2::MainThreadMarker;
use objc2_app_kit::NSApplication;

mod app_delegate;
mod view_controller;

fn main() {
    let mtm = MainThreadMarker::new().unwrap();
    objc2::rc::autoreleasepool(|_| {
        // Setup code that might create autoreleased objects goes here.

        let app = NSApplication::sharedApplication(mtm);
        app_delegate::set_application_delegate(&app);
        app.run();
    });
}
