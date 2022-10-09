use crate::Foundation::generated::Foundation::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSLengthFormatter;
    unsafe impl ClassType for NSLengthFormatter {
        type Super = NSFormatter;
    }
);
extern_methods!(
    unsafe impl NSLengthFormatter {
        #[method_id(numberFormatter)]
        pub unsafe fn numberFormatter(&self) -> Id<NSNumberFormatter, Shared>;
        #[method(setNumberFormatter:)]
        pub unsafe fn setNumberFormatter(&self, numberFormatter: Option<&NSNumberFormatter>);
        #[method(unitStyle)]
        pub unsafe fn unitStyle(&self) -> NSFormattingUnitStyle;
        #[method(setUnitStyle:)]
        pub unsafe fn setUnitStyle(&self, unitStyle: NSFormattingUnitStyle);
        #[method(isForPersonHeightUse)]
        pub unsafe fn isForPersonHeightUse(&self) -> bool;
        #[method(setForPersonHeightUse:)]
        pub unsafe fn setForPersonHeightUse(&self, forPersonHeightUse: bool);
        #[method_id(stringFromValue:unit:)]
        pub unsafe fn stringFromValue_unit(
            &self,
            value: c_double,
            unit: NSLengthFormatterUnit,
        ) -> Id<NSString, Shared>;
        #[method_id(stringFromMeters:)]
        pub unsafe fn stringFromMeters(&self, numberInMeters: c_double) -> Id<NSString, Shared>;
        #[method_id(unitStringFromValue:unit:)]
        pub unsafe fn unitStringFromValue_unit(
            &self,
            value: c_double,
            unit: NSLengthFormatterUnit,
        ) -> Id<NSString, Shared>;
        #[method_id(unitStringFromMeters:usedUnit:)]
        pub unsafe fn unitStringFromMeters_usedUnit(
            &self,
            numberInMeters: c_double,
            unitp: *mut NSLengthFormatterUnit,
        ) -> Id<NSString, Shared>;
        #[method(getObjectValue:forString:errorDescription:)]
        pub unsafe fn getObjectValue_forString_errorDescription(
            &self,
            obj: Option<&mut Option<Id<Object, Shared>>>,
            string: &NSString,
            error: Option<&mut Option<Id<NSString, Shared>>>,
        ) -> bool;
    }
);
