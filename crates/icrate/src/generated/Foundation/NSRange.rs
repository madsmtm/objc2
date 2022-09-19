use super::__exported::NSString;
use crate::Foundation::generated::NSObjCRuntime::*;
use crate::Foundation::generated::NSValue::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, msg_send, msg_send_id, ClassType};
#[doc = "NSValueRangeExtensions"]
impl NSValue {
    pub unsafe fn valueWithRange(range: NSRange) -> Id<NSValue, Shared> {
        msg_send_id![Self::class(), valueWithRange: range]
    }
    pub unsafe fn rangeValue(&self) -> NSRange {
        msg_send![self, rangeValue]
    }
}
