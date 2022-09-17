use super::__exported::NSArray;
use super::__exported::NSDictionary;
use super::__exported::NSString;
use crate::Foundation::generated::NSException::*;
use crate::Foundation::generated::NSNotification::*;
use crate::Foundation::generated::NSObject::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, msg_send, msg_send_id, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSClassDescription;
    unsafe impl ClassType for NSClassDescription {
        type Super = NSObject;
    }
);
impl NSClassDescription {
    pub unsafe fn registerClassDescription_forClass(
        description: &NSClassDescription,
        aClass: &Class,
    ) {
        msg_send![
            Self::class(),
            registerClassDescription: description,
            forClass: aClass
        ]
    }
    pub unsafe fn invalidateClassDescriptionCache() {
        msg_send![Self::class(), invalidateClassDescriptionCache]
    }
    pub unsafe fn classDescriptionForClass(
        aClass: &Class,
    ) -> Option<Id<NSClassDescription, Shared>> {
        msg_send_id![Self::class(), classDescriptionForClass: aClass]
    }
    pub unsafe fn inverseForRelationshipKey(
        &self,
        relationshipKey: &NSString,
    ) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, inverseForRelationshipKey: relationshipKey]
    }
    pub unsafe fn attributeKeys(&self) -> TodoGenerics {
        msg_send![self, attributeKeys]
    }
    pub unsafe fn toOneRelationshipKeys(&self) -> TodoGenerics {
        msg_send![self, toOneRelationshipKeys]
    }
    pub unsafe fn toManyRelationshipKeys(&self) -> TodoGenerics {
        msg_send![self, toManyRelationshipKeys]
    }
}
#[doc = "NSClassDescriptionPrimitives"]
impl NSObject {
    pub unsafe fn inverseForRelationshipKey(
        &self,
        relationshipKey: &NSString,
    ) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, inverseForRelationshipKey: relationshipKey]
    }
    pub unsafe fn classDescription(&self) -> Id<NSClassDescription, Shared> {
        msg_send_id![self, classDescription]
    }
    pub unsafe fn attributeKeys(&self) -> TodoGenerics {
        msg_send![self, attributeKeys]
    }
    pub unsafe fn toOneRelationshipKeys(&self) -> TodoGenerics {
        msg_send![self, toOneRelationshipKeys]
    }
    pub unsafe fn toManyRelationshipKeys(&self) -> TodoGenerics {
        msg_send![self, toManyRelationshipKeys]
    }
}
