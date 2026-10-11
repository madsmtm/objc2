//! Xcode's default macOS application template, using `objc2`.
//!
//! Using a Storyboard outside of Xcode is quite involved, so instead, we set
//! up the entire UI (menubar and window) ourselves.

#![cfg_attr(not(target_os = "macos"), allow(dead_code, unused))]
#![cfg_attr(feature = "unstable-darwin-objc", feature(darwin_objc))]

use objc2::MainThreadMarker;
#[cfg(target_os = "macos")]
use objc2_app_kit::NSApplication;

#[cfg(target_os = "macos")]
mod app_delegate;
#[cfg(target_os = "macos")]
mod view_controller;

#[cfg(target_os = "macos")]
fn main() {
    let mtm = MainThreadMarker::new().unwrap();
    objc2::rc::autoreleasepool(|_| {
        // Setup code that might create autoreleased objects goes here.

        let app = NSApplication::sharedApplication(mtm);
        app_delegate::set_application_delegate(&app);
        app.run();
    });
}

#[cfg(not(target_os = "macos"))]
fn main() {
    panic!("This example is currently only supported on macOS");
}
