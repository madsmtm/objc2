//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;

pub type NSTouchBarItemIdentifier = NSString;

pub static NSTouchBarItemPriorityHigh: NSTouchBarItemPriority = 1000;

pub static NSTouchBarItemPriorityNormal: NSTouchBarItemPriority = 0;

pub static NSTouchBarItemPriorityLow: NSTouchBarItemPriority = -1000;

extern_class!(
    #[derive(Debug)]
    pub struct NSTouchBarItem;

    unsafe impl ClassType for NSTouchBarItem {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl NSTouchBarItem {
        #[method_id(@__retain_semantics Init initWithIdentifier:)]
        pub unsafe fn initWithIdentifier(
            this: Option<Allocated<Self>>,
            identifier: &NSTouchBarItemIdentifier,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Option<Allocated<Self>>,
            coder: &NSCoder,
        ) -> Option<Id<Self, Shared>>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other identifier)]
        pub unsafe fn identifier(&self) -> Id<NSTouchBarItemIdentifier, Shared>;

        #[method(visibilityPriority)]
        pub unsafe fn visibilityPriority(&self) -> NSTouchBarItemPriority;

        #[method(setVisibilityPriority:)]
        pub unsafe fn setVisibilityPriority(&self, visibilityPriority: NSTouchBarItemPriority);

        #[method_id(@__retain_semantics Other view)]
        pub unsafe fn view(&self) -> Option<Id<NSView, Shared>>;

        #[method_id(@__retain_semantics Other viewController)]
        pub unsafe fn viewController(&self) -> Option<Id<NSViewController, Shared>>;

        #[method_id(@__retain_semantics Other customizationLabel)]
        pub unsafe fn customizationLabel(&self) -> Id<NSString, Shared>;

        #[method(isVisible)]
        pub unsafe fn isVisible(&self) -> bool;
    }
);

extern "C" {
    pub static NSTouchBarItemIdentifierFixedSpaceSmall: &'static NSTouchBarItemIdentifier;
}

extern "C" {
    pub static NSTouchBarItemIdentifierFixedSpaceLarge: &'static NSTouchBarItemIdentifier;
}

extern "C" {
    pub static NSTouchBarItemIdentifierFlexibleSpace: &'static NSTouchBarItemIdentifier;
}

extern "C" {
    pub static NSTouchBarItemIdentifierOtherItemsProxy: &'static NSTouchBarItemIdentifier;
}
