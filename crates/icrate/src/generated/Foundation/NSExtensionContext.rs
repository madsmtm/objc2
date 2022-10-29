#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSExtensionContext;
    unsafe impl ClassType for NSExtensionContext {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSExtensionContext {
        #[method_id(inputItems)]
        pub unsafe fn inputItems(&self) -> Id<NSArray, Shared>;
        #[method(completeRequestReturningItems:completionHandler:)]
        pub unsafe fn completeRequestReturningItems_completionHandler(
            &self,
            items: Option<&NSArray>,
            completionHandler: TodoBlock,
        );
        #[method(cancelRequestWithError:)]
        pub unsafe fn cancelRequestWithError(&self, error: &NSError);
        #[method(openURL:completionHandler:)]
        pub unsafe fn openURL_completionHandler(&self, URL: &NSURL, completionHandler: TodoBlock);
    }
);
