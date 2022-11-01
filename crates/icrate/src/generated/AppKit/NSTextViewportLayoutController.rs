//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;

pub type NSTextViewportLayoutControllerDelegate = NSObject;

extern_class!(
    #[derive(Debug)]
    pub struct NSTextViewportLayoutController;

    unsafe impl ClassType for NSTextViewportLayoutController {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl NSTextViewportLayoutController {
        #[method_id(@__retain_semantics Init initWithTextLayoutManager:)]
        pub unsafe fn initWithTextLayoutManager(
            this: Option<Allocated<Self>>,
            textLayoutManager: &NSTextLayoutManager,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self)
            -> Option<Id<NSTextViewportLayoutControllerDelegate, Shared>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&NSTextViewportLayoutControllerDelegate>);

        #[method_id(@__retain_semantics Other textLayoutManager)]
        pub unsafe fn textLayoutManager(&self) -> Option<Id<NSTextLayoutManager, Shared>>;

        #[method(viewportBounds)]
        pub unsafe fn viewportBounds(&self) -> CGRect;

        #[method_id(@__retain_semantics Other viewportRange)]
        pub unsafe fn viewportRange(&self) -> Option<Id<NSTextRange, Shared>>;

        #[method(layoutViewport)]
        pub unsafe fn layoutViewport(&self);

        #[method(relocateViewportToTextLocation:)]
        pub unsafe fn relocateViewportToTextLocation(
            &self,
            textLocation: &NSTextLocation,
        ) -> CGFloat;

        #[method(adjustViewportByVerticalOffset:)]
        pub unsafe fn adjustViewportByVerticalOffset(&self, verticalOffset: CGFloat);
    }
);
