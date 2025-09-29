//! Xcode's default macOS game template, using `objc2` and Metal.
//!
//! This tries to be a fairly 1-to-1 port of Objective-C code to equivalent
//! Rust. A lot of this could probably be written in a more concise way if you
//! were starting from scratch.
//!
//! NOTE: Using a Storyboard outside of Xcode is quite involved, so instead,
//! we set up the entire UI (menubar and window) ourselves.

#![cfg_attr(feature = "unstable-darwin-objc", feature(darwin_objc))]

#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
use objc2::MainThreadMarker;
#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
use objc2_app_kit::NSApplication;

#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
mod app_delegate;
#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
mod game_view_controller;
#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
mod matrix;
#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
mod renderer;
#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
mod shader_types;

#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
fn main() {
    let mtm = MainThreadMarker::new().unwrap();
    objc2::rc::autoreleasepool(|_| {
        let app = NSApplication::sharedApplication(mtm);
        app_delegate::set_application_delegate(&app);
        app.run();
    });
}

#[cfg(not(all(target_os = "macos", target_arch = "aarch64")))]
fn main() {
    panic!("unsupported platform in this example");
}
