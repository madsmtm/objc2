use crate::Foundation::generated::Foundation::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, msg_send, msg_send_id, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSEnergyFormatter;
    unsafe impl ClassType for NSEnergyFormatter {
        type Super = NSFormatter;
    }
);
impl NSEnergyFormatter {
    pub unsafe fn stringFromValue_unit(
        &self,
        value: c_double,
        unit: NSEnergyFormatterUnit,
    ) -> Id<NSString, Shared> {
        msg_send_id![self, stringFromValue: value, unit: unit]
    }
    pub unsafe fn stringFromJoules(&self, numberInJoules: c_double) -> Id<NSString, Shared> {
        msg_send_id![self, stringFromJoules: numberInJoules]
    }
    pub unsafe fn unitStringFromValue_unit(
        &self,
        value: c_double,
        unit: NSEnergyFormatterUnit,
    ) -> Id<NSString, Shared> {
        msg_send_id![self, unitStringFromValue: value, unit: unit]
    }
    pub unsafe fn unitStringFromJoules_usedUnit(
        &self,
        numberInJoules: c_double,
        unitp: *mut NSEnergyFormatterUnit,
    ) -> Id<NSString, Shared> {
        msg_send_id![self, unitStringFromJoules: numberInJoules, usedUnit: unitp]
    }
    pub unsafe fn getObjectValue_forString_errorDescription(
        &self,
        obj: Option<&mut Option<Id<Object, Shared>>>,
        string: &NSString,
        error: Option<&mut Option<Id<NSString, Shared>>>,
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
    pub unsafe fn isForFoodEnergyUse(&self) -> bool {
        msg_send![self, isForFoodEnergyUse]
    }
    pub unsafe fn setForFoodEnergyUse(&self, forFoodEnergyUse: bool) {
        msg_send![self, setForFoodEnergyUse: forFoodEnergyUse]
    }
}
