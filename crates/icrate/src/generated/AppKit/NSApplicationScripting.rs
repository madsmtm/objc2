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
