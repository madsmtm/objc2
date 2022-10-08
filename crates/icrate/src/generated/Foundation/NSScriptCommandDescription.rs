use super::__exported::NSArray;
use super::__exported::NSDictionary;
use super::__exported::NSScriptCommand;
use super::__exported::NSString;
use crate::Foundation::generated::NSObject::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, msg_send, msg_send_id, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSScriptCommandDescription;
    unsafe impl ClassType for NSScriptCommandDescription {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSScriptCommandDescription {
        pub unsafe fn init(&self) -> Id<Object, Shared> {
            msg_send_id![self, init]
        }
        pub unsafe fn initWithSuiteName_commandName_dictionary(
            &self,
            suiteName: &NSString,
            commandName: &NSString,
            commandDeclaration: Option<&NSDictionary>,
        ) -> Option<Id<Self, Shared>> {
            msg_send_id![
                self,
                initWithSuiteName: suiteName,
                commandName: commandName,
                dictionary: commandDeclaration
            ]
        }
        pub unsafe fn initWithCoder(&self, inCoder: &NSCoder) -> Option<Id<Self, Shared>> {
            msg_send_id![self, initWithCoder: inCoder]
        }
        pub unsafe fn suiteName(&self) -> Id<NSString, Shared> {
            msg_send_id![self, suiteName]
        }
        pub unsafe fn commandName(&self) -> Id<NSString, Shared> {
            msg_send_id![self, commandName]
        }
        pub unsafe fn appleEventClassCode(&self) -> FourCharCode {
            msg_send![self, appleEventClassCode]
        }
        pub unsafe fn appleEventCode(&self) -> FourCharCode {
            msg_send![self, appleEventCode]
        }
        pub unsafe fn commandClassName(&self) -> Id<NSString, Shared> {
            msg_send_id![self, commandClassName]
        }
        pub unsafe fn returnType(&self) -> Option<Id<NSString, Shared>> {
            msg_send_id![self, returnType]
        }
        pub unsafe fn appleEventCodeForReturnType(&self) -> FourCharCode {
            msg_send![self, appleEventCodeForReturnType]
        }
        pub unsafe fn argumentNames(&self) -> Id<NSArray<NSString>, Shared> {
            msg_send_id![self, argumentNames]
        }
        pub unsafe fn typeForArgumentWithName(
            &self,
            argumentName: &NSString,
        ) -> Option<Id<NSString, Shared>> {
            msg_send_id![self, typeForArgumentWithName: argumentName]
        }
        pub unsafe fn appleEventCodeForArgumentWithName(
            &self,
            argumentName: &NSString,
        ) -> FourCharCode {
            msg_send![self, appleEventCodeForArgumentWithName: argumentName]
        }
        pub unsafe fn isOptionalArgumentWithName(&self, argumentName: &NSString) -> bool {
            msg_send![self, isOptionalArgumentWithName: argumentName]
        }
        pub unsafe fn createCommandInstance(&self) -> Id<NSScriptCommand, Shared> {
            msg_send_id![self, createCommandInstance]
        }
        pub unsafe fn createCommandInstanceWithZone(
            &self,
            zone: *mut NSZone,
        ) -> Id<NSScriptCommand, Shared> {
            msg_send_id![self, createCommandInstanceWithZone: zone]
        }
    }
);
