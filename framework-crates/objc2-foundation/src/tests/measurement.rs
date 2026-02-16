#![cfg(all(feature = "NSMeasurement", feature = "NSUnit"))]

use crate::{NSMeasurement, NSUnitMass, NSUnitPower};
use objc2::AnyThread;

#[test]
fn create() {
    let mass = NSMeasurement::<NSUnitMass>::initWithDoubleValue_unit(
        NSMeasurement::alloc(),
        10.0,
        &NSUnitMass::grams(),
    );
    let _power = unsafe { mass.cast_unchecked::<NSUnitPower>() };
}
