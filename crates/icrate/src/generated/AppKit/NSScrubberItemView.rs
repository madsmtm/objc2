#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSScrubberArrangedView;
    unsafe impl ClassType for NSScrubberArrangedView {
        type Super = NSView;
    }
);
extern_methods!(
    unsafe impl NSScrubberArrangedView {
        #[method(isSelected)]
        pub unsafe fn isSelected(&self) -> bool;
        #[method(setSelected:)]
        pub unsafe fn setSelected(&self, selected: bool);
        #[method(isHighlighted)]
        pub unsafe fn isHighlighted(&self) -> bool;
        #[method(setHighlighted:)]
        pub unsafe fn setHighlighted(&self, highlighted: bool);
        #[method(applyLayoutAttributes:)]
        pub unsafe fn applyLayoutAttributes(&self, layoutAttributes: &NSScrubberLayoutAttributes);
    }
);
extern_class!(
    #[derive(Debug)]
    pub struct NSScrubberSelectionView;
    unsafe impl ClassType for NSScrubberSelectionView {
        type Super = NSScrubberArrangedView;
    }
);
extern_methods!(
    unsafe impl NSScrubberSelectionView {}
);
extern_class!(
    #[derive(Debug)]
    pub struct NSScrubberItemView;
    unsafe impl ClassType for NSScrubberItemView {
        type Super = NSScrubberArrangedView;
    }
);
extern_methods!(
    unsafe impl NSScrubberItemView {}
);
extern_class!(
    #[derive(Debug)]
    pub struct NSScrubberTextItemView;
    unsafe impl ClassType for NSScrubberTextItemView {
        type Super = NSScrubberItemView;
    }
);
extern_methods!(
    unsafe impl NSScrubberTextItemView {
        #[method_id(textField)]
        pub unsafe fn textField(&self) -> Id<NSTextField, Shared>;
        #[method_id(title)]
        pub unsafe fn title(&self) -> Id<NSString, Shared>;
        #[method(setTitle:)]
        pub unsafe fn setTitle(&self, title: &NSString);
    }
);
extern_class!(
    #[derive(Debug)]
    pub struct NSScrubberImageItemView;
    unsafe impl ClassType for NSScrubberImageItemView {
        type Super = NSScrubberItemView;
    }
);
extern_methods!(
    unsafe impl NSScrubberImageItemView {
        #[method_id(imageView)]
        pub unsafe fn imageView(&self) -> Id<NSImageView, Shared>;
        #[method_id(image)]
        pub unsafe fn image(&self) -> Id<NSImage, Shared>;
        #[method(setImage:)]
        pub unsafe fn setImage(&self, image: &NSImage);
        #[method(imageAlignment)]
        pub unsafe fn imageAlignment(&self) -> NSImageAlignment;
        #[method(setImageAlignment:)]
        pub unsafe fn setImageAlignment(&self, imageAlignment: NSImageAlignment);
    }
);
