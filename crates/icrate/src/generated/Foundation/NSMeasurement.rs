#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, msg_send, msg_send_id, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSMeasurement;
    unsafe impl ClassType for NSMeasurement {
        type Super = NSObject;
    }
);
impl NSMeasurement {
    pub unsafe fn init(&self) -> Id<Self, Shared> {
        msg_send_id![self, init]
    }
    pub unsafe fn initWithDoubleValue_unit(
        &self,
        doubleValue: c_double,
        unit: UnitType,
    ) -> Id<Self, Shared> {
        msg_send_id![self, initWithDoubleValue: doubleValue, unit: unit]
    }
    pub unsafe fn canBeConvertedToUnit(&self, unit: &NSUnit) -> bool {
        msg_send![self, canBeConvertedToUnit: unit]
    }
    pub unsafe fn measurementByConvertingToUnit(&self, unit: &NSUnit) -> Id<NSMeasurement, Shared> {
        msg_send_id![self, measurementByConvertingToUnit: unit]
    }
    pub unsafe fn measurementByAddingMeasurement(&self, measurement: TodoGenerics) -> TodoGenerics {
        msg_send![self, measurementByAddingMeasurement: measurement]
    }
    pub unsafe fn measurementBySubtractingMeasurement(
        &self,
        measurement: TodoGenerics,
    ) -> TodoGenerics {
        msg_send![self, measurementBySubtractingMeasurement: measurement]
    }
    pub unsafe fn unit(&self) -> UnitType {
        msg_send![self, unit]
    }
    pub unsafe fn doubleValue(&self) -> c_double {
        msg_send![self, doubleValue]
    }
}
