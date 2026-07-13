#![cfg(feature = "HKUnit")]
#![cfg(feature = "HKAppleWalkingSteadinessClassification")]
#![cfg(feature = "HKQuantity")]

use objc2_health_kit::{HKAppleWalkingSteadinessClassification, HKQuantity, HKUnit};

#[test]
fn test_fails() {
    let unit = unsafe { HKUnit::meterUnit() };
    let quantity = unsafe { HKQuantity::quantityWithUnit_doubleValue(&unit, 2.0) };

    let mut classification = HKAppleWalkingSteadinessClassification(42); // Invalid value
    let mut err = None;
    let status = unsafe {
        HKAppleWalkingSteadinessClassification::for_quantity(
            &quantity,
            &mut classification,
            Some(&mut err),
        )
    };
    if status {
        assert_ne!(classification.0, 42);
    } else {
        // Ignore error
        let _ = err.unwrap();
    }
}
