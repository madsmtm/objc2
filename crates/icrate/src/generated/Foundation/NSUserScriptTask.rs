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
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSUserScriptTask;
    unsafe impl ClassType for NSUserScriptTask {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSUserScriptTask {
        #[method_id(initWithURL:error:)]
        pub unsafe fn initWithURL_error(
            &self,
            url: &NSURL,
        ) -> Result<Id<Self, Shared>, Id<NSError, Shared>>;
        #[method_id(scriptURL)]
        pub unsafe fn scriptURL(&self) -> Id<NSURL, Shared>;
        #[method(executeWithCompletionHandler:)]
        pub unsafe fn executeWithCompletionHandler(
            &self,
            handler: NSUserScriptTaskCompletionHandler,
        );
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
        #[method_id(standardInput)]
        pub unsafe fn standardInput(&self) -> Option<Id<NSFileHandle, Shared>>;
        #[method(setStandardInput:)]
        pub unsafe fn setStandardInput(&self, standardInput: Option<&NSFileHandle>);
        #[method_id(standardOutput)]
        pub unsafe fn standardOutput(&self) -> Option<Id<NSFileHandle, Shared>>;
        #[method(setStandardOutput:)]
        pub unsafe fn setStandardOutput(&self, standardOutput: Option<&NSFileHandle>);
        #[method_id(standardError)]
        pub unsafe fn standardError(&self) -> Option<Id<NSFileHandle, Shared>>;
        #[method(setStandardError:)]
        pub unsafe fn setStandardError(&self, standardError: Option<&NSFileHandle>);
        #[method(executeWithArguments:completionHandler:)]
        pub unsafe fn executeWithArguments_completionHandler(
            &self,
            arguments: Option<&NSArray<NSString>>,
            handler: NSUserUnixTaskCompletionHandler,
        );
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
        #[method(executeWithAppleEvent:completionHandler:)]
        pub unsafe fn executeWithAppleEvent_completionHandler(
            &self,
            event: Option<&NSAppleEventDescriptor>,
            handler: NSUserAppleScriptTaskCompletionHandler,
        );
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
        #[method_id(variables)]
        pub unsafe fn variables(&self) -> Option<Id<NSDictionary<NSString, Object>, Shared>>;
        #[method(setVariables:)]
        pub unsafe fn setVariables(&self, variables: Option<&NSDictionary<NSString, Object>>);
        #[method(executeWithInput:completionHandler:)]
        pub unsafe fn executeWithInput_completionHandler(
            &self,
            input: Option<&NSSecureCoding>,
            handler: NSUserAutomatorTaskCompletionHandler,
        );
    }
);
