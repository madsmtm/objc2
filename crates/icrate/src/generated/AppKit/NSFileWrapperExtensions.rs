#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_methods!(
    #[doc = "NSExtensions"]
    unsafe impl NSFileWrapper {
        #[method_id(icon)]
        pub unsafe fn icon(&self) -> Option<Id<NSImage, Shared>>;
        #[method(setIcon:)]
        pub unsafe fn setIcon(&self, icon: Option<&NSImage>);
    }
);
