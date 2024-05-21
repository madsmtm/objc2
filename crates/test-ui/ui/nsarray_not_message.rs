//! Test output of creating `NSArray` from a non-`Message` type.
use objc2::rc::Retained;
use objc2_foundation::{NSArray, NSObject};

fn main() {
    let _: Retained<NSArray<i32>> = NSArray::new();
    let _: Retained<NSArray<Retained<NSObject>>> = NSArray::from_slice(&[&NSObject::new()]);
}
