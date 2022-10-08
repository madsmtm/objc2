use super::__exported::NSNumberFormatter;
use crate::Foundation::generated::NSFormatter::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSMassFormatter;
    unsafe impl ClassType for NSMassFormatter {
        type Super = NSFormatter;
    }
);
extern_methods!(
    unsafe impl NSMassFormatter {
        #[method_id(numberFormatter)]
        pub unsafe fn numberFormatter(&self) -> Id<NSNumberFormatter, Shared>;
        # [method (setNumberFormatter :)]
        pub unsafe fn setNumberFormatter(&self, numberFormatter: Option<&NSNumberFormatter>);
        #[method(unitStyle)]
        pub unsafe fn unitStyle(&self) -> NSFormattingUnitStyle;
        # [method (setUnitStyle :)]
        pub unsafe fn setUnitStyle(&self, unitStyle: NSFormattingUnitStyle);
        #[method(isForPersonMassUse)]
        pub unsafe fn isForPersonMassUse(&self) -> bool;
        # [method (setForPersonMassUse :)]
        pub unsafe fn setForPersonMassUse(&self, forPersonMassUse: bool);
        # [method_id (stringFromValue : unit :)]
        pub unsafe fn stringFromValue_unit(
            &self,
            value: c_double,
            unit: NSMassFormatterUnit,
        ) -> Id<NSString, Shared>;
        # [method_id (stringFromKilograms :)]
        pub unsafe fn stringFromKilograms(
            &self,
            numberInKilograms: c_double,
        ) -> Id<NSString, Shared>;
        # [method_id (unitStringFromValue : unit :)]
        pub unsafe fn unitStringFromValue_unit(
            &self,
            value: c_double,
            unit: NSMassFormatterUnit,
        ) -> Id<NSString, Shared>;
        # [method_id (unitStringFromKilograms : usedUnit :)]
        pub unsafe fn unitStringFromKilograms_usedUnit(
            &self,
            numberInKilograms: c_double,
            unitp: *mut NSMassFormatterUnit,
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
