//! Test that `NSSet` can't be created from types with an unknown stable hash.
use icrate::Foundation::{NSObject, NSSet};

fn main() {
    let _ = NSSet::from_vec(vec![NSObject::new()]);
}
