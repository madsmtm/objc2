#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSCachedImageRep;
    unsafe impl ClassType for NSCachedImageRep {
        type Super = NSImageRep;
    }
);
extern_methods!(
    unsafe impl NSCachedImageRep {
        #[method_id(initWithWindow:rect:)]
        pub unsafe fn initWithWindow_rect(
            &self,
            win: Option<&NSWindow>,
            rect: NSRect,
        ) -> Option<Id<Object, Shared>>;
        #[method_id(initWithSize:depth:separate:alpha:)]
        pub unsafe fn initWithSize_depth_separate_alpha(
            &self,
            size: NSSize,
            depth: NSWindowDepth,
            flag: bool,
            alpha: bool,
        ) -> Option<Id<Object, Shared>>;
        #[method_id(window)]
        pub unsafe fn window(&self) -> Option<Id<NSWindow, Shared>>;
        #[method(rect)]
        pub unsafe fn rect(&self) -> NSRect;
    }
);
