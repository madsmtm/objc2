use crate::AppKit::generated::AppKitDefines::*;
use crate::AppKit::generated::NSCell::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSActionCell;
    unsafe impl ClassType for NSActionCell {
        type Super = NSCell;
    }
);
extern_methods!(
    unsafe impl NSActionCell {
        #[method_id(target)]
        pub unsafe fn target(&self) -> Option<Id<Object, Shared>>;
        #[method(setTarget:)]
        pub unsafe fn setTarget(&self, target: Option<&Object>);
        #[method(action)]
        pub unsafe fn action(&self) -> Option<Sel>;
        #[method(setAction:)]
        pub unsafe fn setAction(&self, action: Option<Sel>);
        #[method(tag)]
        pub unsafe fn tag(&self) -> NSInteger;
        #[method(setTag:)]
        pub unsafe fn setTag(&self, tag: NSInteger);
    }
);
