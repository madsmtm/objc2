use crate::AppKit::generated::AppKitDefines::*;
use crate::AppKit::generated::NSTextField::*;
use crate::AppKit::generated::NSTextFieldCell::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSSecureTextField;
    unsafe impl ClassType for NSSecureTextField {
        type Super = NSTextField;
    }
);
extern_methods!(
    unsafe impl NSSecureTextField {}
);
extern_class!(
    #[derive(Debug)]
    pub struct NSSecureTextFieldCell;
    unsafe impl ClassType for NSSecureTextFieldCell {
        type Super = NSTextFieldCell;
    }
);
extern_methods!(
    unsafe impl NSSecureTextFieldCell {
        #[method(echosBullets)]
        pub unsafe fn echosBullets(&self) -> bool;
        #[method(setEchosBullets:)]
        pub unsafe fn setEchosBullets(&self, echosBullets: bool);
    }
);
