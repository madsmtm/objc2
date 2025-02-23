//! Test that NSMutableCopying returns the same object for CNBoundsPrediction.

#[cfg(feature = "CNObjectTracker")]
#[test]
fn bounds_prediction() {
    use objc2::{msg_send, rc::Retained, ClassType};
    use objc2_cinematic::CNBoundsPrediction;
    use objc2_foundation::NSCopying;

    unsafe {
        let obj = CNBoundsPrediction::new();
        obj.setConfidence(42.0);
        assert_eq!(obj.confidence(), 42.0);

        let obj_copy = obj.copy();
        assert_eq!(obj_copy.class(), CNBoundsPrediction::class());
        assert!(!core::ptr::eq(&*obj, &*obj_copy));

        let obj_mutable_copy: Retained<CNBoundsPrediction> = msg_send![&obj, mutableCopy];
        assert_eq!(obj_mutable_copy.class(), CNBoundsPrediction::class());
        assert!(!core::ptr::eq(&*obj, &*obj_mutable_copy));
    }
}
