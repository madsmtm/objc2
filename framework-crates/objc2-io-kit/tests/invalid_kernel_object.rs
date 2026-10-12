#![cfg(feature = "libc")]
use objc2_io_kit::{IOObjectRelease, IOObjectRetain};

#[test]
fn release() {
    assert_ne!(IOObjectRelease(100), 0);
}

#[test]
fn retain() {
    assert_ne!(IOObjectRetain(100), 0);
}
