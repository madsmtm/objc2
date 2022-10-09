use crate::AppKit::generated::AppKitDefines::*;
use crate::Foundation::generated::NSDictionary::*;
use crate::Foundation::generated::NSGeometry::*;
use crate::Foundation::generated::NSObjCRuntime::*;
use crate::Foundation::generated::NSObject::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSTrackingArea;
    unsafe impl ClassType for NSTrackingArea {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSTrackingArea {
        #[method_id(initWithRect:options:owner:userInfo:)]
        pub unsafe fn initWithRect_options_owner_userInfo(
            &self,
            rect: NSRect,
            options: NSTrackingAreaOptions,
            owner: Option<&Object>,
            userInfo: Option<&NSDictionary<Object, Object>>,
        ) -> Id<Self, Shared>;
        #[method(rect)]
        pub unsafe fn rect(&self) -> NSRect;
        #[method(options)]
        pub unsafe fn options(&self) -> NSTrackingAreaOptions;
        #[method_id(owner)]
        pub unsafe fn owner(&self) -> Option<Id<Object, Shared>>;
        #[method_id(userInfo)]
        pub unsafe fn userInfo(&self) -> Option<Id<NSDictionary<Object, Object>, Shared>>;
    }
);
