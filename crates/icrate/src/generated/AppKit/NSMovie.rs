use super::__exported::QTMovie;
use crate::AppKit::generated::AppKitDefines::*;
use crate::Foundation::generated::Foundation::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSMovie;
    unsafe impl ClassType for NSMovie {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSMovie {
        #[method_id(initWithCoder:)]
        pub unsafe fn initWithCoder(&self, coder: &NSCoder) -> Option<Id<Self, Shared>>;
        #[method_id(init)]
        pub unsafe fn init(&self) -> Option<Id<Self, Shared>>;
        #[method_id(initWithMovie:)]
        pub unsafe fn initWithMovie(&self, movie: &QTMovie) -> Option<Id<Self, Shared>>;
        #[method_id(QTMovie)]
        pub unsafe fn QTMovie(&self) -> Option<Id<QTMovie, Shared>>;
    }
);
