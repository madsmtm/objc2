//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug)]
    pub struct NSStepperCell;

    unsafe impl ClassType for NSStepperCell {
        type Super = NSActionCell;
    }
);

extern_methods!(
    unsafe impl NSStepperCell {
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
