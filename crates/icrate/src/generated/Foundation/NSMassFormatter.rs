use super::__exported::NSNumberFormatter;
use crate::Foundation::generated::NSFormatter::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, msg_send, msg_send_id, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSMassFormatter;
    unsafe impl ClassType for NSMassFormatter {
        type Super = NSFormatter;
    }
);
impl NSMassFormatter {
    pub unsafe fn stringFromValue_unit(
        &self,
        value: c_double,
        unit: NSMassFormatterUnit,
    ) -> Id<NSString, Shared> {
        msg_send_id![self, stringFromValue: value, unit: unit]
    }
    pub unsafe fn stringFromKilograms(&self, numberInKilograms: c_double) -> Id<NSString, Shared> {
        msg_send_id![self, stringFromKilograms: numberInKilograms]
    }
    pub unsafe fn unitStringFromValue_unit(
        &self,
        value: c_double,
        unit: NSMassFormatterUnit,
    ) -> Id<NSString, Shared> {
        msg_send_id![self, unitStringFromValue: value, unit: unit]
    }
    pub unsafe fn unitStringFromKilograms_usedUnit(
        &self,
        numberInKilograms: c_double,
        unitp: *mut NSMassFormatterUnit,
    ) -> Id<NSString, Shared> {
        msg_send_id![
            self,
            unitStringFromKilograms: numberInKilograms,
            usedUnit: unitp
        ]
    }
    pub unsafe fn getObjectValue_forString_errorDescription(
        &self,
        obj: *mut *mut Object,
        string: &NSString,
        error: *mut *mut NSString,
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
    pub unsafe fn isForPersonMassUse(&self) -> bool {
        msg_send![self, isForPersonMassUse]
    }
    pub unsafe fn setForPersonMassUse(&self, forPersonMassUse: bool) {
        msg_send![self, setForPersonMassUse: forPersonMassUse]
    }
}
