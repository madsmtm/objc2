use crate::AppKit::generated::AppKitDefines::*;
use crate::AppKit::generated::NSControl::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSStepper;
    unsafe impl ClassType for NSStepper {
        type Super = NSControl;
    }
);
extern_methods!(
    unsafe impl NSStepper {
        #[method(minValue)]
        pub unsafe fn minValue(&self) -> c_double;
        #[method(setMinValue:)]
        pub unsafe fn setMinValue(&self, minValue: c_double);
        #[method(maxValue)]
        pub unsafe fn maxValue(&self) -> c_double;
        #[method(setMaxValue:)]
        pub unsafe fn setMaxValue(&self, maxValue: c_double);
        #[method(increment)]
        pub unsafe fn increment(&self) -> c_double;
        #[method(setIncrement:)]
        pub unsafe fn setIncrement(&self, increment: c_double);
        #[method(valueWraps)]
        pub unsafe fn valueWraps(&self) -> bool;
        #[method(setValueWraps:)]
        pub unsafe fn setValueWraps(&self, valueWraps: bool);
        #[method(autorepeat)]
        pub unsafe fn autorepeat(&self) -> bool;
        #[method(setAutorepeat:)]
        pub unsafe fn setAutorepeat(&self, autorepeat: bool);
    }
);
