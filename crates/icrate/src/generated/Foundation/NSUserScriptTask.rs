use super::__exported::NSAppleEventDescriptor;
use super::__exported::NSArray;
use super::__exported::NSDictionary;
use super::__exported::NSError;
use super::__exported::NSFileHandle;
use super::__exported::NSString;
use super::__exported::NSXPCConnection;
use super::__exported::NSURL;
use crate::Foundation::generated::NSObject::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, msg_send, msg_send_id, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSUserScriptTask;
    unsafe impl ClassType for NSUserScriptTask {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSUserScriptTask {
        pub unsafe fn initWithURL_error(
            &self,
            url: &NSURL,
        ) -> Result<Id<Self, Shared>, Id<NSError, Shared>> {
            msg_send_id![self, initWithURL: url, error: _]
        }
        pub unsafe fn scriptURL(&self) -> Id<NSURL, Shared> {
            msg_send_id![self, scriptURL]
        }
        pub unsafe fn executeWithCompletionHandler(
            &self,
            handler: NSUserScriptTaskCompletionHandler,
        ) {
            msg_send![self, executeWithCompletionHandler: handler]
        }
    }
);
extern_class!(
    #[derive(Debug)]
    pub struct NSUserUnixTask;
    unsafe impl ClassType for NSUserUnixTask {
        type Super = NSUserScriptTask;
    }
);
extern_methods!(
    unsafe impl NSUserUnixTask {
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
        pub unsafe fn executeWithArguments_completionHandler(
            &self,
            arguments: Option<&NSArray<NSString>>,
            handler: NSUserUnixTaskCompletionHandler,
        ) {
            msg_send![
                self,
                executeWithArguments: arguments,
                completionHandler: handler
            ]
        }
    }
);
extern_class!(
    #[derive(Debug)]
    pub struct NSUserAppleScriptTask;
    unsafe impl ClassType for NSUserAppleScriptTask {
        type Super = NSUserScriptTask;
    }
);
extern_methods!(
    unsafe impl NSUserAppleScriptTask {
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
);
extern_class!(
    #[derive(Debug)]
    pub struct NSUserAutomatorTask;
    unsafe impl ClassType for NSUserAutomatorTask {
        type Super = NSUserScriptTask;
    }
);
extern_methods!(
    unsafe impl NSUserAutomatorTask {
        pub unsafe fn variables(&self) -> Option<Id<NSDictionary<NSString, Object>, Shared>> {
            msg_send_id![self, variables]
        }
        pub unsafe fn setVariables(&self, variables: Option<&NSDictionary<NSString, Object>>) {
            msg_send![self, setVariables: variables]
        }
        pub unsafe fn executeWithInput_completionHandler(
            &self,
            input: Option<&NSSecureCoding>,
            handler: NSUserAutomatorTaskCompletionHandler,
        ) {
            msg_send![self, executeWithInput: input, completionHandler: handler]
        }
    }
);
