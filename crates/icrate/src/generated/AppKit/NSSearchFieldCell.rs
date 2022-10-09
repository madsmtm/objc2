use super::__exported::NSButtonCell;
use super::__exported::NSImage;
use super::__exported::NSMenu;
use super::__exported::NSMutableArray;
use super::__exported::NSTimer;
use crate::AppKit::generated::AppKitDefines::*;
use crate::AppKit::generated::NSSearchField::*;
use crate::AppKit::generated::NSTextFieldCell::*;
use crate::Foundation::generated::NSArray::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSSearchFieldCell;
    unsafe impl ClassType for NSSearchFieldCell {
        type Super = NSTextFieldCell;
    }
);
extern_methods!(
    unsafe impl NSSearchFieldCell {
        #[method_id(initTextCell:)]
        pub unsafe fn initTextCell(&self, string: &NSString) -> Id<Self, Shared>;
        #[method_id(initWithCoder:)]
        pub unsafe fn initWithCoder(&self, coder: &NSCoder) -> Id<Self, Shared>;
        #[method_id(initImageCell:)]
        pub unsafe fn initImageCell(&self, image: Option<&NSImage>) -> Id<Self, Shared>;
        #[method_id(searchButtonCell)]
        pub unsafe fn searchButtonCell(&self) -> Option<Id<NSButtonCell, Shared>>;
        #[method(setSearchButtonCell:)]
        pub unsafe fn setSearchButtonCell(&self, searchButtonCell: Option<&NSButtonCell>);
        #[method_id(cancelButtonCell)]
        pub unsafe fn cancelButtonCell(&self) -> Option<Id<NSButtonCell, Shared>>;
        #[method(setCancelButtonCell:)]
        pub unsafe fn setCancelButtonCell(&self, cancelButtonCell: Option<&NSButtonCell>);
        #[method(resetSearchButtonCell)]
        pub unsafe fn resetSearchButtonCell(&self);
        #[method(resetCancelButtonCell)]
        pub unsafe fn resetCancelButtonCell(&self);
        #[method(searchTextRectForBounds:)]
        pub unsafe fn searchTextRectForBounds(&self, rect: NSRect) -> NSRect;
        #[method(searchButtonRectForBounds:)]
        pub unsafe fn searchButtonRectForBounds(&self, rect: NSRect) -> NSRect;
        #[method(cancelButtonRectForBounds:)]
        pub unsafe fn cancelButtonRectForBounds(&self, rect: NSRect) -> NSRect;
        #[method_id(searchMenuTemplate)]
        pub unsafe fn searchMenuTemplate(&self) -> Option<Id<NSMenu, Shared>>;
        #[method(setSearchMenuTemplate:)]
        pub unsafe fn setSearchMenuTemplate(&self, searchMenuTemplate: Option<&NSMenu>);
        #[method(sendsWholeSearchString)]
        pub unsafe fn sendsWholeSearchString(&self) -> bool;
        #[method(setSendsWholeSearchString:)]
        pub unsafe fn setSendsWholeSearchString(&self, sendsWholeSearchString: bool);
        #[method(maximumRecents)]
        pub unsafe fn maximumRecents(&self) -> NSInteger;
        #[method(setMaximumRecents:)]
        pub unsafe fn setMaximumRecents(&self, maximumRecents: NSInteger);
        #[method_id(recentSearches)]
        pub unsafe fn recentSearches(&self) -> Id<NSArray<NSString>, Shared>;
        #[method(setRecentSearches:)]
        pub unsafe fn setRecentSearches(&self, recentSearches: Option<&NSArray<NSString>>);
        #[method_id(recentsAutosaveName)]
        pub unsafe fn recentsAutosaveName(
            &self,
        ) -> Option<Id<NSSearchFieldRecentsAutosaveName, Shared>>;
        #[method(setRecentsAutosaveName:)]
        pub unsafe fn setRecentsAutosaveName(
            &self,
            recentsAutosaveName: Option<&NSSearchFieldRecentsAutosaveName>,
        );
        #[method(sendsSearchStringImmediately)]
        pub unsafe fn sendsSearchStringImmediately(&self) -> bool;
        #[method(setSendsSearchStringImmediately:)]
        pub unsafe fn setSendsSearchStringImmediately(&self, sendsSearchStringImmediately: bool);
    }
);
