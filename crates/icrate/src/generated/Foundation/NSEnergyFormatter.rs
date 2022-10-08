use crate::Foundation::generated::Foundation::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSEnergyFormatter;
    unsafe impl ClassType for NSEnergyFormatter {
        type Super = NSFormatter;
    }
);
extern_methods!(
    unsafe impl NSEnergyFormatter {
        #[method_id(numberFormatter)]
        pub unsafe fn numberFormatter(&self) -> Id<NSNumberFormatter, Shared>;
        # [method (setNumberFormatter :)]
        pub unsafe fn setNumberFormatter(&self, numberFormatter: Option<&NSNumberFormatter>);
        #[method(unitStyle)]
        pub unsafe fn unitStyle(&self) -> NSFormattingUnitStyle;
        # [method (setUnitStyle :)]
        pub unsafe fn setUnitStyle(&self, unitStyle: NSFormattingUnitStyle);
        #[method(isForFoodEnergyUse)]
        pub unsafe fn isForFoodEnergyUse(&self) -> bool;
        # [method (setForFoodEnergyUse :)]
        pub unsafe fn setForFoodEnergyUse(&self, forFoodEnergyUse: bool);
        # [method_id (stringFromValue : unit :)]
        pub unsafe fn stringFromValue_unit(
            &self,
            value: c_double,
            unit: NSEnergyFormatterUnit,
        ) -> Id<NSString, Shared>;
        # [method_id (stringFromJoules :)]
        pub unsafe fn stringFromJoules(&self, numberInJoules: c_double) -> Id<NSString, Shared>;
        # [method_id (unitStringFromValue : unit :)]
        pub unsafe fn unitStringFromValue_unit(
            &self,
            value: c_double,
            unit: NSEnergyFormatterUnit,
        ) -> Id<NSString, Shared>;
        # [method_id (unitStringFromJoules : usedUnit :)]
        pub unsafe fn unitStringFromJoules_usedUnit(
            &self,
            numberInJoules: c_double,
            unitp: *mut NSEnergyFormatterUnit,
        ) -> Id<NSString, Shared>;
        # [method (getObjectValue : forString : errorDescription :)]
        pub unsafe fn getObjectValue_forString_errorDescription(
            &self,
            obj: Option<&mut Option<Id<Object, Shared>>>,
            string: &NSString,
            error: Option<&mut Option<Id<NSString, Shared>>>,
        ) -> bool;
    }
);
