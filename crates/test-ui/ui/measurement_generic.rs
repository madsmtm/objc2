//! Test that using an invalid generic in NSMeasurement fails.
use objc2_foundation::{NSMeasurement, NSObject};

fn main() {
    let _ = unsafe { NSMeasurement::<NSObject>::new() };
}
