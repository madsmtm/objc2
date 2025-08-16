//! Xcode's default macOS game template, using `objc2` and Metal.
//!
//! This tries to be a fairly 1-to-1 port of Objective-C code to equivalent
//! Rust. A lot of this could probably be written in a more concise way if you
//! were starting from scratch.
//!
//! NOTE: Using a Storyboard outside of Xcode is quite involved, so instead,
//! we set up the entire UI (menubar and window) ourselves.
#![cfg(all(target_os = "macos", target_arch = "aarch64"))]

use objc2::MainThreadMarker;
use objc2_app_kit::NSApplication;

mod app_delegate;
mod game_view_controller;
mod matrix;
mod renderer;
mod shader_types;

fn main() {
    let mtm = MainThreadMarker::new().unwrap();
    objc2::rc::autoreleasepool(|_| {
        let app = NSApplication::sharedApplication(mtm);
        app_delegate::set_application_delegate(&app);
        app.run();
    });
}
