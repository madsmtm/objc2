//! A re-implementation of Xcode's Developer Documentation viewer.
//!
//! This uses the implementation in `crates/apple-doc` to get the data from
//! Xcode's directories.
//!
//! This is unfinished, but should give a bit of an idea of what's possible.
//!
//! TODO:
//! - https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/CocoaBindings/CocoaBindings.html
//! - https://developer.apple.com/documentation/appkit/navigating-hierarchical-data-using-outline-and-split-views?language=objc
#![allow(non_snake_case)]

#[cfg(target_os = "macos")]
mod app_delegate;
#[cfg(target_os = "macos")]
mod detail_view_controller;
#[cfg(target_os = "macos")]
mod navigator;
#[cfg(target_os = "macos")]
mod split_view_controller;

#[cfg(target_os = "macos")]
fn main() {
    let mtm = objc2::MainThreadMarker::new().unwrap();
    objc2::rc::autoreleasepool(|_| {
        let app = objc2_app_kit::NSApplication::sharedApplication(mtm);
        app_delegate::set_application_delegate(&app);
        app.run();
    });
}

#[cfg(not(target_os = "macos"))]
fn main() {
    panic!("unsupported platform in this example");
}
