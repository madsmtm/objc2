#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSImageCell;
    unsafe impl ClassType for NSImageCell {
        type Super = NSCell;
    }
);
extern_methods!(
    unsafe impl NSImageCell {
        #[method(imageAlignment)]
        pub unsafe fn imageAlignment(&self) -> NSImageAlignment;
        #[method(setImageAlignment:)]
        pub unsafe fn setImageAlignment(&self, imageAlignment: NSImageAlignment);
        #[method(imageScaling)]
        pub unsafe fn imageScaling(&self) -> NSImageScaling;
        #[method(setImageScaling:)]
        pub unsafe fn setImageScaling(&self, imageScaling: NSImageScaling);
        #[method(imageFrameStyle)]
        pub unsafe fn imageFrameStyle(&self) -> NSImageFrameStyle;
        #[method(setImageFrameStyle:)]
        pub unsafe fn setImageFrameStyle(&self, imageFrameStyle: NSImageFrameStyle);
    }
);
