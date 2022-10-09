use crate::AppKit::generated::AppKitDefines::*;
use crate::AppKit::generated::NSAccessibilityConstants::*;
use crate::AppKit::generated::NSAccessibilityProtocols::*;
use crate::Foundation::generated::Foundation::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSAccessibilityElement;
    unsafe impl ClassType for NSAccessibilityElement {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSAccessibilityElement {
        #[method_id(accessibilityElementWithRole:frame:label:parent:)]
        pub unsafe fn accessibilityElementWithRole_frame_label_parent(
            role: &NSAccessibilityRole,
            frame: NSRect,
            label: Option<&NSString>,
            parent: Option<&Object>,
        ) -> Id<Object, Shared>;
        #[method(accessibilityAddChildElement:)]
        pub unsafe fn accessibilityAddChildElement(&self, childElement: &NSAccessibilityElement);
        #[method(accessibilityFrameInParentSpace)]
        pub unsafe fn accessibilityFrameInParentSpace(&self) -> NSRect;
        #[method(setAccessibilityFrameInParentSpace:)]
        pub unsafe fn setAccessibilityFrameInParentSpace(
            &self,
            accessibilityFrameInParentSpace: NSRect,
        );
    }
);
