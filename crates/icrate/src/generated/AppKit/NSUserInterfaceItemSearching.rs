use crate::AppKit::generated::AppKitDefines::*;
use crate::AppKit::generated::NSApplication::*;
use crate::Foundation::generated::NSArray::*;
use crate::Foundation::generated::NSString::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
pub type NSUserInterfaceItemSearching = NSObject;
extern_methods!(
    #[doc = "NSUserInterfaceItemSearching"]
    unsafe impl NSApplication {
        #[method(registerUserInterfaceItemSearchHandler:)]
        pub unsafe fn registerUserInterfaceItemSearchHandler(
            &self,
            handler: &NSUserInterfaceItemSearching,
        );
        #[method(unregisterUserInterfaceItemSearchHandler:)]
        pub unsafe fn unregisterUserInterfaceItemSearchHandler(
            &self,
            handler: &NSUserInterfaceItemSearching,
        );
        #[method(searchString:inUserInterfaceItemString:searchRange:foundRange:)]
        pub unsafe fn searchString_inUserInterfaceItemString_searchRange_foundRange(
            &self,
            searchString: &NSString,
            stringToSearch: &NSString,
            searchRange: NSRange,
            foundRange: *mut NSRange,
        ) -> bool;
    }
);
