use super::__exported::NSArray;
use super::__exported::NSDictionary;
use super::__exported::NSString;
use crate::Foundation::generated::NSException::*;
use crate::Foundation::generated::NSNotification::*;
use crate::Foundation::generated::NSObject::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSClassDescription;
    unsafe impl ClassType for NSClassDescription {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSClassDescription {
        # [method (registerClassDescription : forClass :)]
        pub unsafe fn registerClassDescription_forClass(
            description: &NSClassDescription,
            aClass: &Class,
        );
        #[method(invalidateClassDescriptionCache)]
        pub unsafe fn invalidateClassDescriptionCache();
        # [method_id (classDescriptionForClass :)]
        pub unsafe fn classDescriptionForClass(
            aClass: &Class,
        ) -> Option<Id<NSClassDescription, Shared>>;
        #[method_id(attributeKeys)]
        pub unsafe fn attributeKeys(&self) -> Id<NSArray<NSString>, Shared>;
        #[method_id(toOneRelationshipKeys)]
        pub unsafe fn toOneRelationshipKeys(&self) -> Id<NSArray<NSString>, Shared>;
        #[method_id(toManyRelationshipKeys)]
        pub unsafe fn toManyRelationshipKeys(&self) -> Id<NSArray<NSString>, Shared>;
        # [method_id (inverseForRelationshipKey :)]
        pub unsafe fn inverseForRelationshipKey(
            &self,
            relationshipKey: &NSString,
        ) -> Option<Id<NSString, Shared>>;
    }
);
extern_methods!(
    #[doc = "NSClassDescriptionPrimitives"]
    unsafe impl NSObject {
        #[method_id(classDescription)]
        pub unsafe fn classDescription(&self) -> Id<NSClassDescription, Shared>;
        #[method_id(attributeKeys)]
        pub unsafe fn attributeKeys(&self) -> Id<NSArray<NSString>, Shared>;
        #[method_id(toOneRelationshipKeys)]
        pub unsafe fn toOneRelationshipKeys(&self) -> Id<NSArray<NSString>, Shared>;
        #[method_id(toManyRelationshipKeys)]
        pub unsafe fn toManyRelationshipKeys(&self) -> Id<NSArray<NSString>, Shared>;
        # [method_id (inverseForRelationshipKey :)]
        pub unsafe fn inverseForRelationshipKey(
            &self,
            relationshipKey: &NSString,
        ) -> Option<Id<NSString, Shared>>;
    }
);
