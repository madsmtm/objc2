//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};

extern_methods!(
    /// NSItemSourceInfo
    unsafe impl NSItemProvider {
        #[method(sourceFrame)]
        pub unsafe fn sourceFrame(&self) -> NSRect;

        #[method(containerFrame)]
        pub unsafe fn containerFrame(&self) -> NSRect;

        #[method(preferredPresentationSize)]
        pub unsafe fn preferredPresentationSize(&self) -> NSSize;
    }
);
