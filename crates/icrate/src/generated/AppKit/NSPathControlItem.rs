#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSPathControlItem;
    unsafe impl ClassType for NSPathControlItem {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSPathControlItem {
        #[method_id(title)]
        pub unsafe fn title(&self) -> Id<NSString, Shared>;
        #[method(setTitle:)]
        pub unsafe fn setTitle(&self, title: &NSString);
        #[method_id(attributedTitle)]
        pub unsafe fn attributedTitle(&self) -> Id<NSAttributedString, Shared>;
        #[method(setAttributedTitle:)]
        pub unsafe fn setAttributedTitle(&self, attributedTitle: &NSAttributedString);
        #[method_id(image)]
        pub unsafe fn image(&self) -> Option<Id<NSImage, Shared>>;
        #[method(setImage:)]
        pub unsafe fn setImage(&self, image: Option<&NSImage>);
        #[method_id(URL)]
        pub unsafe fn URL(&self) -> Option<Id<NSURL, Shared>>;
    }
);
