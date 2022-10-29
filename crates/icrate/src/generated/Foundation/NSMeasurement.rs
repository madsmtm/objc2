#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
__inner_extern_class!(
    #[derive(Debug)]
    pub struct NSMeasurement<UnitType: Message>;
    unsafe impl<UnitType: Message> ClassType for NSMeasurement<UnitType> {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl<UnitType: Message> NSMeasurement<UnitType> {
        #[method_id(unit)]
        pub unsafe fn unit(&self) -> Id<UnitType, Shared>;
        #[method(doubleValue)]
        pub unsafe fn doubleValue(&self) -> c_double;
        #[method_id(init)]
        pub unsafe fn init(&self) -> Id<Self, Shared>;
        #[method_id(initWithDoubleValue:unit:)]
        pub unsafe fn initWithDoubleValue_unit(
            &self,
            doubleValue: c_double,
            unit: &UnitType,
        ) -> Id<Self, Shared>;
        #[method(canBeConvertedToUnit:)]
        pub unsafe fn canBeConvertedToUnit(&self, unit: &NSUnit) -> bool;
        #[method_id(measurementByConvertingToUnit:)]
        pub unsafe fn measurementByConvertingToUnit(
            &self,
            unit: &NSUnit,
        ) -> Id<NSMeasurement, Shared>;
        #[method_id(measurementByAddingMeasurement:)]
        pub unsafe fn measurementByAddingMeasurement(
            &self,
            measurement: &NSMeasurement<UnitType>,
        ) -> Id<NSMeasurement<UnitType>, Shared>;
        #[method_id(measurementBySubtractingMeasurement:)]
        pub unsafe fn measurementBySubtractingMeasurement(
            &self,
            measurement: &NSMeasurement<UnitType>,
        ) -> Id<NSMeasurement<UnitType>, Shared>;
    }
);
