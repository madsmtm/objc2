#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSBrowserCell;
    unsafe impl ClassType for NSBrowserCell {
        type Super = NSCell;
    }
);
extern_methods!(
    unsafe impl NSBrowserCell {
        #[method_id(initTextCell:)]
        pub unsafe fn initTextCell(&self, string: &NSString) -> Id<Self, Shared>;
        #[method_id(initImageCell:)]
        pub unsafe fn initImageCell(&self, image: Option<&NSImage>) -> Id<Self, Shared>;
        #[method_id(initWithCoder:)]
        pub unsafe fn initWithCoder(&self, coder: &NSCoder) -> Id<Self, Shared>;
        #[method_id(branchImage)]
        pub unsafe fn branchImage() -> Option<Id<NSImage, Shared>>;
        #[method_id(highlightedBranchImage)]
        pub unsafe fn highlightedBranchImage() -> Option<Id<NSImage, Shared>>;
        #[method_id(highlightColorInView:)]
        pub unsafe fn highlightColorInView(
            &self,
            controlView: &NSView,
        ) -> Option<Id<NSColor, Shared>>;
        #[method(isLeaf)]
        pub unsafe fn isLeaf(&self) -> bool;
        #[method(setLeaf:)]
        pub unsafe fn setLeaf(&self, leaf: bool);
        #[method(isLoaded)]
        pub unsafe fn isLoaded(&self) -> bool;
        #[method(setLoaded:)]
        pub unsafe fn setLoaded(&self, loaded: bool);
        #[method(reset)]
        pub unsafe fn reset(&self);
        #[method(set)]
        pub unsafe fn set(&self);
        #[method_id(image)]
        pub unsafe fn image(&self) -> Option<Id<NSImage, Shared>>;
        #[method(setImage:)]
        pub unsafe fn setImage(&self, image: Option<&NSImage>);
        #[method_id(alternateImage)]
        pub unsafe fn alternateImage(&self) -> Option<Id<NSImage, Shared>>;
        #[method(setAlternateImage:)]
        pub unsafe fn setAlternateImage(&self, alternateImage: Option<&NSImage>);
    }
);