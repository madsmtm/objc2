use crate::AppKit::generated::AppKitDefines::*;
use crate::AppKit::generated::NSEvent::*;
use crate::AppKit::generated::NSView::*;
use crate::Foundation::generated::NSObjCRuntime::*;
use crate::Foundation::generated::NSObject::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
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
        #[method_id(initWithPressureBehavior:)]
        pub unsafe fn initWithPressureBehavior(
            &self,
            pressureBehavior: NSPressureBehavior,
        ) -> Id<Self, Shared>;
        #[method(set)]
        pub unsafe fn set(&self);
    }
);
extern_methods!(
    #[doc = "NSPressureConfiguration"]
    unsafe impl NSView {
        #[method_id(pressureConfiguration)]
        pub unsafe fn pressureConfiguration(&self) -> Option<Id<NSPressureConfiguration, Shared>>;
        #[method(setPressureConfiguration:)]
        pub unsafe fn setPressureConfiguration(
            &self,
            pressureConfiguration: Option<&NSPressureConfiguration>,
        );
    }
);
