use crate::Foundation::generated::NSObject::*;
use crate::Foundation::generated::NSUnit::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, msg_send, msg_send_id, ClassType};
__inner_extern_class!(
    #[derive(Debug)]
    pub struct NSMeasurement<UnitType: Message>;
    unsafe impl<UnitType: Message> ClassType for NSMeasurement<UnitType> {
        type Super = NSObject;
    }
);
impl<UnitType: Message> NSMeasurement<UnitType> {
    pub unsafe fn unit(&self) -> Id<UnitType, Shared> {
        msg_send_id![self, unit]
    }
    pub unsafe fn doubleValue(&self) -> c_double {
        msg_send![self, doubleValue]
    }
    pub unsafe fn init(&self) -> Id<Self, Shared> {
        msg_send_id![self, init]
    }
    pub unsafe fn initWithDoubleValue_unit(
        &self,
        doubleValue: c_double,
        unit: &UnitType,
    ) -> Id<Self, Shared> {
        msg_send_id![self, initWithDoubleValue: doubleValue, unit: unit]
    }
    pub unsafe fn canBeConvertedToUnit(&self, unit: &NSUnit) -> bool {
        msg_send![self, canBeConvertedToUnit: unit]
    }
    pub unsafe fn measurementByConvertingToUnit(&self, unit: &NSUnit) -> Id<NSMeasurement, Shared> {
        msg_send_id![self, measurementByConvertingToUnit: unit]
    }
    pub unsafe fn measurementByAddingMeasurement(
        &self,
        measurement: &NSMeasurement<UnitType>,
    ) -> Id<NSMeasurement<UnitType>, Shared> {
        msg_send_id![self, measurementByAddingMeasurement: measurement]
    }
    pub unsafe fn measurementBySubtractingMeasurement(
        &self,
        measurement: &NSMeasurement<UnitType>,
    ) -> Id<NSMeasurement<UnitType>, Shared> {
        msg_send_id![self, measurementBySubtractingMeasurement: measurement]
    }
}
