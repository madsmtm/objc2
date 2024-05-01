#![cfg(feature = "MTLDevice")]
use objc2::rc::Id;
use objc2_metal::{MTLCopyAllDevices, MTLCreateSystemDefaultDevice};

#[link(name = "CoreGraphics", kind = "framework")]
extern "C" {}

#[test]
#[ignore = "doesn't work in CI"]
fn test_create_default() {
    let _ = unsafe { Id::from_raw(MTLCreateSystemDefaultDevice()).unwrap() };
}

#[test]
fn get_all() {
    let _ = unsafe { Id::from_raw(MTLCopyAllDevices().as_ptr()).unwrap() };
}
