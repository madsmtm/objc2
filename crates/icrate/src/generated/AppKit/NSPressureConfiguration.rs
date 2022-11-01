//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug)]
    pub struct NSPressureConfiguration;

    unsafe impl ClassType for NSPressureConfiguration {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl NSPressureConfiguration {
        #[method(pressureBehavior)]
        pub unsafe fn pressureBehavior(&self) -> NSPressureBehavior;

        #[method_id(@__retain_semantics Init initWithPressureBehavior:)]
        pub unsafe fn initWithPressureBehavior(
            this: Option<Allocated<Self>>,
            pressureBehavior: NSPressureBehavior,
        ) -> Id<Self, Shared>;

        #[method(set)]
        pub unsafe fn set(&self);
    }
);

extern_methods!(
    /// NSPressureConfiguration
    unsafe impl NSView {
        #[method_id(@__retain_semantics Other pressureConfiguration)]
        pub unsafe fn pressureConfiguration(&self) -> Option<Id<NSPressureConfiguration, Shared>>;

        #[method(setPressureConfiguration:)]
        pub unsafe fn setPressureConfiguration(
            &self,
            pressureConfiguration: Option<&NSPressureConfiguration>,
        );
    }
);
