//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;

pub type NSTokenStyle = NSUInteger;
pub const NSTokenStyleDefault: NSTokenStyle = 0;
pub const NSTokenStyleNone: NSTokenStyle = 1;
pub const NSTokenStyleRounded: NSTokenStyle = 2;
pub const NSTokenStyleSquared: NSTokenStyle = 3;
pub const NSTokenStylePlainSquared: NSTokenStyle = 4;

extern_class!(
    #[derive(Debug)]
    pub struct NSTokenFieldCell;

    unsafe impl ClassType for NSTokenFieldCell {
        type Super = NSTextFieldCell;
    }
);

extern_methods!(
    unsafe impl NSTokenFieldCell {
        #[method(tokenStyle)]
        pub unsafe fn tokenStyle(&self) -> NSTokenStyle;

        #[method(setTokenStyle:)]
        pub unsafe fn setTokenStyle(&self, tokenStyle: NSTokenStyle);

        #[method(completionDelay)]
        pub unsafe fn completionDelay(&self) -> NSTimeInterval;

        #[method(setCompletionDelay:)]
        pub unsafe fn setCompletionDelay(&self, completionDelay: NSTimeInterval);

        #[method(defaultCompletionDelay)]
        pub unsafe fn defaultCompletionDelay() -> NSTimeInterval;

        #[method_id(tokenizingCharacterSet)]
        pub unsafe fn tokenizingCharacterSet(&self) -> Id<NSCharacterSet, Shared>;

        #[method(setTokenizingCharacterSet:)]
        pub unsafe fn setTokenizingCharacterSet(
            &self,
            tokenizingCharacterSet: Option<&NSCharacterSet>,
        );

        #[method_id(defaultTokenizingCharacterSet)]
        pub unsafe fn defaultTokenizingCharacterSet() -> Id<NSCharacterSet, Shared>;

        #[method_id(delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<NSTokenFieldCellDelegate, Shared>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&NSTokenFieldCellDelegate>);
    }
);

pub type NSTokenFieldCellDelegate = NSObject;

pub static NSDefaultTokenStyle: NSTokenStyle = NSTokenStyleDefault;

pub static NSPlainTextTokenStyle: NSTokenStyle = NSTokenStyleNone;

pub static NSRoundedTokenStyle: NSTokenStyle = NSTokenStyleRounded;
