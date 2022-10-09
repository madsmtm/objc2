use super::__exported::NSOperationQueue;
use super::__exported::NSTextAttachmentViewProvider;
use super::__exported::NSTextElement;
use super::__exported::NSTextLayoutManager;
use super::__exported::NSTextLineFragment;
use super::__exported::NSTextLocation;
use super::__exported::NSTextParagraph;
use super::__exported::NSTextRange;
use crate::CoreGraphics::generated::CoreGraphics::*;
use crate::Foundation::generated::NSArray::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSTextLayoutFragment;
    unsafe impl ClassType for NSTextLayoutFragment {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSTextLayoutFragment {
        #[method_id(initWithTextElement:range:)]
        pub unsafe fn initWithTextElement_range(
            &self,
            textElement: &NSTextElement,
            rangeInElement: Option<&NSTextRange>,
        ) -> Id<Self, Shared>;
        #[method_id(initWithCoder:)]
        pub unsafe fn initWithCoder(&self, coder: &NSCoder) -> Option<Id<Self, Shared>>;
        #[method_id(init)]
        pub unsafe fn init(&self) -> Id<Self, Shared>;
        #[method_id(textLayoutManager)]
        pub unsafe fn textLayoutManager(&self) -> Option<Id<NSTextLayoutManager, Shared>>;
        #[method_id(textElement)]
        pub unsafe fn textElement(&self) -> Option<Id<NSTextElement, Shared>>;
        #[method_id(rangeInElement)]
        pub unsafe fn rangeInElement(&self) -> Id<NSTextRange, Shared>;
        #[method_id(textLineFragments)]
        pub unsafe fn textLineFragments(&self) -> Id<NSArray<NSTextLineFragment>, Shared>;
        #[method_id(layoutQueue)]
        pub unsafe fn layoutQueue(&self) -> Option<Id<NSOperationQueue, Shared>>;
        #[method(setLayoutQueue:)]
        pub unsafe fn setLayoutQueue(&self, layoutQueue: Option<&NSOperationQueue>);
        #[method(state)]
        pub unsafe fn state(&self) -> NSTextLayoutFragmentState;
        #[method(invalidateLayout)]
        pub unsafe fn invalidateLayout(&self);
        #[method(layoutFragmentFrame)]
        pub unsafe fn layoutFragmentFrame(&self) -> CGRect;
        #[method(renderingSurfaceBounds)]
        pub unsafe fn renderingSurfaceBounds(&self) -> CGRect;
        #[method(leadingPadding)]
        pub unsafe fn leadingPadding(&self) -> CGFloat;
        #[method(trailingPadding)]
        pub unsafe fn trailingPadding(&self) -> CGFloat;
        #[method(topMargin)]
        pub unsafe fn topMargin(&self) -> CGFloat;
        #[method(bottomMargin)]
        pub unsafe fn bottomMargin(&self) -> CGFloat;
        #[method(drawAtPoint:inContext:)]
        pub unsafe fn drawAtPoint_inContext(&self, point: CGPoint, context: CGContextRef);
        #[method_id(textAttachmentViewProviders)]
        pub unsafe fn textAttachmentViewProviders(
            &self,
        ) -> Id<NSArray<NSTextAttachmentViewProvider>, Shared>;
        #[method(frameForTextAttachmentAtLocation:)]
        pub unsafe fn frameForTextAttachmentAtLocation(&self, location: &NSTextLocation) -> CGRect;
    }
);
