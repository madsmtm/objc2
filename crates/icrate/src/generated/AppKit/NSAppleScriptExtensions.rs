#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_methods!(
    #[doc = "NSExtensions"]
    unsafe impl NSAppleScript {
        #[method_id(richTextSource)]
        pub unsafe fn richTextSource(&self) -> Option<Id<NSAttributedString, Shared>>;
    }
);
