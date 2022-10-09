use crate::Foundation::generated::NSFormatter::*;
use crate::Foundation::generated::NSLocale::*;
use crate::Foundation::generated::NSMeasurement::*;
use crate::Foundation::generated::NSNumberFormatter::*;
use crate::Foundation::generated::NSObject::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSMeasurementFormatter;
    unsafe impl ClassType for NSMeasurementFormatter {
        type Super = NSFormatter;
    }
);
extern_methods!(
    unsafe impl NSMeasurementFormatter {
        #[method(unitOptions)]
        pub unsafe fn unitOptions(&self) -> NSMeasurementFormatterUnitOptions;
        #[method(setUnitOptions:)]
        pub unsafe fn setUnitOptions(&self, unitOptions: NSMeasurementFormatterUnitOptions);
        #[method(unitStyle)]
        pub unsafe fn unitStyle(&self) -> NSFormattingUnitStyle;
        #[method(setUnitStyle:)]
        pub unsafe fn setUnitStyle(&self, unitStyle: NSFormattingUnitStyle);
        #[method_id(locale)]
        pub unsafe fn locale(&self) -> Id<NSLocale, Shared>;
        #[method(setLocale:)]
        pub unsafe fn setLocale(&self, locale: Option<&NSLocale>);
        #[method_id(numberFormatter)]
        pub unsafe fn numberFormatter(&self) -> Id<NSNumberFormatter, Shared>;
        #[method(setNumberFormatter:)]
        pub unsafe fn setNumberFormatter(&self, numberFormatter: Option<&NSNumberFormatter>);
        #[method_id(stringFromMeasurement:)]
        pub unsafe fn stringFromMeasurement(
            &self,
            measurement: &NSMeasurement,
        ) -> Id<NSString, Shared>;
        #[method_id(stringFromUnit:)]
        pub unsafe fn stringFromUnit(&self, unit: &NSUnit) -> Id<NSString, Shared>;
    }
);
