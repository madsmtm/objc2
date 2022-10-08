use crate::Foundation::generated::Foundation::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, msg_send, msg_send_id, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSExtensionContext;
    unsafe impl ClassType for NSExtensionContext {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSExtensionContext {
        pub unsafe fn inputItems(&self) -> Id<NSArray, Shared> {
            msg_send_id![self, inputItems]
        }
        pub unsafe fn completeRequestReturningItems_completionHandler(
            &self,
            items: Option<&NSArray>,
            completionHandler: TodoBlock,
        ) {
            msg_send![
                self,
                completeRequestReturningItems: items,
                completionHandler: completionHandler
            ]
        }
        pub unsafe fn cancelRequestWithError(&self, error: &NSError) {
            msg_send![self, cancelRequestWithError: error]
        }
        pub unsafe fn openURL_completionHandler(&self, URL: &NSURL, completionHandler: TodoBlock) {
            msg_send![self, openURL: URL, completionHandler: completionHandler]
        }
    }
);
