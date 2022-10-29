#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSSharingServicePickerTouchBarItem;
    unsafe impl ClassType for NSSharingServicePickerTouchBarItem {
        type Super = NSTouchBarItem;
    }
);
extern_methods!(
    unsafe impl NSSharingServicePickerTouchBarItem {
        #[method_id(delegate)]
        pub unsafe fn delegate(
            &self,
        ) -> Option<Id<NSSharingServicePickerTouchBarItemDelegate, Shared>>;
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&NSSharingServicePickerTouchBarItemDelegate>,
        );
        #[method(isEnabled)]
        pub unsafe fn isEnabled(&self) -> bool;
        #[method(setEnabled:)]
        pub unsafe fn setEnabled(&self, enabled: bool);
        #[method_id(buttonTitle)]
        pub unsafe fn buttonTitle(&self) -> Id<NSString, Shared>;
        #[method(setButtonTitle:)]
        pub unsafe fn setButtonTitle(&self, buttonTitle: &NSString);
        #[method_id(buttonImage)]
        pub unsafe fn buttonImage(&self) -> Option<Id<NSImage, Shared>>;
        #[method(setButtonImage:)]
        pub unsafe fn setButtonImage(&self, buttonImage: Option<&NSImage>);
    }
);
pub type NSSharingServicePickerTouchBarItemDelegate = NSObject;