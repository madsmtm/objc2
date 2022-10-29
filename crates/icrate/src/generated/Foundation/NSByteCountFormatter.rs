#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSByteCountFormatter;
    unsafe impl ClassType for NSByteCountFormatter {
        type Super = NSFormatter;
    }
);
extern_methods!(
    unsafe impl NSByteCountFormatter {
        #[method_id(stringFromByteCount:countStyle:)]
        pub unsafe fn stringFromByteCount_countStyle(
            byteCount: c_longlong,
            countStyle: NSByteCountFormatterCountStyle,
        ) -> Id<NSString, Shared>;
        #[method_id(stringFromByteCount:)]
        pub unsafe fn stringFromByteCount(&self, byteCount: c_longlong) -> Id<NSString, Shared>;
        #[method_id(stringFromMeasurement:countStyle:)]
        pub unsafe fn stringFromMeasurement_countStyle(
            measurement: &NSMeasurement<NSUnitInformationStorage>,
            countStyle: NSByteCountFormatterCountStyle,
        ) -> Id<NSString, Shared>;
        #[method_id(stringFromMeasurement:)]
        pub unsafe fn stringFromMeasurement(
            &self,
            measurement: &NSMeasurement<NSUnitInformationStorage>,
        ) -> Id<NSString, Shared>;
        #[method_id(stringForObjectValue:)]
        pub unsafe fn stringForObjectValue(
            &self,
            obj: Option<&Object>,
        ) -> Option<Id<NSString, Shared>>;
        #[method(allowedUnits)]
        pub unsafe fn allowedUnits(&self) -> NSByteCountFormatterUnits;
        #[method(setAllowedUnits:)]
        pub unsafe fn setAllowedUnits(&self, allowedUnits: NSByteCountFormatterUnits);
        #[method(countStyle)]
        pub unsafe fn countStyle(&self) -> NSByteCountFormatterCountStyle;
        #[method(setCountStyle:)]
        pub unsafe fn setCountStyle(&self, countStyle: NSByteCountFormatterCountStyle);
        #[method(allowsNonnumericFormatting)]
        pub unsafe fn allowsNonnumericFormatting(&self) -> bool;
        #[method(setAllowsNonnumericFormatting:)]
        pub unsafe fn setAllowsNonnumericFormatting(&self, allowsNonnumericFormatting: bool);
        #[method(includesUnit)]
        pub unsafe fn includesUnit(&self) -> bool;
        #[method(setIncludesUnit:)]
        pub unsafe fn setIncludesUnit(&self, includesUnit: bool);
        #[method(includesCount)]
        pub unsafe fn includesCount(&self) -> bool;
        #[method(setIncludesCount:)]
        pub unsafe fn setIncludesCount(&self, includesCount: bool);
        #[method(includesActualByteCount)]
        pub unsafe fn includesActualByteCount(&self) -> bool;
        #[method(setIncludesActualByteCount:)]
        pub unsafe fn setIncludesActualByteCount(&self, includesActualByteCount: bool);
        #[method(isAdaptive)]
        pub unsafe fn isAdaptive(&self) -> bool;
        #[method(setAdaptive:)]
        pub unsafe fn setAdaptive(&self, adaptive: bool);
        #[method(zeroPadsFractionDigits)]
        pub unsafe fn zeroPadsFractionDigits(&self) -> bool;
        #[method(setZeroPadsFractionDigits:)]
        pub unsafe fn setZeroPadsFractionDigits(&self, zeroPadsFractionDigits: bool);
        #[method(formattingContext)]
        pub unsafe fn formattingContext(&self) -> NSFormattingContext;
        #[method(setFormattingContext:)]
        pub unsafe fn setFormattingContext(&self, formattingContext: NSFormattingContext);
    }
);
