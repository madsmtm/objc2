//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};

pub const NSNoInterfaceStyle: i32 = 0;
pub const NSNextStepInterfaceStyle: i32 = 1;
pub const NSWindows95InterfaceStyle: i32 = 2;
pub const NSMacintoshInterfaceStyle: i32 = 3;

pub type NSInterfaceStyle = NSUInteger;

extern_methods!(
    /// NSInterfaceStyle
    unsafe impl NSResponder {
        #[method(interfaceStyle)]
        pub unsafe fn interfaceStyle(&self) -> NSInterfaceStyle;

        #[method(setInterfaceStyle:)]
        pub unsafe fn setInterfaceStyle(&self, interfaceStyle: NSInterfaceStyle);
    }
);
