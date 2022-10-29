#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSSharingServicePickerToolbarItem;
    unsafe impl ClassType for NSSharingServicePickerToolbarItem {
        type Super = NSToolbarItem;
    }
);
extern_methods!(
    unsafe impl NSSharingServicePickerToolbarItem {
        #[method_id(delegate)]
        pub unsafe fn delegate(
            &self,
        ) -> Option<Id<NSSharingServicePickerToolbarItemDelegate, Shared>>;
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&NSSharingServicePickerToolbarItemDelegate>,
        );
    }
);
pub type NSSharingServicePickerToolbarItemDelegate = NSObject;