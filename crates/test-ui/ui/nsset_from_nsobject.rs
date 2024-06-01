//! Test that `NSSet` can't be created from types with an unknown stable hash.
use objc2_foundation::{NSObject, NSSet};

fn main() {
    let _ = NSSet::from_vec(vec![NSObject::new()]);
}
