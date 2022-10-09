use crate::AppKit::generated::AppKitDefines::*;
use crate::AppKit::generated::NSButton::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSStatusBarButton;
    unsafe impl ClassType for NSStatusBarButton {
        type Super = NSButton;
    }
);
extern_methods!(
    unsafe impl NSStatusBarButton {
        #[method(appearsDisabled)]
        pub unsafe fn appearsDisabled(&self) -> bool;
        #[method(setAppearsDisabled:)]
        pub unsafe fn setAppearsDisabled(&self, appearsDisabled: bool);
    }
);
