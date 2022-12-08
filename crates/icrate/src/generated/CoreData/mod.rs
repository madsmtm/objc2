pub(crate) mod CoreDataDefines;
pub(crate) mod CoreDataErrors;
pub(crate) mod NSAtomicStore;
pub(crate) mod NSAtomicStoreCacheNode;
pub(crate) mod NSAttributeDescription;
pub(crate) mod NSBatchDeleteRequest;
pub(crate) mod NSBatchInsertRequest;
pub(crate) mod NSBatchUpdateRequest;
pub(crate) mod NSCoreDataCoreSpotlightDelegate;
pub(crate) mod NSDerivedAttributeDescription;
pub(crate) mod NSEntityDescription;
pub(crate) mod NSEntityMapping;
pub(crate) mod NSEntityMigrationPolicy;
pub(crate) mod NSExpressionDescription;
pub(crate) mod NSFetchIndexDescription;
pub(crate) mod NSFetchIndexElementDescription;
pub(crate) mod NSFetchRequest;
pub(crate) mod NSFetchRequestExpression;
pub(crate) mod NSFetchedPropertyDescription;
pub(crate) mod NSFetchedResultsController;
pub(crate) mod NSIncrementalStore;
pub(crate) mod NSIncrementalStoreNode;
pub(crate) mod NSManagedObject;
pub(crate) mod NSManagedObjectContext;
pub(crate) mod NSManagedObjectID;
pub(crate) mod NSManagedObjectModel;
pub(crate) mod NSMappingModel;
pub(crate) mod NSMergePolicy;
pub(crate) mod NSMigrationManager;
pub(crate) mod NSPersistentCloudKitContainer;
pub(crate) mod NSPersistentCloudKitContainerEvent;
pub(crate) mod NSPersistentCloudKitContainerEventRequest;
pub(crate) mod NSPersistentCloudKitContainerOptions;
pub(crate) mod NSPersistentContainer;
pub(crate) mod NSPersistentHistoryChange;
pub(crate) mod NSPersistentHistoryChangeRequest;
pub(crate) mod NSPersistentHistoryToken;
pub(crate) mod NSPersistentHistoryTransaction;
pub(crate) mod NSPersistentStore;
pub(crate) mod NSPersistentStoreCoordinator;
pub(crate) mod NSPersistentStoreDescription;
pub(crate) mod NSPersistentStoreRequest;
pub(crate) mod NSPersistentStoreResult;
pub(crate) mod NSPropertyDescription;
pub(crate) mod NSPropertyMapping;
pub(crate) mod NSQueryGenerationToken;
pub(crate) mod NSRelationshipDescription;
pub(crate) mod NSSaveChangesRequest;

mod __exported {
    pub use super::CoreDataDefines::NSCoreDataVersionNumber;
    pub use super::CoreDataErrors::{
        NSAffectedObjectsErrorKey, NSAffectedStoresErrorKey, NSCoreDataError, NSDetailedErrorsKey,
        NSEntityMigrationPolicyError, NSExternalRecordImportError, NSInferredMappingModelError,
        NSManagedObjectConstraintMergeError, NSManagedObjectConstraintValidationError,
        NSManagedObjectContextLockingError, NSManagedObjectExternalRelationshipError,
        NSManagedObjectMergeError, NSManagedObjectReferentialIntegrityError,
        NSManagedObjectValidationError, NSMigrationCancelledError,
        NSMigrationConstraintViolationError, NSMigrationError,
        NSMigrationManagerDestinationStoreError, NSMigrationManagerSourceStoreError,
        NSMigrationMissingMappingModelError, NSMigrationMissingSourceModelError,
        NSPersistentHistoryTokenExpiredError, NSPersistentStoreCoordinatorLockingError,
        NSPersistentStoreIncompatibleSchemaError, NSPersistentStoreIncompatibleVersionHashError,
        NSPersistentStoreIncompleteSaveError, NSPersistentStoreInvalidTypeError,
        NSPersistentStoreOpenError, NSPersistentStoreOperationError,
        NSPersistentStoreSaveConflictsError, NSPersistentStoreSaveConflictsErrorKey,
        NSPersistentStoreSaveError, NSPersistentStoreTimeoutError,
        NSPersistentStoreTypeMismatchError, NSPersistentStoreUnsupportedRequestTypeError,
        NSSQLiteError, NSSQLiteErrorDomain, NSValidationDateTooLateError,
        NSValidationDateTooSoonError, NSValidationInvalidDateError, NSValidationInvalidURIError,
        NSValidationKeyErrorKey, NSValidationMissingMandatoryPropertyError,
        NSValidationMultipleErrorsError, NSValidationNumberTooLargeError,
        NSValidationNumberTooSmallError, NSValidationObjectErrorKey, NSValidationPredicateErrorKey,
        NSValidationRelationshipDeniedDeleteError,
        NSValidationRelationshipExceedsMaximumCountError,
        NSValidationRelationshipLacksMinimumCountError, NSValidationStringPatternMatchingError,
        NSValidationStringTooLongError, NSValidationStringTooShortError, NSValidationValueErrorKey,
    };
    pub use super::NSAtomicStore::NSAtomicStore;
    pub use super::NSAtomicStoreCacheNode::NSAtomicStoreCacheNode;
    pub use super::NSAttributeDescription::{
        NSAttributeDescription, NSAttributeType, NSBinaryDataAttributeType, NSBooleanAttributeType,
        NSDateAttributeType, NSDecimalAttributeType, NSDoubleAttributeType, NSFloatAttributeType,
        NSInteger16AttributeType, NSInteger32AttributeType, NSInteger64AttributeType,
        NSObjectIDAttributeType, NSStringAttributeType, NSTransformableAttributeType,
        NSURIAttributeType, NSUUIDAttributeType, NSUndefinedAttributeType,
    };
    pub use super::NSBatchDeleteRequest::NSBatchDeleteRequest;
    pub use super::NSBatchInsertRequest::NSBatchInsertRequest;
    pub use super::NSBatchUpdateRequest::NSBatchUpdateRequest;
    pub use super::NSCoreDataCoreSpotlightDelegate::{
        NSCoreDataCoreSpotlightDelegate, NSCoreDataCoreSpotlightDelegateIndexDidUpdateNotification,
    };
    pub use super::NSDerivedAttributeDescription::NSDerivedAttributeDescription;
    pub use super::NSEntityDescription::NSEntityDescription;
    pub use super::NSEntityMapping::{
        NSAddEntityMappingType, NSCopyEntityMappingType, NSCustomEntityMappingType,
        NSEntityMapping, NSEntityMappingType, NSRemoveEntityMappingType,
        NSTransformEntityMappingType, NSUndefinedEntityMappingType,
    };
    pub use super::NSEntityMigrationPolicy::{
        NSEntityMigrationPolicy, NSMigrationDestinationObjectKey, NSMigrationEntityMappingKey,
        NSMigrationEntityPolicyKey, NSMigrationManagerKey, NSMigrationPropertyMappingKey,
        NSMigrationSourceObjectKey,
    };
    pub use super::NSExpressionDescription::NSExpressionDescription;
    pub use super::NSFetchIndexDescription::NSFetchIndexDescription;
    pub use super::NSFetchIndexElementDescription::{
        NSFetchIndexElementDescription, NSFetchIndexElementType, NSFetchIndexElementTypeBinary,
        NSFetchIndexElementTypeRTree,
    };
    pub use super::NSFetchRequest::{
        NSAsynchronousFetchRequest, NSCountResultType, NSDictionaryResultType, NSFetchRequest,
        NSFetchRequestResult, NSFetchRequestResultType, NSManagedObjectIDResultType,
        NSManagedObjectResultType, NSPersistentStoreAsynchronousFetchResultCompletionBlock,
    };
    pub use super::NSFetchRequestExpression::{
        NSFetchRequestExpression, NSFetchRequestExpressionType,
    };
    pub use super::NSFetchedPropertyDescription::NSFetchedPropertyDescription;
    pub use super::NSFetchedResultsController::{
        NSFetchedResultsChangeDelete, NSFetchedResultsChangeInsert, NSFetchedResultsChangeMove,
        NSFetchedResultsChangeType, NSFetchedResultsChangeUpdate, NSFetchedResultsController,
        NSFetchedResultsControllerDelegate, NSFetchedResultsSectionInfo,
    };
    pub use super::NSIncrementalStore::NSIncrementalStore;
    pub use super::NSIncrementalStoreNode::NSIncrementalStoreNode;
    pub use super::NSManagedObject::{
        NSManagedObject, NSSnapshotEventMergePolicy, NSSnapshotEventRefresh,
        NSSnapshotEventRollback, NSSnapshotEventType, NSSnapshotEventUndoDeletion,
        NSSnapshotEventUndoInsertion, NSSnapshotEventUndoUpdate,
    };
    pub use super::NSManagedObjectContext::{
        NSConfinementConcurrencyType, NSDeletedObjectIDsKey, NSDeletedObjectsKey,
        NSErrorMergePolicy, NSInsertedObjectIDsKey, NSInsertedObjectsKey,
        NSInvalidatedAllObjectsKey, NSInvalidatedObjectIDsKey, NSInvalidatedObjectsKey,
        NSMainQueueConcurrencyType, NSManagedObjectContext, NSManagedObjectContextConcurrencyType,
        NSManagedObjectContextDidMergeChangesObjectIDsNotification,
        NSManagedObjectContextDidSaveNotification,
        NSManagedObjectContextDidSaveObjectIDsNotification,
        NSManagedObjectContextObjectsDidChangeNotification,
        NSManagedObjectContextQueryGenerationKey, NSManagedObjectContextWillSaveNotification,
        NSMergeByPropertyObjectTrumpMergePolicy, NSMergeByPropertyStoreTrumpMergePolicy,
        NSOverwriteMergePolicy, NSPrivateQueueConcurrencyType, NSRefreshedObjectIDsKey,
        NSRefreshedObjectsKey, NSRollbackMergePolicy, NSUpdatedObjectIDsKey, NSUpdatedObjectsKey,
    };
    pub use super::NSManagedObjectID::NSManagedObjectID;
    pub use super::NSManagedObjectModel::NSManagedObjectModel;
    pub use super::NSMappingModel::NSMappingModel;
    pub use super::NSMergePolicy::{
        NSConstraintConflict, NSErrorMergePolicy, NSErrorMergePolicyType,
        NSMergeByPropertyObjectTrumpMergePolicy, NSMergeByPropertyObjectTrumpMergePolicyType,
        NSMergeByPropertyStoreTrumpMergePolicy, NSMergeByPropertyStoreTrumpMergePolicyType,
        NSMergeConflict, NSMergePolicy, NSMergePolicyType, NSOverwriteMergePolicy,
        NSOverwriteMergePolicyType, NSRollbackMergePolicy, NSRollbackMergePolicyType,
    };
    pub use super::NSMigrationManager::NSMigrationManager;
    pub use super::NSPersistentCloudKitContainer::{
        NSPersistentCloudKitContainer, NSPersistentCloudKitContainerSchemaInitializationOptions,
        NSPersistentCloudKitContainerSchemaInitializationOptionsDryRun,
        NSPersistentCloudKitContainerSchemaInitializationOptionsNone,
        NSPersistentCloudKitContainerSchemaInitializationOptionsPrintSchema,
    };
    pub use super::NSPersistentCloudKitContainerEvent::{
        NSPersistentCloudKitContainerEvent, NSPersistentCloudKitContainerEventChangedNotification,
        NSPersistentCloudKitContainerEventType, NSPersistentCloudKitContainerEventTypeExport,
        NSPersistentCloudKitContainerEventTypeImport, NSPersistentCloudKitContainerEventTypeSetup,
        NSPersistentCloudKitContainerEventUserInfoKey,
    };
    pub use super::NSPersistentCloudKitContainerEventRequest::NSPersistentCloudKitContainerEventRequest;
    pub use super::NSPersistentCloudKitContainerOptions::NSPersistentCloudKitContainerOptions;
    pub use super::NSPersistentContainer::NSPersistentContainer;
    pub use super::NSPersistentHistoryChange::{
        NSPersistentHistoryChange, NSPersistentHistoryChangeType,
        NSPersistentHistoryChangeTypeDelete, NSPersistentHistoryChangeTypeInsert,
        NSPersistentHistoryChangeTypeUpdate,
    };
    pub use super::NSPersistentHistoryChangeRequest::NSPersistentHistoryChangeRequest;
    pub use super::NSPersistentHistoryToken::NSPersistentHistoryToken;
    pub use super::NSPersistentHistoryTransaction::NSPersistentHistoryTransaction;
    pub use super::NSPersistentStore::NSPersistentStore;
    pub use super::NSPersistentStoreCoordinator::{
        NSAddedPersistentStoresKey, NSBinaryExternalRecordType,
        NSBinaryStoreInsecureDecodingCompatibilityOption, NSBinaryStoreSecureDecodingClasses,
        NSBinaryStoreType, NSCoreDataCoreSpotlightExporter, NSEntityNameInPathKey,
        NSExternalRecordExtensionOption, NSExternalRecordsDirectoryOption,
        NSExternalRecordsFileFormatOption, NSIgnorePersistentStoreVersioningOption,
        NSInMemoryStoreType, NSInferMappingModelAutomaticallyOption,
        NSMigratePersistentStoresAutomaticallyOption, NSModelPathKey, NSObjectURIKey,
        NSPersistentHistoryTokenKey, NSPersistentHistoryTrackingKey,
        NSPersistentStoreConnectionPoolMaxSizeKey, NSPersistentStoreCoordinator,
        NSPersistentStoreCoordinatorStoresDidChangeNotification,
        NSPersistentStoreCoordinatorStoresWillChangeNotification,
        NSPersistentStoreCoordinatorWillRemoveStoreNotification,
        NSPersistentStoreDidImportUbiquitousContentChangesNotification,
        NSPersistentStoreFileProtectionKey, NSPersistentStoreForceDestroyOption,
        NSPersistentStoreOSCompatibility, NSPersistentStoreRebuildFromUbiquitousContentOption,
        NSPersistentStoreRemoteChangeNotification,
        NSPersistentStoreRemoteChangeNotificationPostOptionKey,
        NSPersistentStoreRemoveUbiquitousMetadataOption, NSPersistentStoreTimeoutOption,
        NSPersistentStoreURLKey, NSPersistentStoreUbiquitousContainerIdentifierKey,
        NSPersistentStoreUbiquitousContentNameKey, NSPersistentStoreUbiquitousContentURLKey,
        NSPersistentStoreUbiquitousPeerTokenOption, NSPersistentStoreUbiquitousTransitionType,
        NSPersistentStoreUbiquitousTransitionTypeAccountAdded,
        NSPersistentStoreUbiquitousTransitionTypeAccountRemoved,
        NSPersistentStoreUbiquitousTransitionTypeContentRemoved,
        NSPersistentStoreUbiquitousTransitionTypeInitialImportCompleted,
        NSPersistentStoreUbiquitousTransitionTypeKey, NSReadOnlyPersistentStoreOption,
        NSRemovedPersistentStoresKey, NSSQLiteAnalyzeOption, NSSQLiteManualVacuumOption,
        NSSQLitePragmasOption, NSSQLiteStoreType, NSStoreModelVersionHashesKey,
        NSStoreModelVersionIdentifiersKey, NSStorePathKey, NSStoreTypeKey, NSStoreUUIDInPathKey,
        NSStoreUUIDKey, NSUUIDChangedPersistentStoresKey, NSValidateXMLStoreOption,
        NSXMLExternalRecordType, NSXMLStoreType,
    };
    pub use super::NSPersistentStoreDescription::NSPersistentStoreDescription;
    pub use super::NSPersistentStoreRequest::{
        NSBatchDeleteRequestType, NSBatchInsertRequestType, NSBatchUpdateRequestType,
        NSFetchRequestType, NSPersistentStoreRequest, NSPersistentStoreRequestType,
        NSSaveRequestType,
    };
    pub use super::NSPersistentStoreResult::{
        NSAsynchronousFetchResult, NSBatchDeleteRequestResultType, NSBatchDeleteResult,
        NSBatchDeleteResultTypeCount, NSBatchDeleteResultTypeObjectIDs,
        NSBatchDeleteResultTypeStatusOnly, NSBatchInsertRequestResultType,
        NSBatchInsertRequestResultTypeCount, NSBatchInsertRequestResultTypeObjectIDs,
        NSBatchInsertRequestResultTypeStatusOnly, NSBatchInsertResult,
        NSBatchUpdateRequestResultType, NSBatchUpdateResult,
        NSPersistentCloudKitContainerEventResult, NSPersistentCloudKitContainerEventResultType,
        NSPersistentCloudKitContainerEventResultTypeCountEvents,
        NSPersistentCloudKitContainerEventResultTypeEvents, NSPersistentHistoryResult,
        NSPersistentHistoryResultType, NSPersistentHistoryResultTypeChangesOnly,
        NSPersistentHistoryResultTypeCount, NSPersistentHistoryResultTypeObjectIDs,
        NSPersistentHistoryResultTypeStatusOnly,
        NSPersistentHistoryResultTypeTransactionsAndChanges,
        NSPersistentHistoryResultTypeTransactionsOnly, NSPersistentStoreAsynchronousResult,
        NSPersistentStoreResult, NSStatusOnlyResultType, NSUpdatedObjectIDsResultType,
        NSUpdatedObjectsCountResultType,
    };
    pub use super::NSPropertyDescription::NSPropertyDescription;
    pub use super::NSPropertyMapping::NSPropertyMapping;
    pub use super::NSQueryGenerationToken::NSQueryGenerationToken;
    pub use super::NSRelationshipDescription::{
        NSCascadeDeleteRule, NSDeleteRule, NSDenyDeleteRule, NSNoActionDeleteRule,
        NSNullifyDeleteRule, NSRelationshipDescription,
    };
    pub use super::NSSaveChangesRequest::NSSaveChangesRequest;
}
