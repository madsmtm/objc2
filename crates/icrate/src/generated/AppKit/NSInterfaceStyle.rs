use crate::AppKit::generated::AppKitDefines::*;
use crate::AppKit::generated::NSResponder::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
pub type NSInterfaceStyle = NSUInteger;
extern_methods!(
    #[doc = "NSInterfaceStyle"]
    unsafe impl NSResponder {
        #[method(interfaceStyle)]
        pub unsafe fn interfaceStyle(&self) -> NSInterfaceStyle;
        #[method(setInterfaceStyle:)]
        pub unsafe fn setInterfaceStyle(&self, interfaceStyle: NSInterfaceStyle);
    }
);
