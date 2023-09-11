//! Test output of creating `NSArray` from a non-`Message` type.
use icrate::Foundation::{NSArray, NSObject};
use objc2::rc::Id;

fn main() {
    let _: Id<NSArray<i32>> = NSArray::new();
    let _: Id<NSArray<Id<NSObject>>> = NSArray::from_slice(&[&NSObject::new()]);
}
