use super::__exported::NSAppleEventDescriptor;
use super::__exported::NSDictionary;
use super::__exported::NSMutableDictionary;
use super::__exported::NSScriptCommandDescription;
use super::__exported::NSScriptObjectSpecifier;
use super::__exported::NSString;
use crate::Foundation::generated::NSObject::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSScriptCommand;
    unsafe impl ClassType for NSScriptCommand {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSScriptCommand {
        #[method_id(initWithCommandDescription:)]
        pub unsafe fn initWithCommandDescription(
            &self,
            commandDef: &NSScriptCommandDescription,
        ) -> Id<Self, Shared>;
        #[method_id(initWithCoder:)]
        pub unsafe fn initWithCoder(&self, inCoder: &NSCoder) -> Option<Id<Self, Shared>>;
        #[method_id(commandDescription)]
        pub unsafe fn commandDescription(&self) -> Id<NSScriptCommandDescription, Shared>;
        #[method_id(directParameter)]
        pub unsafe fn directParameter(&self) -> Option<Id<Object, Shared>>;
        #[method(setDirectParameter:)]
        pub unsafe fn setDirectParameter(&self, directParameter: Option<&Object>);
        #[method_id(receiversSpecifier)]
        pub unsafe fn receiversSpecifier(&self) -> Option<Id<NSScriptObjectSpecifier, Shared>>;
        #[method(setReceiversSpecifier:)]
        pub unsafe fn setReceiversSpecifier(
            &self,
            receiversSpecifier: Option<&NSScriptObjectSpecifier>,
        );
        #[method_id(evaluatedReceivers)]
        pub unsafe fn evaluatedReceivers(&self) -> Option<Id<Object, Shared>>;
        #[method_id(arguments)]
        pub unsafe fn arguments(&self) -> Option<Id<NSDictionary<NSString, Object>, Shared>>;
        #[method(setArguments:)]
        pub unsafe fn setArguments(&self, arguments: Option<&NSDictionary<NSString, Object>>);
        #[method_id(evaluatedArguments)]
        pub unsafe fn evaluatedArguments(
            &self,
        ) -> Option<Id<NSDictionary<NSString, Object>, Shared>>;
        #[method(isWellFormed)]
        pub unsafe fn isWellFormed(&self) -> bool;
        #[method_id(performDefaultImplementation)]
        pub unsafe fn performDefaultImplementation(&self) -> Option<Id<Object, Shared>>;
        #[method_id(executeCommand)]
        pub unsafe fn executeCommand(&self) -> Option<Id<Object, Shared>>;
        #[method(scriptErrorNumber)]
        pub unsafe fn scriptErrorNumber(&self) -> NSInteger;
        #[method(setScriptErrorNumber:)]
        pub unsafe fn setScriptErrorNumber(&self, scriptErrorNumber: NSInteger);
        #[method_id(scriptErrorOffendingObjectDescriptor)]
        pub unsafe fn scriptErrorOffendingObjectDescriptor(
            &self,
        ) -> Option<Id<NSAppleEventDescriptor, Shared>>;
        #[method(setScriptErrorOffendingObjectDescriptor:)]
        pub unsafe fn setScriptErrorOffendingObjectDescriptor(
            &self,
            scriptErrorOffendingObjectDescriptor: Option<&NSAppleEventDescriptor>,
        );
        #[method_id(scriptErrorExpectedTypeDescriptor)]
        pub unsafe fn scriptErrorExpectedTypeDescriptor(
            &self,
        ) -> Option<Id<NSAppleEventDescriptor, Shared>>;
        #[method(setScriptErrorExpectedTypeDescriptor:)]
        pub unsafe fn setScriptErrorExpectedTypeDescriptor(
            &self,
            scriptErrorExpectedTypeDescriptor: Option<&NSAppleEventDescriptor>,
        );
        #[method_id(scriptErrorString)]
        pub unsafe fn scriptErrorString(&self) -> Option<Id<NSString, Shared>>;
        #[method(setScriptErrorString:)]
        pub unsafe fn setScriptErrorString(&self, scriptErrorString: Option<&NSString>);
        #[method_id(currentCommand)]
        pub unsafe fn currentCommand() -> Option<Id<NSScriptCommand, Shared>>;
        #[method_id(appleEvent)]
        pub unsafe fn appleEvent(&self) -> Option<Id<NSAppleEventDescriptor, Shared>>;
        #[method(suspendExecution)]
        pub unsafe fn suspendExecution(&self);
        #[method(resumeExecutionWithResult:)]
        pub unsafe fn resumeExecutionWithResult(&self, result: Option<&Object>);
    }
);
