use crate::AppKit::generated::NSToolbarItem::*;
use crate::Foundation::generated::Foundation::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSToolbarItemGroup;
    unsafe impl ClassType for NSToolbarItemGroup {
        type Super = NSToolbarItem;
    }
);
extern_methods!(
    unsafe impl NSToolbarItemGroup {
        #[method_id(groupWithItemIdentifier:titles:selectionMode:labels:target:action:)]
        pub unsafe fn groupWithItemIdentifier_titles_selectionMode_labels_target_action(
            itemIdentifier: &NSToolbarItemIdentifier,
            titles: &NSArray<NSString>,
            selectionMode: NSToolbarItemGroupSelectionMode,
            labels: Option<&NSArray<NSString>>,
            target: Option<&Object>,
            action: Option<Sel>,
        ) -> Id<Self, Shared>;
        #[method_id(groupWithItemIdentifier:images:selectionMode:labels:target:action:)]
        pub unsafe fn groupWithItemIdentifier_images_selectionMode_labels_target_action(
            itemIdentifier: &NSToolbarItemIdentifier,
            images: &NSArray<NSImage>,
            selectionMode: NSToolbarItemGroupSelectionMode,
            labels: Option<&NSArray<NSString>>,
            target: Option<&Object>,
            action: Option<Sel>,
        ) -> Id<Self, Shared>;
        #[method_id(subitems)]
        pub unsafe fn subitems(&self) -> Id<NSArray<NSToolbarItem>, Shared>;
        #[method(setSubitems:)]
        pub unsafe fn setSubitems(&self, subitems: &NSArray<NSToolbarItem>);
        #[method(controlRepresentation)]
        pub unsafe fn controlRepresentation(&self) -> NSToolbarItemGroupControlRepresentation;
        #[method(setControlRepresentation:)]
        pub unsafe fn setControlRepresentation(
            &self,
            controlRepresentation: NSToolbarItemGroupControlRepresentation,
        );
        #[method(selectionMode)]
        pub unsafe fn selectionMode(&self) -> NSToolbarItemGroupSelectionMode;
        #[method(setSelectionMode:)]
        pub unsafe fn setSelectionMode(&self, selectionMode: NSToolbarItemGroupSelectionMode);
        #[method(selectedIndex)]
        pub unsafe fn selectedIndex(&self) -> NSInteger;
        #[method(setSelectedIndex:)]
        pub unsafe fn setSelectedIndex(&self, selectedIndex: NSInteger);
        #[method(setSelected:atIndex:)]
        pub unsafe fn setSelected_atIndex(&self, selected: bool, index: NSInteger);
        #[method(isSelectedAtIndex:)]
        pub unsafe fn isSelectedAtIndex(&self, index: NSInteger) -> bool;
    }
);
