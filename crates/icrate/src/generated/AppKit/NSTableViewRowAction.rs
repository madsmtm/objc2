#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSTableViewRowAction;
    unsafe impl ClassType for NSTableViewRowAction {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSTableViewRowAction {
        #[method_id(rowActionWithStyle:title:handler:)]
        pub unsafe fn rowActionWithStyle_title_handler(
            style: NSTableViewRowActionStyle,
            title: &NSString,
            handler: TodoBlock,
        ) -> Id<Self, Shared>;
        #[method(style)]
        pub unsafe fn style(&self) -> NSTableViewRowActionStyle;
        #[method_id(title)]
        pub unsafe fn title(&self) -> Id<NSString, Shared>;
        #[method(setTitle:)]
        pub unsafe fn setTitle(&self, title: &NSString);
        #[method_id(backgroundColor)]
        pub unsafe fn backgroundColor(&self) -> Id<NSColor, Shared>;
        #[method(setBackgroundColor:)]
        pub unsafe fn setBackgroundColor(&self, backgroundColor: Option<&NSColor>);
        #[method_id(image)]
        pub unsafe fn image(&self) -> Option<Id<NSImage, Shared>>;
        #[method(setImage:)]
        pub unsafe fn setImage(&self, image: Option<&NSImage>);
    }
);
