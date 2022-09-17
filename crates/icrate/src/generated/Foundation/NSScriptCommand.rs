#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, msg_send, msg_send_id, ClassType};
extern_class!(
    #[derive(Debug)]
    struct NSScriptCommand;
    unsafe impl ClassType for NSScriptCommand {
        type Super = NSObject;
    }
);
impl NSScriptCommand {
    pub unsafe fn initWithCommandDescription(
        &self,
        commandDef: &NSScriptCommandDescription,
    ) -> Id<Self, Shared> {
        msg_send_id![self, initWithCommandDescription: commandDef]
    }
    pub unsafe fn initWithCoder(&self, inCoder: &NSCoder) -> Option<Id<Self, Shared>> {
        msg_send_id![self, initWithCoder: inCoder]
    }
    pub unsafe fn performDefaultImplementation(&self) -> Option<Id<Object, Shared>> {
        msg_send_id![self, performDefaultImplementation]
    }
    pub unsafe fn executeCommand(&self) -> Option<Id<Object, Shared>> {
        msg_send_id![self, executeCommand]
    }
    pub unsafe fn currentCommand() -> Option<Id<NSScriptCommand, Shared>> {
        msg_send_id![Self::class(), currentCommand]
    }
    pub unsafe fn suspendExecution(&self) {
        msg_send![self, suspendExecution]
    }
    pub unsafe fn resumeExecutionWithResult(&self, result: Option<&Object>) {
        msg_send![self, resumeExecutionWithResult: result]
    }
    pub unsafe fn commandDescription(&self) -> Id<NSScriptCommandDescription, Shared> {
        msg_send_id![self, commandDescription]
    }
    pub unsafe fn directParameter(&self) -> Option<Id<Object, Shared>> {
        msg_send_id![self, directParameter]
    }
    pub unsafe fn setDirectParameter(&self, directParameter: Option<&Object>) {
        msg_send![self, setDirectParameter: directParameter]
    }
    pub unsafe fn receiversSpecifier(&self) -> Option<Id<NSScriptObjectSpecifier, Shared>> {
        msg_send_id![self, receiversSpecifier]
    }
    pub unsafe fn setReceiversSpecifier(
        &self,
        receiversSpecifier: Option<&NSScriptObjectSpecifier>,
    ) {
        msg_send![self, setReceiversSpecifier: receiversSpecifier]
    }
    pub unsafe fn evaluatedReceivers(&self) -> Option<Id<Object, Shared>> {
        msg_send_id![self, evaluatedReceivers]
    }
    pub unsafe fn arguments(&self) -> TodoGenerics {
        msg_send![self, arguments]
    }
    pub unsafe fn setArguments(&self, arguments: TodoGenerics) {
        msg_send![self, setArguments: arguments]
    }
    pub unsafe fn evaluatedArguments(&self) -> TodoGenerics {
        msg_send![self, evaluatedArguments]
    }
    pub unsafe fn isWellFormed(&self) -> bool {
        msg_send![self, isWellFormed]
    }
    pub unsafe fn scriptErrorNumber(&self) -> NSInteger {
        msg_send![self, scriptErrorNumber]
    }
    pub unsafe fn setScriptErrorNumber(&self, scriptErrorNumber: NSInteger) {
        msg_send![self, setScriptErrorNumber: scriptErrorNumber]
    }
    pub unsafe fn scriptErrorOffendingObjectDescriptor(
        &self,
    ) -> Option<Id<NSAppleEventDescriptor, Shared>> {
        msg_send_id![self, scriptErrorOffendingObjectDescriptor]
    }
    pub unsafe fn setScriptErrorOffendingObjectDescriptor(
        &self,
        scriptErrorOffendingObjectDescriptor: Option<&NSAppleEventDescriptor>,
    ) {
        msg_send![
            self,
            setScriptErrorOffendingObjectDescriptor: scriptErrorOffendingObjectDescriptor
        ]
    }
    pub unsafe fn scriptErrorExpectedTypeDescriptor(
        &self,
    ) -> Option<Id<NSAppleEventDescriptor, Shared>> {
        msg_send_id![self, scriptErrorExpectedTypeDescriptor]
    }
    pub unsafe fn setScriptErrorExpectedTypeDescriptor(
        &self,
        scriptErrorExpectedTypeDescriptor: Option<&NSAppleEventDescriptor>,
    ) {
        msg_send![
            self,
            setScriptErrorExpectedTypeDescriptor: scriptErrorExpectedTypeDescriptor
        ]
    }
    pub unsafe fn scriptErrorString(&self) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, scriptErrorString]
    }
    pub unsafe fn setScriptErrorString(&self, scriptErrorString: Option<&NSString>) {
        msg_send![self, setScriptErrorString: scriptErrorString]
    }
    pub unsafe fn appleEvent(&self) -> Option<Id<NSAppleEventDescriptor, Shared>> {
        msg_send_id![self, appleEvent]
    }
}
