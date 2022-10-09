use super::__exported::NSOperationQueue;
use super::__exported::NSView;
use crate::AppKit::generated::AppKitDefines::*;
use crate::AppKit::generated::NSNibDeclarations::*;
use crate::Foundation::generated::NSArray::*;
use crate::Foundation::generated::NSGeometry::*;
use crate::Foundation::generated::NSObject::*;
use crate::Foundation::generated::NSRange::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
pub type NSPasteboardTypeTextFinderOptionKey = NSString;
extern_class!(
    #[derive(Debug)]
    pub struct NSTextFinder;
    unsafe impl ClassType for NSTextFinder {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSTextFinder {
        #[method_id(init)]
        pub unsafe fn init(&self) -> Id<Self, Shared>;
        #[method_id(initWithCoder:)]
        pub unsafe fn initWithCoder(&self, coder: &NSCoder) -> Id<Self, Shared>;
        #[method_id(client)]
        pub unsafe fn client(&self) -> Option<Id<NSTextFinderClient, Shared>>;
        #[method(setClient:)]
        pub unsafe fn setClient(&self, client: Option<&NSTextFinderClient>);
        #[method(performAction:)]
        pub unsafe fn performAction(&self, op: NSTextFinderAction);
        #[method(validateAction:)]
        pub unsafe fn validateAction(&self, op: NSTextFinderAction) -> bool;
        #[method_id(findBarContainer)]
        pub unsafe fn findBarContainer(&self) -> Option<Id<NSTextFinderBarContainer, Shared>>;
        #[method(setFindBarContainer:)]
        pub unsafe fn setFindBarContainer(
            &self,
            findBarContainer: Option<&NSTextFinderBarContainer>,
        );
        #[method(cancelFindIndicator)]
        pub unsafe fn cancelFindIndicator(&self);
        #[method(findIndicatorNeedsUpdate)]
        pub unsafe fn findIndicatorNeedsUpdate(&self) -> bool;
        #[method(setFindIndicatorNeedsUpdate:)]
        pub unsafe fn setFindIndicatorNeedsUpdate(&self, findIndicatorNeedsUpdate: bool);
        #[method(isIncrementalSearchingEnabled)]
        pub unsafe fn isIncrementalSearchingEnabled(&self) -> bool;
        #[method(setIncrementalSearchingEnabled:)]
        pub unsafe fn setIncrementalSearchingEnabled(&self, incrementalSearchingEnabled: bool);
        #[method(incrementalSearchingShouldDimContentView)]
        pub unsafe fn incrementalSearchingShouldDimContentView(&self) -> bool;
        #[method(setIncrementalSearchingShouldDimContentView:)]
        pub unsafe fn setIncrementalSearchingShouldDimContentView(
            &self,
            incrementalSearchingShouldDimContentView: bool,
        );
        #[method_id(incrementalMatchRanges)]
        pub unsafe fn incrementalMatchRanges(&self) -> Id<NSArray<NSValue>, Shared>;
        #[method(drawIncrementalMatchHighlightInRect:)]
        pub unsafe fn drawIncrementalMatchHighlightInRect(rect: NSRect);
        #[method(noteClientStringWillChange)]
        pub unsafe fn noteClientStringWillChange(&self);
    }
);
pub type NSTextFinderClient = NSObject;
pub type NSTextFinderBarContainer = NSObject;
