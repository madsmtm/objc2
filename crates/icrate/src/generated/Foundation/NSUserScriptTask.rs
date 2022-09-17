extern_class!(
    #[derive(Debug)]
    struct NSUserScriptTask;
    unsafe impl ClassType for NSUserScriptTask {
        type Super = NSObject;
    }
);
impl NSUserScriptTask {
    pub unsafe fn initWithURL_error(
        &self,
        url: &NSURL,
        error: *mut Option<&NSError>,
    ) -> Option<Id<Self, Shared>> {
        msg_send_id![self, initWithURL: url, error: error]
    }
    pub unsafe fn executeWithCompletionHandler(&self, handler: NSUserScriptTaskCompletionHandler) {
        msg_send![self, executeWithCompletionHandler: handler]
    }
    pub unsafe fn scriptURL(&self) -> Id<NSURL, Shared> {
        msg_send_id![self, scriptURL]
    }
}
extern_class!(
    #[derive(Debug)]
    struct NSUserUnixTask;
    unsafe impl ClassType for NSUserUnixTask {
        type Super = NSUserScriptTask;
    }
);
impl NSUserUnixTask {
    pub unsafe fn executeWithArguments_completionHandler(
        &self,
        arguments: TodoGenerics,
        handler: NSUserUnixTaskCompletionHandler,
    ) {
        msg_send![
            self,
            executeWithArguments: arguments,
            completionHandler: handler
        ]
    }
    pub unsafe fn standardInput(&self) -> Option<Id<NSFileHandle, Shared>> {
        msg_send_id![self, standardInput]
    }
    pub unsafe fn setStandardInput(&self, standardInput: Option<&NSFileHandle>) {
        msg_send![self, setStandardInput: standardInput]
    }
    pub unsafe fn standardOutput(&self) -> Option<Id<NSFileHandle, Shared>> {
        msg_send_id![self, standardOutput]
    }
    pub unsafe fn setStandardOutput(&self, standardOutput: Option<&NSFileHandle>) {
        msg_send![self, setStandardOutput: standardOutput]
    }
    pub unsafe fn standardError(&self) -> Option<Id<NSFileHandle, Shared>> {
        msg_send_id![self, standardError]
    }
    pub unsafe fn setStandardError(&self, standardError: Option<&NSFileHandle>) {
        msg_send![self, setStandardError: standardError]
    }
}
extern_class!(
    #[derive(Debug)]
    struct NSUserAppleScriptTask;
    unsafe impl ClassType for NSUserAppleScriptTask {
        type Super = NSUserScriptTask;
    }
);
impl NSUserAppleScriptTask {
    pub unsafe fn executeWithAppleEvent_completionHandler(
        &self,
        event: Option<&NSAppleEventDescriptor>,
        handler: NSUserAppleScriptTaskCompletionHandler,
    ) {
        msg_send![
            self,
            executeWithAppleEvent: event,
            completionHandler: handler
        ]
    }
}
extern_class!(
    #[derive(Debug)]
    struct NSUserAutomatorTask;
    unsafe impl ClassType for NSUserAutomatorTask {
        type Super = NSUserScriptTask;
    }
);
impl NSUserAutomatorTask {
    pub unsafe fn executeWithInput_completionHandler(
        &self,
        input: TodoGenerics,
        handler: NSUserAutomatorTaskCompletionHandler,
    ) {
        msg_send![self, executeWithInput: input, completionHandler: handler]
    }
    pub unsafe fn variables(&self) -> TodoGenerics {
        msg_send![self, variables]
    }
    pub unsafe fn setVariables(&self, variables: TodoGenerics) {
        msg_send![self, setVariables: variables]
    }
}
