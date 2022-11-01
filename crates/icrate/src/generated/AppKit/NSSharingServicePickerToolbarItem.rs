//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;

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
