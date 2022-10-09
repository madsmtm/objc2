use super::__exported::NSView;
use crate::AppKit::generated::AppKitDefines::*;
use crate::AppKit::generated::NSApplication::*;
use crate::Foundation::generated::NSGeometry::*;
use crate::Foundation::generated::NSObject::*;
use crate::Foundation::generated::NSString::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSDockTile;
    unsafe impl ClassType for NSDockTile {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSDockTile {
        #[method(size)]
        pub unsafe fn size(&self) -> NSSize;
        #[method_id(contentView)]
        pub unsafe fn contentView(&self) -> Option<Id<NSView, Shared>>;
        #[method(setContentView:)]
        pub unsafe fn setContentView(&self, contentView: Option<&NSView>);
        #[method(display)]
        pub unsafe fn display(&self);
        #[method(showsApplicationBadge)]
        pub unsafe fn showsApplicationBadge(&self) -> bool;
        #[method(setShowsApplicationBadge:)]
        pub unsafe fn setShowsApplicationBadge(&self, showsApplicationBadge: bool);
        #[method_id(badgeLabel)]
        pub unsafe fn badgeLabel(&self) -> Option<Id<NSString, Shared>>;
        #[method(setBadgeLabel:)]
        pub unsafe fn setBadgeLabel(&self, badgeLabel: Option<&NSString>);
        #[method_id(owner)]
        pub unsafe fn owner(&self) -> Option<Id<Object, Shared>>;
    }
);
use super::__exported::NSMenu;
pub type NSDockTilePlugIn = NSObject;
