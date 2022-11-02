//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreData::*;
use crate::Foundation::*;

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSPersistentCloudKitContainerSchemaInitializationOptions {
        NSPersistentCloudKitContainerSchemaInitializationOptionsNone = 0,
        NSPersistentCloudKitContainerSchemaInitializationOptionsDryRun = 1 << 1,
        NSPersistentCloudKitContainerSchemaInitializationOptionsPrintSchema = 1 << 2,
    }
);

extern_class!(
    #[derive(Debug)]
    pub struct NSPersistentCloudKitContainer;

    unsafe impl ClassType for NSPersistentCloudKitContainer {
        type Super = NSPersistentContainer;
    }
);

extern_methods!(
    unsafe impl NSPersistentCloudKitContainer {
        #[method(initializeCloudKitSchemaWithOptions:error:)]
        pub unsafe fn initializeCloudKitSchemaWithOptions_error(
            &self,
            options: NSPersistentCloudKitContainerSchemaInitializationOptions,
        ) -> Result<(), Id<NSError, Shared>>;

        #[method_id(@__retain_semantics Other recordForManagedObjectID:)]
        pub unsafe fn recordForManagedObjectID(
            &self,
            managedObjectID: &NSManagedObjectID,
        ) -> Option<Id<CKRecord, Shared>>;

        #[method_id(@__retain_semantics Other recordsForManagedObjectIDs:)]
        pub unsafe fn recordsForManagedObjectIDs(
            &self,
            managedObjectIDs: &NSArray<NSManagedObjectID>,
        ) -> Id<NSDictionary<NSManagedObjectID, CKRecord>, Shared>;

        #[method_id(@__retain_semantics Other recordIDForManagedObjectID:)]
        pub unsafe fn recordIDForManagedObjectID(
            &self,
            managedObjectID: &NSManagedObjectID,
        ) -> Option<Id<CKRecordID, Shared>>;

        #[method_id(@__retain_semantics Other recordIDsForManagedObjectIDs:)]
        pub unsafe fn recordIDsForManagedObjectIDs(
            &self,
            managedObjectIDs: &NSArray<NSManagedObjectID>,
        ) -> Id<NSDictionary<NSManagedObjectID, CKRecordID>, Shared>;

        #[method(canUpdateRecordForManagedObjectWithID:)]
        pub unsafe fn canUpdateRecordForManagedObjectWithID(
            &self,
            objectID: &NSManagedObjectID,
        ) -> bool;

        #[method(canDeleteRecordForManagedObjectWithID:)]
        pub unsafe fn canDeleteRecordForManagedObjectWithID(
            &self,
            objectID: &NSManagedObjectID,
        ) -> bool;

        #[method(canModifyManagedObjectsInStore:)]
        pub unsafe fn canModifyManagedObjectsInStore(&self, store: &NSPersistentStore) -> bool;
    }
);
