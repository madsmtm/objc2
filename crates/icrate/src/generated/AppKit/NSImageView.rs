use super::__exported::NSImageSymbolConfiguration;
use crate::AppKit::generated::AppKitDefines::*;
use crate::AppKit::generated::NSControl::*;
use crate::AppKit::generated::NSImageCell::*;
use crate::AppKit::generated::NSMenu::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSImageView;
    unsafe impl ClassType for NSImageView {
        type Super = NSControl;
    }
);
extern_methods!(
    unsafe impl NSImageView {
        #[method_id(imageViewWithImage:)]
        pub unsafe fn imageViewWithImage(image: &NSImage) -> Id<Self, Shared>;
        #[method_id(image)]
        pub unsafe fn image(&self) -> Option<Id<NSImage, Shared>>;
        #[method(setImage:)]
        pub unsafe fn setImage(&self, image: Option<&NSImage>);
        #[method(isEditable)]
        pub unsafe fn isEditable(&self) -> bool;
        #[method(setEditable:)]
        pub unsafe fn setEditable(&self, editable: bool);
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
        #[method_id(symbolConfiguration)]
        pub unsafe fn symbolConfiguration(&self) -> Option<Id<NSImageSymbolConfiguration, Shared>>;
        #[method(setSymbolConfiguration:)]
        pub unsafe fn setSymbolConfiguration(
            &self,
            symbolConfiguration: Option<&NSImageSymbolConfiguration>,
        );
        #[method_id(contentTintColor)]
        pub unsafe fn contentTintColor(&self) -> Option<Id<NSColor, Shared>>;
        #[method(setContentTintColor:)]
        pub unsafe fn setContentTintColor(&self, contentTintColor: Option<&NSColor>);
        #[method(animates)]
        pub unsafe fn animates(&self) -> bool;
        #[method(setAnimates:)]
        pub unsafe fn setAnimates(&self, animates: bool);
        #[method(allowsCutCopyPaste)]
        pub unsafe fn allowsCutCopyPaste(&self) -> bool;
        #[method(setAllowsCutCopyPaste:)]
        pub unsafe fn setAllowsCutCopyPaste(&self, allowsCutCopyPaste: bool);
    }
);
