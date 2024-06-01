#![allow(deprecated)]
#![cfg(feature = "MLCTensor")]
#![cfg(feature = "MLCTypes")]
use objc2_foundation::{NSArray, NSNumber};
use objc2_ml_compute::{MLCDataType, MLCTensor};

#[test]
fn new() {
    unsafe {
        MLCTensor::tensorWithShape_dataType(
            &NSArray::from_vec(vec![NSNumber::new_u8(4), NSNumber::new_u8(4)]),
            MLCDataType::Float32,
        );
    }
}
