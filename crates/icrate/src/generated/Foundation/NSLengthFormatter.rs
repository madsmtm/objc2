#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, msg_send, msg_send_id, ClassType};
extern_class!(
    #[derive(Debug)]
    struct NSLengthFormatter;
    unsafe impl ClassType for NSLengthFormatter {
        type Super = NSFormatter;
    }
);
impl NSLengthFormatter {
    pub unsafe fn stringFromValue_unit(
        &self,
        value: c_double,
        unit: NSLengthFormatterUnit,
    ) -> Id<NSString, Shared> {
        msg_send_id![self, stringFromValue: value, unit: unit]
    }
    pub unsafe fn stringFromMeters(&self, numberInMeters: c_double) -> Id<NSString, Shared> {
        msg_send_id![self, stringFromMeters: numberInMeters]
    }
    pub unsafe fn unitStringFromValue_unit(
        &self,
        value: c_double,
        unit: NSLengthFormatterUnit,
    ) -> Id<NSString, Shared> {
        msg_send_id![self, unitStringFromValue: value, unit: unit]
    }
    pub unsafe fn unitStringFromMeters_usedUnit(
        &self,
        numberInMeters: c_double,
        unitp: *mut NSLengthFormatterUnit,
    ) -> Id<NSString, Shared> {
        msg_send_id![self, unitStringFromMeters: numberInMeters, usedUnit: unitp]
    }
    pub unsafe fn getObjectValue_forString_errorDescription(
        &self,
        obj: *mut Option<&Object>,
        string: &NSString,
        error: *mut Option<&NSString>,
    ) -> bool {
        msg_send![
            self,
            getObjectValue: obj,
            forString: string,
            errorDescription: error
        ]
    }
    pub unsafe fn numberFormatter(&self) -> Id<NSNumberFormatter, Shared> {
        msg_send_id![self, numberFormatter]
    }
    pub unsafe fn setNumberFormatter(&self, numberFormatter: Option<&NSNumberFormatter>) {
        msg_send![self, setNumberFormatter: numberFormatter]
    }
    pub unsafe fn unitStyle(&self) -> NSFormattingUnitStyle {
        msg_send![self, unitStyle]
    }
    pub unsafe fn setUnitStyle(&self, unitStyle: NSFormattingUnitStyle) {
        msg_send![self, setUnitStyle: unitStyle]
    }
    pub unsafe fn isForPersonHeightUse(&self) -> bool {
        msg_send![self, isForPersonHeightUse]
    }
    pub unsafe fn setForPersonHeightUse(&self, forPersonHeightUse: bool) {
        msg_send![self, setForPersonHeightUse: forPersonHeightUse]
    }
}
