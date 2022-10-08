use super::__exported::NSString;
use crate::Foundation::generated::NSObjCRuntime::*;
use crate::Foundation::generated::NSValue::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_methods!(
    #[doc = "NSValueRangeExtensions"]
    unsafe impl NSValue {
        # [method_id (valueWithRange :)]
        pub unsafe fn valueWithRange(range: NSRange) -> Id<NSValue, Shared>;
        #[method(rangeValue)]
        pub unsafe fn rangeValue(&self) -> NSRange;
    }
);
