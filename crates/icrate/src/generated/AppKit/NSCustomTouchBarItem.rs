use crate::AppKit::generated::AppKitDefines::*;
use crate::AppKit::generated::NSTouchBarItem::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSCustomTouchBarItem;
    unsafe impl ClassType for NSCustomTouchBarItem {
        type Super = NSTouchBarItem;
    }
);
extern_methods!(
    unsafe impl NSCustomTouchBarItem {
        #[method_id(view)]
        pub unsafe fn view(&self) -> Id<NSView, Shared>;
        #[method(setView:)]
        pub unsafe fn setView(&self, view: &NSView);
        #[method_id(viewController)]
        pub unsafe fn viewController(&self) -> Option<Id<NSViewController, Shared>>;
        #[method(setViewController:)]
        pub unsafe fn setViewController(&self, viewController: Option<&NSViewController>);
        #[method_id(customizationLabel)]
        pub unsafe fn customizationLabel(&self) -> Id<NSString, Shared>;
        #[method(setCustomizationLabel:)]
        pub unsafe fn setCustomizationLabel(&self, customizationLabel: Option<&NSString>);
    }
);
