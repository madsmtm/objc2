use crate::AppKit::generated::AppKitDefines::*;
use crate::AppKit::generated::NSControl::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSColorWell;
    unsafe impl ClassType for NSColorWell {
        type Super = NSControl;
    }
);
extern_methods!(
    unsafe impl NSColorWell {
        #[method(deactivate)]
        pub unsafe fn deactivate(&self);
        #[method(activate:)]
        pub unsafe fn activate(&self, exclusive: bool);
        #[method(isActive)]
        pub unsafe fn isActive(&self) -> bool;
        #[method(drawWellInside:)]
        pub unsafe fn drawWellInside(&self, insideRect: NSRect);
        #[method(isBordered)]
        pub unsafe fn isBordered(&self) -> bool;
        #[method(setBordered:)]
        pub unsafe fn setBordered(&self, bordered: bool);
        #[method(takeColorFrom:)]
        pub unsafe fn takeColorFrom(&self, sender: Option<&Object>);
        #[method_id(color)]
        pub unsafe fn color(&self) -> Id<NSColor, Shared>;
        #[method(setColor:)]
        pub unsafe fn setColor(&self, color: &NSColor);
    }
);
