use crate::Foundation::generated::NSFormatter::*;
use crate::Foundation::generated::NSMeasurement::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, msg_send, msg_send_id, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSByteCountFormatter;
    unsafe impl ClassType for NSByteCountFormatter {
        type Super = NSFormatter;
    }
);
extern_methods!(
    unsafe impl NSByteCountFormatter {
        pub unsafe fn stringFromByteCount_countStyle(
            byteCount: c_longlong,
            countStyle: NSByteCountFormatterCountStyle,
        ) -> Id<NSString, Shared> {
            msg_send_id![
                Self::class(),
                stringFromByteCount: byteCount,
                countStyle: countStyle
            ]
        }
        pub unsafe fn stringFromByteCount(&self, byteCount: c_longlong) -> Id<NSString, Shared> {
            msg_send_id![self, stringFromByteCount: byteCount]
        }
        pub unsafe fn stringFromMeasurement_countStyle(
            measurement: &NSMeasurement<NSUnitInformationStorage>,
            countStyle: NSByteCountFormatterCountStyle,
        ) -> Id<NSString, Shared> {
            msg_send_id![
                Self::class(),
                stringFromMeasurement: measurement,
                countStyle: countStyle
            ]
        }
        pub unsafe fn stringFromMeasurement(
            &self,
            measurement: &NSMeasurement<NSUnitInformationStorage>,
        ) -> Id<NSString, Shared> {
            msg_send_id![self, stringFromMeasurement: measurement]
        }
        pub unsafe fn stringForObjectValue(
            &self,
            obj: Option<&Object>,
        ) -> Option<Id<NSString, Shared>> {
            msg_send_id![self, stringForObjectValue: obj]
        }
        pub unsafe fn allowedUnits(&self) -> NSByteCountFormatterUnits {
            msg_send![self, allowedUnits]
        }
        pub unsafe fn setAllowedUnits(&self, allowedUnits: NSByteCountFormatterUnits) {
            msg_send![self, setAllowedUnits: allowedUnits]
        }
        pub unsafe fn countStyle(&self) -> NSByteCountFormatterCountStyle {
            msg_send![self, countStyle]
        }
        pub unsafe fn setCountStyle(&self, countStyle: NSByteCountFormatterCountStyle) {
            msg_send![self, setCountStyle: countStyle]
        }
        pub unsafe fn allowsNonnumericFormatting(&self) -> bool {
            msg_send![self, allowsNonnumericFormatting]
        }
        pub unsafe fn setAllowsNonnumericFormatting(&self, allowsNonnumericFormatting: bool) {
            msg_send![
                self,
                setAllowsNonnumericFormatting: allowsNonnumericFormatting
            ]
        }
        pub unsafe fn includesUnit(&self) -> bool {
            msg_send![self, includesUnit]
        }
        pub unsafe fn setIncludesUnit(&self, includesUnit: bool) {
            msg_send![self, setIncludesUnit: includesUnit]
        }
        pub unsafe fn includesCount(&self) -> bool {
            msg_send![self, includesCount]
        }
        pub unsafe fn setIncludesCount(&self, includesCount: bool) {
            msg_send![self, setIncludesCount: includesCount]
        }
        pub unsafe fn includesActualByteCount(&self) -> bool {
            msg_send![self, includesActualByteCount]
        }
        pub unsafe fn setIncludesActualByteCount(&self, includesActualByteCount: bool) {
            msg_send![self, setIncludesActualByteCount: includesActualByteCount]
        }
        pub unsafe fn isAdaptive(&self) -> bool {
            msg_send![self, isAdaptive]
        }
        pub unsafe fn setAdaptive(&self, adaptive: bool) {
            msg_send![self, setAdaptive: adaptive]
        }
        pub unsafe fn zeroPadsFractionDigits(&self) -> bool {
            msg_send![self, zeroPadsFractionDigits]
        }
        pub unsafe fn setZeroPadsFractionDigits(&self, zeroPadsFractionDigits: bool) {
            msg_send![self, setZeroPadsFractionDigits: zeroPadsFractionDigits]
        }
        pub unsafe fn formattingContext(&self) -> NSFormattingContext {
            msg_send![self, formattingContext]
        }
        pub unsafe fn setFormattingContext(&self, formattingContext: NSFormattingContext) {
            msg_send![self, setFormattingContext: formattingContext]
        }
    }
);
