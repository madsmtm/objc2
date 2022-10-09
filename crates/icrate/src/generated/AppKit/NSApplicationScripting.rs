use super::__exported::NSDocument;
use super::__exported::NSWindow;
use crate::AppKit::generated::AppKitDefines::*;
use crate::AppKit::generated::NSApplication::*;
use crate::Foundation::generated::NSArray::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_methods!(
    #[doc = "NSScripting"]
    unsafe impl NSApplication {
        #[method_id(orderedDocuments)]
        pub unsafe fn orderedDocuments(&self) -> Id<NSArray<NSDocument>, Shared>;
        #[method_id(orderedWindows)]
        pub unsafe fn orderedWindows(&self) -> Id<NSArray<NSWindow>, Shared>;
    }
);
extern_methods!(
    #[doc = "NSApplicationScriptingDelegation"]
    unsafe impl NSObject {
        #[method(application:delegateHandlesKey:)]
        pub unsafe fn application_delegateHandlesKey(
            &self,
            sender: &NSApplication,
            key: &NSString,
        ) -> bool;
    }
);
