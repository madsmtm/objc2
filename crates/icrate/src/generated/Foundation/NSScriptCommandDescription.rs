use super::__exported::NSArray;
use super::__exported::NSDictionary;
use super::__exported::NSScriptCommand;
use super::__exported::NSString;
use crate::Foundation::generated::NSObject::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSScriptCommandDescription;
    unsafe impl ClassType for NSScriptCommandDescription {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSScriptCommandDescription {
        #[method_id(init)]
        pub unsafe fn init(&self) -> Id<Object, Shared>;
        # [method_id (initWithSuiteName : commandName : dictionary :)]
        pub unsafe fn initWithSuiteName_commandName_dictionary(
            &self,
            suiteName: &NSString,
            commandName: &NSString,
            commandDeclaration: Option<&NSDictionary>,
        ) -> Option<Id<Self, Shared>>;
        # [method_id (initWithCoder :)]
        pub unsafe fn initWithCoder(&self, inCoder: &NSCoder) -> Option<Id<Self, Shared>>;
        #[method_id(suiteName)]
        pub unsafe fn suiteName(&self) -> Id<NSString, Shared>;
        #[method_id(commandName)]
        pub unsafe fn commandName(&self) -> Id<NSString, Shared>;
        #[method(appleEventClassCode)]
        pub unsafe fn appleEventClassCode(&self) -> FourCharCode;
        #[method(appleEventCode)]
        pub unsafe fn appleEventCode(&self) -> FourCharCode;
        #[method_id(commandClassName)]
        pub unsafe fn commandClassName(&self) -> Id<NSString, Shared>;
        #[method_id(returnType)]
        pub unsafe fn returnType(&self) -> Option<Id<NSString, Shared>>;
        #[method(appleEventCodeForReturnType)]
        pub unsafe fn appleEventCodeForReturnType(&self) -> FourCharCode;
        #[method_id(argumentNames)]
        pub unsafe fn argumentNames(&self) -> Id<NSArray<NSString>, Shared>;
        # [method_id (typeForArgumentWithName :)]
        pub unsafe fn typeForArgumentWithName(
            &self,
            argumentName: &NSString,
        ) -> Option<Id<NSString, Shared>>;
        # [method (appleEventCodeForArgumentWithName :)]
        pub unsafe fn appleEventCodeForArgumentWithName(
            &self,
            argumentName: &NSString,
        ) -> FourCharCode;
        # [method (isOptionalArgumentWithName :)]
        pub unsafe fn isOptionalArgumentWithName(&self, argumentName: &NSString) -> bool;
        #[method_id(createCommandInstance)]
        pub unsafe fn createCommandInstance(&self) -> Id<NSScriptCommand, Shared>;
        # [method_id (createCommandInstanceWithZone :)]
        pub unsafe fn createCommandInstanceWithZone(
            &self,
            zone: *mut NSZone,
        ) -> Id<NSScriptCommand, Shared>;
    }
);
