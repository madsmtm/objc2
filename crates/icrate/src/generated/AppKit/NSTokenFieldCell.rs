use super::__exported::NSTextContainer;
use crate::AppKit::generated::AppKitDefines::*;
use crate::AppKit::generated::NSTextFieldCell::*;
use crate::Foundation::generated::Foundation::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
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
