use super::__exported::NSTextLayoutFragment;
use super::__exported::NSTextLayoutManager;
use super::__exported::NSTextLocation;
use super::__exported::NSTextRange;
use crate::CoreGraphics::generated::CGGeometry::*;
use crate::Foundation::generated::NSObject::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
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
        #[method_id(initWithTextLayoutManager:)]
        pub unsafe fn initWithTextLayoutManager(
            &self,
            textLayoutManager: &NSTextLayoutManager,
        ) -> Id<Self, Shared>;
        #[method_id(new)]
        pub unsafe fn new() -> Id<Self, Shared>;
        #[method_id(init)]
        pub unsafe fn init(&self) -> Id<Self, Shared>;
        #[method_id(delegate)]
        pub unsafe fn delegate(&self)
            -> Option<Id<NSTextViewportLayoutControllerDelegate, Shared>>;
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&NSTextViewportLayoutControllerDelegate>);
        #[method_id(textLayoutManager)]
        pub unsafe fn textLayoutManager(&self) -> Option<Id<NSTextLayoutManager, Shared>>;
        #[method(viewportBounds)]
        pub unsafe fn viewportBounds(&self) -> CGRect;
        #[method_id(viewportRange)]
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
