//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern "C" {
    pub static NSSQLiteStoreType: &'static NSString;
}

extern "C" {
    pub static NSXMLStoreType: &'static NSString;
}

extern "C" {
    pub static NSBinaryStoreType: &'static NSString;
}

extern "C" {
    pub static NSInMemoryStoreType: &'static NSString;
}

extern "C" {
    pub static NSStoreTypeKey: &'static NSString;
}

extern "C" {
    pub static NSStoreUUIDKey: &'static NSString;
}

extern "C" {
    pub static NSPersistentStoreCoordinatorStoresWillChangeNotification: &'static NSString;
}

extern "C" {
    pub static NSPersistentStoreCoordinatorStoresDidChangeNotification: &'static NSString;
}

extern "C" {
    pub static NSPersistentStoreCoordinatorWillRemoveStoreNotification: &'static NSString;
}

extern "C" {
    pub static NSAddedPersistentStoresKey: &'static NSString;
}

extern "C" {
    pub static NSRemovedPersistentStoresKey: &'static NSString;
}

extern "C" {
    pub static NSUUIDChangedPersistentStoresKey: &'static NSString;
}

extern "C" {
    pub static NSReadOnlyPersistentStoreOption: &'static NSString;
}

extern "C" {
    pub static NSValidateXMLStoreOption: &'static NSString;
}

extern "C" {
    pub static NSPersistentStoreTimeoutOption: &'static NSString;
}

extern "C" {
    pub static NSSQLitePragmasOption: &'static NSString;
}

extern "C" {
    pub static NSSQLiteAnalyzeOption: &'static NSString;
}

extern "C" {
    pub static NSSQLiteManualVacuumOption: &'static NSString;
}

extern "C" {
    pub static NSIgnorePersistentStoreVersioningOption: &'static NSString;
}

extern "C" {
    pub static NSMigratePersistentStoresAutomaticallyOption: &'static NSString;
}

extern "C" {
    pub static NSInferMappingModelAutomaticallyOption: &'static NSString;
}

extern "C" {
    pub static NSStoreModelVersionHashesKey: &'static NSString;
}

extern "C" {
    pub static NSStoreModelVersionIdentifiersKey: &'static NSString;
}

extern "C" {
    pub static NSPersistentStoreOSCompatibility: &'static NSString;
}

extern "C" {
    pub static NSPersistentStoreConnectionPoolMaxSizeKey: &'static NSString;
}

extern "C" {
    pub static NSCoreDataCoreSpotlightExporter: &'static NSString;
}

extern "C" {
    pub static NSXMLExternalRecordType: &'static NSString;
}

extern "C" {
    pub static NSBinaryExternalRecordType: &'static NSString;
}

extern "C" {
    pub static NSExternalRecordsFileFormatOption: &'static NSString;
}

extern "C" {
    pub static NSExternalRecordsDirectoryOption: &'static NSString;
}

extern "C" {
    pub static NSExternalRecordExtensionOption: &'static NSString;
}

extern "C" {
    pub static NSEntityNameInPathKey: &'static NSString;
}

extern "C" {
    pub static NSStoreUUIDInPathKey: &'static NSString;
}

extern "C" {
    pub static NSStorePathKey: &'static NSString;
}

extern "C" {
    pub static NSModelPathKey: &'static NSString;
}

extern "C" {
    pub static NSObjectURIKey: &'static NSString;
}

extern "C" {
    pub static NSPersistentStoreForceDestroyOption: &'static NSString;
}

extern "C" {
    pub static NSPersistentStoreFileProtectionKey: &'static NSString;
}

extern "C" {
    pub static NSPersistentHistoryTrackingKey: &'static NSString;
}

extern "C" {
    pub static NSBinaryStoreSecureDecodingClasses: &'static NSString;
}

extern "C" {
    pub static NSBinaryStoreInsecureDecodingCompatibilityOption: &'static NSString;
}

extern "C" {
    pub static NSPersistentStoreRemoteChangeNotificationPostOptionKey: &'static NSString;
}

extern "C" {
    pub static NSPersistentStoreRemoteChangeNotification: &'static NSString;
}

extern "C" {
    pub static NSPersistentStoreURLKey: &'static NSString;
}

extern "C" {
    pub static NSPersistentHistoryTokenKey: &'static NSString;
}

extern_class!(
    #[derive(Debug)]
    pub struct NSPersistentStoreCoordinator;

    unsafe impl ClassType for NSPersistentStoreCoordinator {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl NSPersistentStoreCoordinator {
        #[method_id(@__retain_semantics Init initWithManagedObjectModel:)]
        pub unsafe fn initWithManagedObjectModel(
            this: Option<Allocated<Self>>,
            model: &NSManagedObjectModel,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other managedObjectModel)]
        pub unsafe fn managedObjectModel(&self) -> Id<NSManagedObjectModel, Shared>;

        #[method_id(@__retain_semantics Other persistentStores)]
        pub unsafe fn persistentStores(&self) -> Id<NSArray<NSPersistentStore>, Shared>;

        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Option<Id<NSString, Shared>>;

        #[method(setName:)]
        pub unsafe fn setName(&self, name: Option<&NSString>);

        #[method_id(@__retain_semantics Other persistentStoreForURL:)]
        pub unsafe fn persistentStoreForURL(
            &self,
            URL: &NSURL,
        ) -> Option<Id<NSPersistentStore, Shared>>;

        #[method_id(@__retain_semantics Other URLForPersistentStore:)]
        pub unsafe fn URLForPersistentStore(&self, store: &NSPersistentStore) -> Id<NSURL, Shared>;

        #[method(setURL:forPersistentStore:)]
        pub unsafe fn setURL_forPersistentStore(
            &self,
            url: &NSURL,
            store: &NSPersistentStore,
        ) -> bool;

        #[method_id(@__retain_semantics Other addPersistentStoreWithType:configuration:URL:options:error:)]
        pub unsafe fn addPersistentStoreWithType_configuration_URL_options_error(
            &self,
            storeType: &NSString,
            configuration: Option<&NSString>,
            storeURL: Option<&NSURL>,
            options: Option<&NSDictionary>,
        ) -> Result<Id<NSPersistentStore, Shared>, Id<NSError, Shared>>;

        #[method(addPersistentStoreWithDescription:completionHandler:)]
        pub unsafe fn addPersistentStoreWithDescription_completionHandler(
            &self,
            storeDescription: &NSPersistentStoreDescription,
            block: TodoBlock,
        );

        #[method(removePersistentStore:error:)]
        pub unsafe fn removePersistentStore_error(
            &self,
            store: &NSPersistentStore,
        ) -> Result<(), Id<NSError, Shared>>;

        #[method(setMetadata:forPersistentStore:)]
        pub unsafe fn setMetadata_forPersistentStore(
            &self,
            metadata: Option<&NSDictionary<NSString, Object>>,
            store: &NSPersistentStore,
        );

        #[method_id(@__retain_semantics Other metadataForPersistentStore:)]
        pub unsafe fn metadataForPersistentStore(
            &self,
            store: &NSPersistentStore,
        ) -> Id<NSDictionary<NSString, Object>, Shared>;

        #[method_id(@__retain_semantics Other managedObjectIDForURIRepresentation:)]
        pub unsafe fn managedObjectIDForURIRepresentation(
            &self,
            url: &NSURL,
        ) -> Option<Id<NSManagedObjectID, Shared>>;

        #[method_id(@__retain_semantics Other executeRequest:withContext:error:)]
        pub unsafe fn executeRequest_withContext_error(
            &self,
            request: &NSPersistentStoreRequest,
            context: &NSManagedObjectContext,
        ) -> Result<Id<Object, Shared>, Id<NSError, Shared>>;

        #[method_id(@__retain_semantics Other registeredStoreTypes)]
        pub unsafe fn registeredStoreTypes() -> Id<NSDictionary<NSString, NSValue>, Shared>;

        #[method(registerStoreClass:forStoreType:)]
        pub unsafe fn registerStoreClass_forStoreType(
            storeClass: Option<&Class>,
            storeType: &NSString,
        );

        #[method_id(@__retain_semantics Other metadataForPersistentStoreOfType:URL:options:error:)]
        pub unsafe fn metadataForPersistentStoreOfType_URL_options_error(
            storeType: &NSString,
            url: &NSURL,
            options: Option<&NSDictionary>,
        ) -> Result<Id<NSDictionary<NSString, Object>, Shared>, Id<NSError, Shared>>;

        #[method(setMetadata:forPersistentStoreOfType:URL:options:error:)]
        pub unsafe fn setMetadata_forPersistentStoreOfType_URL_options_error(
            metadata: Option<&NSDictionary<NSString, Object>>,
            storeType: &NSString,
            url: &NSURL,
            options: Option<&NSDictionary>,
        ) -> Result<(), Id<NSError, Shared>>;

        #[method_id(@__retain_semantics Other elementsDerivedFromExternalRecordURL:)]
        pub unsafe fn elementsDerivedFromExternalRecordURL(
            fileURL: &NSURL,
        ) -> Id<NSDictionary, Shared>;

        #[method_id(@__retain_semantics Other importStoreWithIdentifier:fromExternalRecordsDirectory:toURL:options:withType:error:)]
        pub unsafe fn importStoreWithIdentifier_fromExternalRecordsDirectory_toURL_options_withType_error(
            &self,
            storeIdentifier: Option<&NSString>,
            externalRecordsURL: &NSURL,
            destinationURL: &NSURL,
            options: Option<&NSDictionary>,
            storeType: &NSString,
        ) -> Result<Id<NSPersistentStore, Shared>, Id<NSError, Shared>>;

        #[method_id(@__retain_semantics Other migratePersistentStore:toURL:options:withType:error:)]
        pub unsafe fn migratePersistentStore_toURL_options_withType_error(
            &self,
            store: &NSPersistentStore,
            URL: &NSURL,
            options: Option<&NSDictionary>,
            storeType: &NSString,
        ) -> Result<Id<NSPersistentStore, Shared>, Id<NSError, Shared>>;

        #[method(destroyPersistentStoreAtURL:withType:options:error:)]
        pub unsafe fn destroyPersistentStoreAtURL_withType_options_error(
            &self,
            url: &NSURL,
            storeType: &NSString,
            options: Option<&NSDictionary>,
        ) -> Result<(), Id<NSError, Shared>>;

        #[method(replacePersistentStoreAtURL:destinationOptions:withPersistentStoreFromURL:sourceOptions:storeType:error:)]
        pub unsafe fn replacePersistentStoreAtURL_destinationOptions_withPersistentStoreFromURL_sourceOptions_storeType_error(
            &self,
            destinationURL: &NSURL,
            destinationOptions: Option<&NSDictionary>,
            sourceURL: &NSURL,
            sourceOptions: Option<&NSDictionary>,
            storeType: &NSString,
        ) -> Result<(), Id<NSError, Shared>>;

        #[method(performBlock:)]
        pub unsafe fn performBlock(&self, block: TodoBlock);

        #[method(performBlockAndWait:)]
        pub unsafe fn performBlockAndWait(&self, block: TodoBlock);

        #[method_id(@__retain_semantics Other currentPersistentHistoryTokenFromStores:)]
        pub unsafe fn currentPersistentHistoryTokenFromStores(
            &self,
            stores: Option<&NSArray>,
        ) -> Option<Id<NSPersistentHistoryToken, Shared>>;

        #[method_id(@__retain_semantics Other metadataForPersistentStoreWithURL:error:)]
        pub unsafe fn metadataForPersistentStoreWithURL_error(
            url: &NSURL,
        ) -> Result<Id<NSDictionary, Shared>, Id<NSError, Shared>>;

        #[method(lock)]
        pub unsafe fn lock(&self);

        #[method(unlock)]
        pub unsafe fn unlock(&self);

        #[method(tryLock)]
        pub unsafe fn tryLock(&self) -> bool;

        #[method_id(@__retain_semantics Other metadataForPersistentStoreOfType:URL:error:)]
        pub unsafe fn metadataForPersistentStoreOfType_URL_error(
            storeType: Option<&NSString>,
            url: &NSURL,
        ) -> Result<Id<NSDictionary<NSString, Object>, Shared>, Id<NSError, Shared>>;

        #[method(setMetadata:forPersistentStoreOfType:URL:error:)]
        pub unsafe fn setMetadata_forPersistentStoreOfType_URL_error(
            metadata: Option<&NSDictionary<NSString, Object>>,
            storeType: Option<&NSString>,
            url: &NSURL,
        ) -> Result<(), Id<NSError, Shared>>;

        #[method(removeUbiquitousContentAndPersistentStoreAtURL:options:error:)]
        pub unsafe fn removeUbiquitousContentAndPersistentStoreAtURL_options_error(
            storeURL: &NSURL,
            options: Option<&NSDictionary>,
        ) -> Result<(), Id<NSError, Shared>>;
    }
);

pub type NSPersistentStoreUbiquitousTransitionType = NSUInteger;
pub const NSPersistentStoreUbiquitousTransitionTypeAccountAdded:
    NSPersistentStoreUbiquitousTransitionType = 1;
pub const NSPersistentStoreUbiquitousTransitionTypeAccountRemoved:
    NSPersistentStoreUbiquitousTransitionType = 2;
pub const NSPersistentStoreUbiquitousTransitionTypeContentRemoved:
    NSPersistentStoreUbiquitousTransitionType = 3;
pub const NSPersistentStoreUbiquitousTransitionTypeInitialImportCompleted:
    NSPersistentStoreUbiquitousTransitionType = 4;

extern "C" {
    pub static NSPersistentStoreUbiquitousContentNameKey: &'static NSString;
}

extern "C" {
    pub static NSPersistentStoreUbiquitousContentURLKey: &'static NSString;
}

extern "C" {
    pub static NSPersistentStoreDidImportUbiquitousContentChangesNotification: &'static NSString;
}

extern "C" {
    pub static NSPersistentStoreUbiquitousTransitionTypeKey: &'static NSString;
}

extern "C" {
    pub static NSPersistentStoreUbiquitousPeerTokenOption: &'static NSString;
}

extern "C" {
    pub static NSPersistentStoreRemoveUbiquitousMetadataOption: &'static NSString;
}

extern "C" {
    pub static NSPersistentStoreUbiquitousContainerIdentifierKey: &'static NSString;
}

extern "C" {
    pub static NSPersistentStoreRebuildFromUbiquitousContentOption: &'static NSString;
}
