use super::__exported::CKShare;
use super::__exported::NSImage;
use super::__exported::NSMenuItem;
use super::__exported::NSView;
use crate::AppKit::generated::NSMenu::*;
use crate::AppKit::generated::NSText::*;
use crate::AppKit::generated::NSToolbar::*;
use crate::AppKit::generated::NSUserInterfaceValidation::*;
use crate::Foundation::generated::Foundation::*;
use crate::Foundation::generated::NSGeometry::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
pub type NSToolbarItemVisibilityPriority = NSInteger;
extern_class!(
    #[derive(Debug)]
    pub struct NSToolbarItem;
    unsafe impl ClassType for NSToolbarItem {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSToolbarItem {
        #[method_id(initWithItemIdentifier:)]
        pub unsafe fn initWithItemIdentifier(
            &self,
            itemIdentifier: &NSToolbarItemIdentifier,
        ) -> Id<Self, Shared>;
        #[method_id(itemIdentifier)]
        pub unsafe fn itemIdentifier(&self) -> Id<NSToolbarItemIdentifier, Shared>;
        #[method_id(toolbar)]
        pub unsafe fn toolbar(&self) -> Option<Id<NSToolbar, Shared>>;
        #[method_id(label)]
        pub unsafe fn label(&self) -> Id<NSString, Shared>;
        #[method(setLabel:)]
        pub unsafe fn setLabel(&self, label: &NSString);
        #[method_id(paletteLabel)]
        pub unsafe fn paletteLabel(&self) -> Id<NSString, Shared>;
        #[method(setPaletteLabel:)]
        pub unsafe fn setPaletteLabel(&self, paletteLabel: &NSString);
        #[method_id(toolTip)]
        pub unsafe fn toolTip(&self) -> Option<Id<NSString, Shared>>;
        #[method(setToolTip:)]
        pub unsafe fn setToolTip(&self, toolTip: Option<&NSString>);
        #[method_id(menuFormRepresentation)]
        pub unsafe fn menuFormRepresentation(&self) -> Option<Id<NSMenuItem, Shared>>;
        #[method(setMenuFormRepresentation:)]
        pub unsafe fn setMenuFormRepresentation(&self, menuFormRepresentation: Option<&NSMenuItem>);
        #[method(tag)]
        pub unsafe fn tag(&self) -> NSInteger;
        #[method(setTag:)]
        pub unsafe fn setTag(&self, tag: NSInteger);
        #[method_id(target)]
        pub unsafe fn target(&self) -> Option<Id<Object, Shared>>;
        #[method(setTarget:)]
        pub unsafe fn setTarget(&self, target: Option<&Object>);
        #[method(action)]
        pub unsafe fn action(&self) -> Option<Sel>;
        #[method(setAction:)]
        pub unsafe fn setAction(&self, action: Option<Sel>);
        #[method(isEnabled)]
        pub unsafe fn isEnabled(&self) -> bool;
        #[method(setEnabled:)]
        pub unsafe fn setEnabled(&self, enabled: bool);
        #[method_id(image)]
        pub unsafe fn image(&self) -> Option<Id<NSImage, Shared>>;
        #[method(setImage:)]
        pub unsafe fn setImage(&self, image: Option<&NSImage>);
        #[method_id(title)]
        pub unsafe fn title(&self) -> Id<NSString, Shared>;
        #[method(setTitle:)]
        pub unsafe fn setTitle(&self, title: &NSString);
        #[method(isBordered)]
        pub unsafe fn isBordered(&self) -> bool;
        #[method(setBordered:)]
        pub unsafe fn setBordered(&self, bordered: bool);
        #[method(isNavigational)]
        pub unsafe fn isNavigational(&self) -> bool;
        #[method(setNavigational:)]
        pub unsafe fn setNavigational(&self, navigational: bool);
        #[method_id(view)]
        pub unsafe fn view(&self) -> Option<Id<NSView, Shared>>;
        #[method(setView:)]
        pub unsafe fn setView(&self, view: Option<&NSView>);
        #[method(minSize)]
        pub unsafe fn minSize(&self) -> NSSize;
        #[method(setMinSize:)]
        pub unsafe fn setMinSize(&self, minSize: NSSize);
        #[method(maxSize)]
        pub unsafe fn maxSize(&self) -> NSSize;
        #[method(setMaxSize:)]
        pub unsafe fn setMaxSize(&self, maxSize: NSSize);
        #[method(visibilityPriority)]
        pub unsafe fn visibilityPriority(&self) -> NSToolbarItemVisibilityPriority;
        #[method(setVisibilityPriority:)]
        pub unsafe fn setVisibilityPriority(
            &self,
            visibilityPriority: NSToolbarItemVisibilityPriority,
        );
        #[method(validate)]
        pub unsafe fn validate(&self);
        #[method(autovalidates)]
        pub unsafe fn autovalidates(&self) -> bool;
        #[method(setAutovalidates:)]
        pub unsafe fn setAutovalidates(&self, autovalidates: bool);
        #[method(allowsDuplicatesInToolbar)]
        pub unsafe fn allowsDuplicatesInToolbar(&self) -> bool;
    }
);
extern_methods!(
    unsafe impl NSToolbarItem {}
);
pub type NSToolbarItemValidation = NSObject;
extern_methods!(
    #[doc = "NSToolbarItemValidation"]
    unsafe impl NSObject {
        #[method(validateToolbarItem:)]
        pub unsafe fn validateToolbarItem(&self, item: &NSToolbarItem) -> bool;
    }
);
pub type NSCloudSharingValidation = NSObject;
