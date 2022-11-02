//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_static!(NSMigrationManagerKey: &'static NSString);

extern_static!(NSMigrationSourceObjectKey: &'static NSString);

extern_static!(NSMigrationDestinationObjectKey: &'static NSString);

extern_static!(NSMigrationEntityMappingKey: &'static NSString);

extern_static!(NSMigrationPropertyMappingKey: &'static NSString);

extern_static!(NSMigrationEntityPolicyKey: &'static NSString);

extern_class!(
    #[derive(Debug)]
    pub struct NSEntityMigrationPolicy;

    unsafe impl ClassType for NSEntityMigrationPolicy {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl NSEntityMigrationPolicy {
        #[method(beginEntityMapping:manager:error:)]
        pub unsafe fn beginEntityMapping_manager_error(
            &self,
            mapping: &NSEntityMapping,
            manager: &NSMigrationManager,
        ) -> Result<(), Id<NSError, Shared>>;

        #[method(createDestinationInstancesForSourceInstance:entityMapping:manager:error:)]
        pub unsafe fn createDestinationInstancesForSourceInstance_entityMapping_manager_error(
            &self,
            sInstance: &NSManagedObject,
            mapping: &NSEntityMapping,
            manager: &NSMigrationManager,
        ) -> Result<(), Id<NSError, Shared>>;

        #[method(endInstanceCreationForEntityMapping:manager:error:)]
        pub unsafe fn endInstanceCreationForEntityMapping_manager_error(
            &self,
            mapping: &NSEntityMapping,
            manager: &NSMigrationManager,
        ) -> Result<(), Id<NSError, Shared>>;

        #[method(createRelationshipsForDestinationInstance:entityMapping:manager:error:)]
        pub unsafe fn createRelationshipsForDestinationInstance_entityMapping_manager_error(
            &self,
            dInstance: &NSManagedObject,
            mapping: &NSEntityMapping,
            manager: &NSMigrationManager,
        ) -> Result<(), Id<NSError, Shared>>;

        #[method(endRelationshipCreationForEntityMapping:manager:error:)]
        pub unsafe fn endRelationshipCreationForEntityMapping_manager_error(
            &self,
            mapping: &NSEntityMapping,
            manager: &NSMigrationManager,
        ) -> Result<(), Id<NSError, Shared>>;

        #[method(performCustomValidationForEntityMapping:manager:error:)]
        pub unsafe fn performCustomValidationForEntityMapping_manager_error(
            &self,
            mapping: &NSEntityMapping,
            manager: &NSMigrationManager,
        ) -> Result<(), Id<NSError, Shared>>;

        #[method(endEntityMapping:manager:error:)]
        pub unsafe fn endEntityMapping_manager_error(
            &self,
            mapping: &NSEntityMapping,
            manager: &NSMigrationManager,
        ) -> Result<(), Id<NSError, Shared>>;
    }
);
