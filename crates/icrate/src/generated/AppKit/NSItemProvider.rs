use crate::AppKit::generated::AppKitDefines::*;
use crate::Foundation::generated::NSGeometry::*;
use crate::Foundation::generated::NSItemProvider::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_methods!(
    #[doc = "NSItemSourceInfo"]
    unsafe impl NSItemProvider {
        #[method(sourceFrame)]
        pub unsafe fn sourceFrame(&self) -> NSRect;
        #[method(containerFrame)]
        pub unsafe fn containerFrame(&self) -> NSRect;
        #[method(preferredPresentationSize)]
        pub unsafe fn preferredPresentationSize(&self) -> NSSize;
    }
);
