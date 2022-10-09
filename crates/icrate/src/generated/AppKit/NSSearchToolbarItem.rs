use super::__exported::NSSearchField;
use super::__exported::NSView;
use crate::AppKit::generated::NSToolbarItem::*;
use crate::Foundation::generated::Foundation::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSSearchToolbarItem;
    unsafe impl ClassType for NSSearchToolbarItem {
        type Super = NSToolbarItem;
    }
);
extern_methods!(
    unsafe impl NSSearchToolbarItem {
        #[method_id(searchField)]
        pub unsafe fn searchField(&self) -> Id<NSSearchField, Shared>;
        #[method(setSearchField:)]
        pub unsafe fn setSearchField(&self, searchField: &NSSearchField);
        #[method_id(view)]
        pub unsafe fn view(&self) -> Option<Id<NSView, Shared>>;
        #[method(setView:)]
        pub unsafe fn setView(&self, view: Option<&NSView>);
        #[method(resignsFirstResponderWithCancel)]
        pub unsafe fn resignsFirstResponderWithCancel(&self) -> bool;
        #[method(setResignsFirstResponderWithCancel:)]
        pub unsafe fn setResignsFirstResponderWithCancel(
            &self,
            resignsFirstResponderWithCancel: bool,
        );
        #[method(preferredWidthForSearchField)]
        pub unsafe fn preferredWidthForSearchField(&self) -> CGFloat;
        #[method(setPreferredWidthForSearchField:)]
        pub unsafe fn setPreferredWidthForSearchField(&self, preferredWidthForSearchField: CGFloat);
        #[method(beginSearchInteraction)]
        pub unsafe fn beginSearchInteraction(&self);
        #[method(endSearchInteraction)]
        pub unsafe fn endSearchInteraction(&self);
    }
);
