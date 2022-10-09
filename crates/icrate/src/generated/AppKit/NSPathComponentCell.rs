use super::__exported::NSImage;
use super::__exported::NSString;
use super::__exported::NSURL;
use crate::AppKit::generated::AppKitDefines::*;
use crate::AppKit::generated::NSTextFieldCell::*;
use crate::Foundation::generated::NSObject::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSPathComponentCell;
    unsafe impl ClassType for NSPathComponentCell {
        type Super = NSTextFieldCell;
    }
);
extern_methods!(
    unsafe impl NSPathComponentCell {
        #[method_id(image)]
        pub unsafe fn image(&self) -> Option<Id<NSImage, Shared>>;
        #[method(setImage:)]
        pub unsafe fn setImage(&self, image: Option<&NSImage>);
        #[method_id(URL)]
        pub unsafe fn URL(&self) -> Option<Id<NSURL, Shared>>;
        #[method(setURL:)]
        pub unsafe fn setURL(&self, URL: Option<&NSURL>);
    }
);
