use crate::AppKit::generated::AppKitDefines::*;
use crate::AppKit::generated::NSTextField::*;
use crate::Foundation::generated::NSArray::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
pub type NSSearchFieldRecentsAutosaveName = NSString;
pub type NSSearchFieldDelegate = NSObject;
extern_class!(
    #[derive(Debug)]
    pub struct NSSearchField;
    unsafe impl ClassType for NSSearchField {
        type Super = NSTextField;
    }
);
extern_methods!(
    unsafe impl NSSearchField {
        #[method(searchTextBounds)]
        pub unsafe fn searchTextBounds(&self) -> NSRect;
        #[method(searchButtonBounds)]
        pub unsafe fn searchButtonBounds(&self) -> NSRect;
        #[method(cancelButtonBounds)]
        pub unsafe fn cancelButtonBounds(&self) -> NSRect;
        #[method_id(recentSearches)]
        pub unsafe fn recentSearches(&self) -> Id<NSArray<NSString>, Shared>;
        #[method(setRecentSearches:)]
        pub unsafe fn setRecentSearches(&self, recentSearches: &NSArray<NSString>);
        #[method_id(recentsAutosaveName)]
        pub unsafe fn recentsAutosaveName(
            &self,
        ) -> Option<Id<NSSearchFieldRecentsAutosaveName, Shared>>;
        #[method(setRecentsAutosaveName:)]
        pub unsafe fn setRecentsAutosaveName(
            &self,
            recentsAutosaveName: Option<&NSSearchFieldRecentsAutosaveName>,
        );
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
        #[method(sendsSearchStringImmediately)]
        pub unsafe fn sendsSearchStringImmediately(&self) -> bool;
        #[method(setSendsSearchStringImmediately:)]
        pub unsafe fn setSendsSearchStringImmediately(&self, sendsSearchStringImmediately: bool);
        #[method_id(delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<NSSearchFieldDelegate, Shared>>;
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&NSSearchFieldDelegate>);
    }
);
extern_methods!(
    #[doc = "NSSearchField_Deprecated"]
    unsafe impl NSSearchField {
        #[method(rectForSearchTextWhenCentered:)]
        pub unsafe fn rectForSearchTextWhenCentered(&self, isCentered: bool) -> NSRect;
        #[method(rectForSearchButtonWhenCentered:)]
        pub unsafe fn rectForSearchButtonWhenCentered(&self, isCentered: bool) -> NSRect;
        #[method(rectForCancelButtonWhenCentered:)]
        pub unsafe fn rectForCancelButtonWhenCentered(&self, isCentered: bool) -> NSRect;
        #[method(centersPlaceholder)]
        pub unsafe fn centersPlaceholder(&self) -> bool;
        #[method(setCentersPlaceholder:)]
        pub unsafe fn setCentersPlaceholder(&self, centersPlaceholder: bool);
    }
);
