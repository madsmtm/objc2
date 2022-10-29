#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSTableCellView;
    unsafe impl ClassType for NSTableCellView {
        type Super = NSView;
    }
);
extern_methods!(
    unsafe impl NSTableCellView {
        #[method_id(objectValue)]
        pub unsafe fn objectValue(&self) -> Option<Id<Object, Shared>>;
        #[method(setObjectValue:)]
        pub unsafe fn setObjectValue(&self, objectValue: Option<&Object>);
        #[method_id(textField)]
        pub unsafe fn textField(&self) -> Option<Id<NSTextField, Shared>>;
        #[method(setTextField:)]
        pub unsafe fn setTextField(&self, textField: Option<&NSTextField>);
        #[method_id(imageView)]
        pub unsafe fn imageView(&self) -> Option<Id<NSImageView, Shared>>;
        #[method(setImageView:)]
        pub unsafe fn setImageView(&self, imageView: Option<&NSImageView>);
        #[method(backgroundStyle)]
        pub unsafe fn backgroundStyle(&self) -> NSBackgroundStyle;
        #[method(setBackgroundStyle:)]
        pub unsafe fn setBackgroundStyle(&self, backgroundStyle: NSBackgroundStyle);
        #[method(rowSizeStyle)]
        pub unsafe fn rowSizeStyle(&self) -> NSTableViewRowSizeStyle;
        #[method(setRowSizeStyle:)]
        pub unsafe fn setRowSizeStyle(&self, rowSizeStyle: NSTableViewRowSizeStyle);
        #[method_id(draggingImageComponents)]
        pub unsafe fn draggingImageComponents(
            &self,
        ) -> Id<NSArray<NSDraggingImageComponent>, Shared>;
    }
);
