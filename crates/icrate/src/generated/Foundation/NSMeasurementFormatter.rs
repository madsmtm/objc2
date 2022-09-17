#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, msg_send, msg_send_id, ClassType};
extern_class!(
    #[derive(Debug)]
    struct NSMeasurementFormatter;
    unsafe impl ClassType for NSMeasurementFormatter {
        type Super = NSFormatter;
    }
);
impl NSMeasurementFormatter {
    pub unsafe fn stringFromMeasurement(
        &self,
        measurement: &NSMeasurement,
    ) -> Id<NSString, Shared> {
        msg_send_id![self, stringFromMeasurement: measurement]
    }
    pub unsafe fn stringFromUnit(&self, unit: &NSUnit) -> Id<NSString, Shared> {
        msg_send_id![self, stringFromUnit: unit]
    }
    pub unsafe fn unitOptions(&self) -> NSMeasurementFormatterUnitOptions {
        msg_send![self, unitOptions]
    }
    pub unsafe fn setUnitOptions(&self, unitOptions: NSMeasurementFormatterUnitOptions) {
        msg_send![self, setUnitOptions: unitOptions]
    }
    pub unsafe fn unitStyle(&self) -> NSFormattingUnitStyle {
        msg_send![self, unitStyle]
    }
    pub unsafe fn setUnitStyle(&self, unitStyle: NSFormattingUnitStyle) {
        msg_send![self, setUnitStyle: unitStyle]
    }
    pub unsafe fn locale(&self) -> Id<NSLocale, Shared> {
        msg_send_id![self, locale]
    }
    pub unsafe fn setLocale(&self, locale: Option<&NSLocale>) {
        msg_send![self, setLocale: locale]
    }
    pub unsafe fn numberFormatter(&self) -> Id<NSNumberFormatter, Shared> {
        msg_send_id![self, numberFormatter]
    }
    pub unsafe fn setNumberFormatter(&self, numberFormatter: Option<&NSNumberFormatter>) {
        msg_send![self, setNumberFormatter: numberFormatter]
    }
}
