use super::__exported::NSScriptCommandDescription;
use crate::Foundation::generated::NSClassDescription::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSScriptClassDescription;
    unsafe impl ClassType for NSScriptClassDescription {
        type Super = NSClassDescription;
    }
);
extern_methods!(
    unsafe impl NSScriptClassDescription {
        #[method_id(classDescriptionForClass:)]
        pub unsafe fn classDescriptionForClass(
            aClass: &Class,
        ) -> Option<Id<NSScriptClassDescription, Shared>>;
        #[method_id(initWithSuiteName:className:dictionary:)]
        pub unsafe fn initWithSuiteName_className_dictionary(
            &self,
            suiteName: &NSString,
            className: &NSString,
            classDeclaration: Option<&NSDictionary>,
        ) -> Option<Id<Self, Shared>>;
        #[method_id(suiteName)]
        pub unsafe fn suiteName(&self) -> Option<Id<NSString, Shared>>;
        #[method_id(className)]
        pub unsafe fn className(&self) -> Option<Id<NSString, Shared>>;
        #[method_id(implementationClassName)]
        pub unsafe fn implementationClassName(&self) -> Option<Id<NSString, Shared>>;
        #[method_id(superclassDescription)]
        pub unsafe fn superclassDescription(&self) -> Option<Id<NSScriptClassDescription, Shared>>;
        #[method(appleEventCode)]
        pub unsafe fn appleEventCode(&self) -> FourCharCode;
        #[method(matchesAppleEventCode:)]
        pub unsafe fn matchesAppleEventCode(&self, appleEventCode: FourCharCode) -> bool;
        #[method(supportsCommand:)]
        pub unsafe fn supportsCommand(
            &self,
            commandDescription: &NSScriptCommandDescription,
        ) -> bool;
        #[method(selectorForCommand:)]
        pub unsafe fn selectorForCommand(
            &self,
            commandDescription: &NSScriptCommandDescription,
        ) -> Option<Sel>;
        #[method_id(typeForKey:)]
        pub unsafe fn typeForKey(&self, key: &NSString) -> Option<Id<NSString, Shared>>;
        #[method_id(classDescriptionForKey:)]
        pub unsafe fn classDescriptionForKey(
            &self,
            key: &NSString,
        ) -> Option<Id<NSScriptClassDescription, Shared>>;
        #[method(appleEventCodeForKey:)]
        pub unsafe fn appleEventCodeForKey(&self, key: &NSString) -> FourCharCode;
        #[method_id(keyWithAppleEventCode:)]
        pub unsafe fn keyWithAppleEventCode(
            &self,
            appleEventCode: FourCharCode,
        ) -> Option<Id<NSString, Shared>>;
        #[method_id(defaultSubcontainerAttributeKey)]
        pub unsafe fn defaultSubcontainerAttributeKey(&self) -> Option<Id<NSString, Shared>>;
        #[method(isLocationRequiredToCreateForKey:)]
        pub unsafe fn isLocationRequiredToCreateForKey(
            &self,
            toManyRelationshipKey: &NSString,
        ) -> bool;
        #[method(hasPropertyForKey:)]
        pub unsafe fn hasPropertyForKey(&self, key: &NSString) -> bool;
        #[method(hasOrderedToManyRelationshipForKey:)]
        pub unsafe fn hasOrderedToManyRelationshipForKey(&self, key: &NSString) -> bool;
        #[method(hasReadablePropertyForKey:)]
        pub unsafe fn hasReadablePropertyForKey(&self, key: &NSString) -> bool;
        #[method(hasWritablePropertyForKey:)]
        pub unsafe fn hasWritablePropertyForKey(&self, key: &NSString) -> bool;
    }
);
extern_methods!(
    #[doc = "NSDeprecated"]
    unsafe impl NSScriptClassDescription {
        #[method(isReadOnlyKey:)]
        pub unsafe fn isReadOnlyKey(&self, key: &NSString) -> bool;
    }
);
extern_methods!(
    #[doc = "NSScriptClassDescription"]
    unsafe impl NSObject {
        #[method(classCode)]
        pub unsafe fn classCode(&self) -> FourCharCode;
        #[method_id(className)]
        pub unsafe fn className(&self) -> Id<NSString, Shared>;
    }
);
