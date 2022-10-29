#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSWindowTab;
    unsafe impl ClassType for NSWindowTab {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSWindowTab {
        #[method_id(title)]
        pub unsafe fn title(&self) -> Id<NSString, Shared>;
        #[method(setTitle:)]
        pub unsafe fn setTitle(&self, title: Option<&NSString>);
        #[method_id(attributedTitle)]
        pub unsafe fn attributedTitle(&self) -> Option<Id<NSAttributedString, Shared>>;
        #[method(setAttributedTitle:)]
        pub unsafe fn setAttributedTitle(&self, attributedTitle: Option<&NSAttributedString>);
        #[method_id(toolTip)]
        pub unsafe fn toolTip(&self) -> Id<NSString, Shared>;
        #[method(setToolTip:)]
        pub unsafe fn setToolTip(&self, toolTip: Option<&NSString>);
        #[method_id(accessoryView)]
        pub unsafe fn accessoryView(&self) -> Option<Id<NSView, Shared>>;
        #[method(setAccessoryView:)]
        pub unsafe fn setAccessoryView(&self, accessoryView: Option<&NSView>);
    }
);
