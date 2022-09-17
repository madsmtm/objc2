#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, msg_send, msg_send_id, ClassType};
extern_class!(
    #[derive(Debug)]
    struct NSScriptClassDescription;
    unsafe impl ClassType for NSScriptClassDescription {
        type Super = NSClassDescription;
    }
);
impl NSScriptClassDescription {
    pub unsafe fn classDescriptionForClass(
        aClass: &Class,
    ) -> Option<Id<NSScriptClassDescription, Shared>> {
        msg_send_id![Self::class(), classDescriptionForClass: aClass]
    }
    pub unsafe fn initWithSuiteName_className_dictionary(
        &self,
        suiteName: &NSString,
        className: &NSString,
        classDeclaration: Option<&NSDictionary>,
    ) -> Option<Id<Self, Shared>> {
        msg_send_id![
            self,
            initWithSuiteName: suiteName,
            className: className,
            dictionary: classDeclaration
        ]
    }
    pub unsafe fn matchesAppleEventCode(&self, appleEventCode: FourCharCode) -> bool {
        msg_send![self, matchesAppleEventCode: appleEventCode]
    }
    pub unsafe fn supportsCommand(&self, commandDescription: &NSScriptCommandDescription) -> bool {
        msg_send![self, supportsCommand: commandDescription]
    }
    pub unsafe fn selectorForCommand(
        &self,
        commandDescription: &NSScriptCommandDescription,
    ) -> Option<Sel> {
        msg_send![self, selectorForCommand: commandDescription]
    }
    pub unsafe fn typeForKey(&self, key: &NSString) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, typeForKey: key]
    }
    pub unsafe fn classDescriptionForKey(
        &self,
        key: &NSString,
    ) -> Option<Id<NSScriptClassDescription, Shared>> {
        msg_send_id![self, classDescriptionForKey: key]
    }
    pub unsafe fn appleEventCodeForKey(&self, key: &NSString) -> FourCharCode {
        msg_send![self, appleEventCodeForKey: key]
    }
    pub unsafe fn keyWithAppleEventCode(
        &self,
        appleEventCode: FourCharCode,
    ) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, keyWithAppleEventCode: appleEventCode]
    }
    pub unsafe fn isLocationRequiredToCreateForKey(
        &self,
        toManyRelationshipKey: &NSString,
    ) -> bool {
        msg_send![
            self,
            isLocationRequiredToCreateForKey: toManyRelationshipKey
        ]
    }
    pub unsafe fn hasPropertyForKey(&self, key: &NSString) -> bool {
        msg_send![self, hasPropertyForKey: key]
    }
    pub unsafe fn hasOrderedToManyRelationshipForKey(&self, key: &NSString) -> bool {
        msg_send![self, hasOrderedToManyRelationshipForKey: key]
    }
    pub unsafe fn hasReadablePropertyForKey(&self, key: &NSString) -> bool {
        msg_send![self, hasReadablePropertyForKey: key]
    }
    pub unsafe fn hasWritablePropertyForKey(&self, key: &NSString) -> bool {
        msg_send![self, hasWritablePropertyForKey: key]
    }
    pub unsafe fn suiteName(&self) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, suiteName]
    }
    pub unsafe fn className(&self) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, className]
    }
    pub unsafe fn implementationClassName(&self) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, implementationClassName]
    }
    pub unsafe fn superclassDescription(&self) -> Option<Id<NSScriptClassDescription, Shared>> {
        msg_send_id![self, superclassDescription]
    }
    pub unsafe fn appleEventCode(&self) -> FourCharCode {
        msg_send![self, appleEventCode]
    }
    pub unsafe fn defaultSubcontainerAttributeKey(&self) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, defaultSubcontainerAttributeKey]
    }
}
#[doc = "NSDeprecated"]
impl NSScriptClassDescription {
    pub unsafe fn isReadOnlyKey(&self, key: &NSString) -> bool {
        msg_send![self, isReadOnlyKey: key]
    }
}
#[doc = "NSScriptClassDescription"]
impl NSObject {
    pub unsafe fn classCode(&self) -> FourCharCode {
        msg_send![self, classCode]
    }
    pub unsafe fn className(&self) -> Id<NSString, Shared> {
        msg_send_id![self, className]
    }
}
