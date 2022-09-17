extern_class!(
    #[derive(Debug)]
    struct NSExtensionContext;
    unsafe impl ClassType for NSExtensionContext {
        type Super = NSObject;
    }
);
impl NSExtensionContext {
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
    pub unsafe fn inputItems(&self) -> Id<NSArray, Shared> {
        msg_send_id![self, inputItems]
    }
}
