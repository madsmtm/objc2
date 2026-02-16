//! Test that using an invalid generic in NSMeasurement fails.
use objc2::AnyThread;
use objc2_foundation::{NSMeasurement, NSObject};

fn main() {
    let mass = NSMeasurement::<NSObject>::initWithDoubleValue_unit(
        NSMeasurement::alloc(),
        10.0,
        &NSObject::new(),
    );
}
