//! Test output of creating `NSArray` from a non-`Message` type.
use objc2::rc::Id;
use objc2_foundation::{NSArray, NSObject};

fn main() {
    let _: Id<NSArray<i32>> = NSArray::new();
    let _: Id<NSArray<Id<NSObject>>> = NSArray::from_slice(&[&NSObject::new()]);
}
