//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

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
