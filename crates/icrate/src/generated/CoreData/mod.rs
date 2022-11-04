#[path = "CoreDataDefines.rs"]
mod __CoreDataDefines;
#[path = "CoreDataErrors.rs"]
mod __CoreDataErrors;
#[path = "NSAtomicStore.rs"]
mod __NSAtomicStore;
#[path = "NSAtomicStoreCacheNode.rs"]
mod __NSAtomicStoreCacheNode;
#[path = "NSAttributeDescription.rs"]
mod __NSAttributeDescription;
#[path = "NSBatchDeleteRequest.rs"]
mod __NSBatchDeleteRequest;
#[path = "NSBatchInsertRequest.rs"]
mod __NSBatchInsertRequest;
#[path = "NSBatchUpdateRequest.rs"]
mod __NSBatchUpdateRequest;
#[path = "NSCoreDataCoreSpotlightDelegate.rs"]
mod __NSCoreDataCoreSpotlightDelegate;
#[path = "NSDerivedAttributeDescription.rs"]
mod __NSDerivedAttributeDescription;
#[path = "NSEntityDescription.rs"]
mod __NSEntityDescription;
#[path = "NSEntityMapping.rs"]
mod __NSEntityMapping;
#[path = "NSEntityMigrationPolicy.rs"]
mod __NSEntityMigrationPolicy;
#[path = "NSExpressionDescription.rs"]
mod __NSExpressionDescription;
#[path = "NSFetchIndexDescription.rs"]
mod __NSFetchIndexDescription;
#[path = "NSFetchIndexElementDescription.rs"]
mod __NSFetchIndexElementDescription;
#[path = "NSFetchRequest.rs"]
mod __NSFetchRequest;
#[path = "NSFetchRequestExpression.rs"]
mod __NSFetchRequestExpression;
#[path = "NSFetchedPropertyDescription.rs"]
mod __NSFetchedPropertyDescription;
#[path = "NSFetchedResultsController.rs"]
mod __NSFetchedResultsController;
#[path = "NSIncrementalStore.rs"]
mod __NSIncrementalStore;
#[path = "NSIncrementalStoreNode.rs"]
mod __NSIncrementalStoreNode;
#[path = "NSManagedObject.rs"]
mod __NSManagedObject;
#[path = "NSManagedObjectContext.rs"]
mod __NSManagedObjectContext;
#[path = "NSManagedObjectID.rs"]
mod __NSManagedObjectID;
#[path = "NSManagedObjectModel.rs"]
mod __NSManagedObjectModel;
#[path = "NSMappingModel.rs"]
mod __NSMappingModel;
#[path = "NSMergePolicy.rs"]
mod __NSMergePolicy;
#[path = "NSMigrationManager.rs"]
mod __NSMigrationManager;
#[path = "NSPersistentCloudKitContainer.rs"]
mod __NSPersistentCloudKitContainer;
#[path = "NSPersistentCloudKitContainerEvent.rs"]
mod __NSPersistentCloudKitContainerEvent;
#[path = "NSPersistentCloudKitContainerEventRequest.rs"]
mod __NSPersistentCloudKitContainerEventRequest;
#[path = "NSPersistentCloudKitContainerOptions.rs"]
mod __NSPersistentCloudKitContainerOptions;
#[path = "NSPersistentContainer.rs"]
mod __NSPersistentContainer;
#[path = "NSPersistentHistoryChange.rs"]
mod __NSPersistentHistoryChange;
#[path = "NSPersistentHistoryChangeRequest.rs"]
mod __NSPersistentHistoryChangeRequest;
#[path = "NSPersistentHistoryToken.rs"]
mod __NSPersistentHistoryToken;
#[path = "NSPersistentHistoryTransaction.rs"]
mod __NSPersistentHistoryTransaction;
#[path = "NSPersistentStore.rs"]
mod __NSPersistentStore;
#[path = "NSPersistentStoreCoordinator.rs"]
mod __NSPersistentStoreCoordinator;
#[path = "NSPersistentStoreDescription.rs"]
mod __NSPersistentStoreDescription;
#[path = "NSPersistentStoreRequest.rs"]
mod __NSPersistentStoreRequest;
#[path = "NSPersistentStoreResult.rs"]
mod __NSPersistentStoreResult;
#[path = "NSPropertyDescription.rs"]
mod __NSPropertyDescription;
#[path = "NSPropertyMapping.rs"]
mod __NSPropertyMapping;
#[path = "NSQueryGenerationToken.rs"]
mod __NSQueryGenerationToken;
#[path = "NSRelationshipDescription.rs"]
mod __NSRelationshipDescription;
#[path = "NSSaveChangesRequest.rs"]
mod __NSSaveChangesRequest;

pub use self::__CoreDataDefines::NSCoreDataVersionNumber;
pub use self::__CoreDataErrors::{
    NSAffectedObjectsErrorKey, NSAffectedStoresErrorKey, NSCoreDataError, NSDetailedErrorsKey,
    NSEntityMigrationPolicyError, NSExternalRecordImportError, NSInferredMappingModelError,
    NSManagedObjectConstraintMergeError, NSManagedObjectConstraintValidationError,
    NSManagedObjectContextLockingError, NSManagedObjectExternalRelationshipError,
    NSManagedObjectMergeError, NSManagedObjectReferentialIntegrityError,
    NSManagedObjectValidationError, NSMigrationCancelledError, NSMigrationConstraintViolationError,
    NSMigrationError, NSMigrationManagerDestinationStoreError, NSMigrationManagerSourceStoreError,
    NSMigrationMissingMappingModelError, NSMigrationMissingSourceModelError,
    NSPersistentHistoryTokenExpiredError, NSPersistentStoreCoordinatorLockingError,
    NSPersistentStoreIncompatibleSchemaError, NSPersistentStoreIncompatibleVersionHashError,
    NSPersistentStoreIncompleteSaveError, NSPersistentStoreInvalidTypeError,
    NSPersistentStoreOpenError, NSPersistentStoreOperationError,
    NSPersistentStoreSaveConflictsError, NSPersistentStoreSaveConflictsErrorKey,
    NSPersistentStoreSaveError, NSPersistentStoreTimeoutError, NSPersistentStoreTypeMismatchError,
    NSPersistentStoreUnsupportedRequestTypeError, NSSQLiteError, NSSQLiteErrorDomain,
    NSValidationDateTooLateError, NSValidationDateTooSoonError, NSValidationInvalidDateError,
    NSValidationInvalidURIError, NSValidationKeyErrorKey,
    NSValidationMissingMandatoryPropertyError, NSValidationMultipleErrorsError,
    NSValidationNumberTooLargeError, NSValidationNumberTooSmallError, NSValidationObjectErrorKey,
    NSValidationPredicateErrorKey, NSValidationRelationshipDeniedDeleteError,
    NSValidationRelationshipExceedsMaximumCountError,
    NSValidationRelationshipLacksMinimumCountError, NSValidationStringPatternMatchingError,
    NSValidationStringTooLongError, NSValidationStringTooShortError, NSValidationValueErrorKey,
};
pub use self::__NSAtomicStore::NSAtomicStore;
pub use self::__NSAtomicStoreCacheNode::NSAtomicStoreCacheNode;
pub use self::__NSAttributeDescription::{
    NSAttributeDescription, NSAttributeType, NSBinaryDataAttributeType, NSBooleanAttributeType,
    NSDateAttributeType, NSDecimalAttributeType, NSDoubleAttributeType, NSFloatAttributeType,
    NSInteger16AttributeType, NSInteger32AttributeType, NSInteger64AttributeType,
    NSObjectIDAttributeType, NSStringAttributeType, NSTransformableAttributeType,
    NSURIAttributeType, NSUUIDAttributeType, NSUndefinedAttributeType,
};
pub use self::__NSBatchDeleteRequest::NSBatchDeleteRequest;
pub use self::__NSBatchInsertRequest::NSBatchInsertRequest;
pub use self::__NSBatchUpdateRequest::NSBatchUpdateRequest;
pub use self::__NSCoreDataCoreSpotlightDelegate::{
    NSCoreDataCoreSpotlightDelegate, NSCoreDataCoreSpotlightDelegateIndexDidUpdateNotification,
};
pub use self::__NSDerivedAttributeDescription::NSDerivedAttributeDescription;
pub use self::__NSEntityDescription::NSEntityDescription;
pub use self::__NSEntityMapping::{
    NSAddEntityMappingType, NSCopyEntityMappingType, NSCustomEntityMappingType, NSEntityMapping,
    NSEntityMappingType, NSRemoveEntityMappingType, NSTransformEntityMappingType,
    NSUndefinedEntityMappingType,
};
pub use self::__NSEntityMigrationPolicy::{
    NSEntityMigrationPolicy, NSMigrationDestinationObjectKey, NSMigrationEntityMappingKey,
    NSMigrationEntityPolicyKey, NSMigrationManagerKey, NSMigrationPropertyMappingKey,
    NSMigrationSourceObjectKey,
};
pub use self::__NSExpressionDescription::NSExpressionDescription;
pub use self::__NSFetchIndexDescription::NSFetchIndexDescription;
pub use self::__NSFetchIndexElementDescription::{
    NSFetchIndexElementDescription, NSFetchIndexElementType, NSFetchIndexElementTypeBinary,
    NSFetchIndexElementTypeRTree,
};
pub use self::__NSFetchRequest::{
    NSAsynchronousFetchRequest, NSCountResultType, NSDictionaryResultType, NSFetchRequest,
    NSFetchRequestResult, NSFetchRequestResultType, NSManagedObjectIDResultType,
    NSManagedObjectResultType, NSPersistentStoreAsynchronousFetchResultCompletionBlock,
};
pub use self::__NSFetchRequestExpression::{
    NSFetchRequestExpression, NSFetchRequestExpressionType,
};
pub use self::__NSFetchedPropertyDescription::NSFetchedPropertyDescription;
pub use self::__NSFetchedResultsController::{
    NSFetchedResultsChangeDelete, NSFetchedResultsChangeInsert, NSFetchedResultsChangeMove,
    NSFetchedResultsChangeType, NSFetchedResultsChangeUpdate, NSFetchedResultsController,
    NSFetchedResultsControllerDelegate, NSFetchedResultsSectionInfo,
};
pub use self::__NSIncrementalStore::NSIncrementalStore;
pub use self::__NSIncrementalStoreNode::NSIncrementalStoreNode;
pub use self::__NSManagedObject::{
    NSManagedObject, NSSnapshotEventMergePolicy, NSSnapshotEventRefresh, NSSnapshotEventRollback,
    NSSnapshotEventType, NSSnapshotEventUndoDeletion, NSSnapshotEventUndoInsertion,
    NSSnapshotEventUndoUpdate,
};
pub use self::__NSManagedObjectContext::{
    NSConfinementConcurrencyType, NSDeletedObjectIDsKey, NSDeletedObjectsKey,
    NSInsertedObjectIDsKey, NSInsertedObjectsKey, NSInvalidatedAllObjectsKey,
    NSInvalidatedObjectIDsKey, NSInvalidatedObjectsKey, NSMainQueueConcurrencyType,
    NSManagedObjectContext, NSManagedObjectContextConcurrencyType,
    NSManagedObjectContextDidMergeChangesObjectIDsNotification,
    NSManagedObjectContextDidSaveNotification, NSManagedObjectContextDidSaveObjectIDsNotification,
    NSManagedObjectContextObjectsDidChangeNotification, NSManagedObjectContextQueryGenerationKey,
    NSManagedObjectContextWillSaveNotification, NSPrivateQueueConcurrencyType,
    NSRefreshedObjectIDsKey, NSRefreshedObjectsKey, NSUpdatedObjectIDsKey, NSUpdatedObjectsKey,
};
pub use self::__NSManagedObjectID::NSManagedObjectID;
pub use self::__NSManagedObjectModel::NSManagedObjectModel;
pub use self::__NSMappingModel::NSMappingModel;
pub use self::__NSMergePolicy::{
    NSConstraintConflict, NSErrorMergePolicyType, NSMergeByPropertyObjectTrumpMergePolicyType,
    NSMergeByPropertyStoreTrumpMergePolicyType, NSMergeConflict, NSMergePolicy, NSMergePolicyType,
    NSOverwriteMergePolicyType, NSRollbackMergePolicyType,
};
pub use self::__NSMigrationManager::NSMigrationManager;
pub use self::__NSPersistentCloudKitContainer::{
    NSPersistentCloudKitContainer, NSPersistentCloudKitContainerSchemaInitializationOptions,
    NSPersistentCloudKitContainerSchemaInitializationOptionsDryRun,
    NSPersistentCloudKitContainerSchemaInitializationOptionsNone,
    NSPersistentCloudKitContainerSchemaInitializationOptionsPrintSchema,
};
pub use self::__NSPersistentCloudKitContainerEvent::{
    NSPersistentCloudKitContainerEvent, NSPersistentCloudKitContainerEventChangedNotification,
    NSPersistentCloudKitContainerEventType, NSPersistentCloudKitContainerEventTypeExport,
    NSPersistentCloudKitContainerEventTypeImport, NSPersistentCloudKitContainerEventTypeSetup,
    NSPersistentCloudKitContainerEventUserInfoKey,
};
pub use self::__NSPersistentCloudKitContainerEventRequest::NSPersistentCloudKitContainerEventRequest;
pub use self::__NSPersistentCloudKitContainerOptions::NSPersistentCloudKitContainerOptions;
pub use self::__NSPersistentContainer::NSPersistentContainer;
pub use self::__NSPersistentHistoryChange::{
    NSPersistentHistoryChange, NSPersistentHistoryChangeType, NSPersistentHistoryChangeTypeDelete,
    NSPersistentHistoryChangeTypeInsert, NSPersistentHistoryChangeTypeUpdate,
};
pub use self::__NSPersistentHistoryChangeRequest::NSPersistentHistoryChangeRequest;
pub use self::__NSPersistentHistoryToken::NSPersistentHistoryToken;
pub use self::__NSPersistentHistoryTransaction::NSPersistentHistoryTransaction;
pub use self::__NSPersistentStore::NSPersistentStore;
pub use self::__NSPersistentStoreCoordinator::{
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
pub use self::__NSPersistentStoreDescription::NSPersistentStoreDescription;
pub use self::__NSPersistentStoreRequest::{
    NSBatchDeleteRequestType, NSBatchInsertRequestType, NSBatchUpdateRequestType,
    NSFetchRequestType, NSPersistentStoreRequest, NSPersistentStoreRequestType, NSSaveRequestType,
};
pub use self::__NSPersistentStoreResult::{
    NSAsynchronousFetchResult, NSBatchDeleteRequestResultType, NSBatchDeleteResult,
    NSBatchDeleteResultTypeCount, NSBatchDeleteResultTypeObjectIDs,
    NSBatchDeleteResultTypeStatusOnly, NSBatchInsertRequestResultType,
    NSBatchInsertRequestResultTypeCount, NSBatchInsertRequestResultTypeObjectIDs,
    NSBatchInsertRequestResultTypeStatusOnly, NSBatchInsertResult, NSBatchUpdateRequestResultType,
    NSBatchUpdateResult, NSPersistentCloudKitContainerEventResult,
    NSPersistentCloudKitContainerEventResultType,
    NSPersistentCloudKitContainerEventResultTypeCountEvents,
    NSPersistentCloudKitContainerEventResultTypeEvents, NSPersistentHistoryResult,
    NSPersistentHistoryResultType, NSPersistentHistoryResultTypeChangesOnly,
    NSPersistentHistoryResultTypeCount, NSPersistentHistoryResultTypeObjectIDs,
    NSPersistentHistoryResultTypeStatusOnly, NSPersistentHistoryResultTypeTransactionsAndChanges,
    NSPersistentHistoryResultTypeTransactionsOnly, NSPersistentStoreAsynchronousResult,
    NSPersistentStoreResult, NSStatusOnlyResultType, NSUpdatedObjectIDsResultType,
    NSUpdatedObjectsCountResultType,
};
pub use self::__NSPropertyDescription::NSPropertyDescription;
pub use self::__NSPropertyMapping::NSPropertyMapping;
pub use self::__NSQueryGenerationToken::NSQueryGenerationToken;
pub use self::__NSRelationshipDescription::{
    NSCascadeDeleteRule, NSDeleteRule, NSDenyDeleteRule, NSNoActionDeleteRule, NSNullifyDeleteRule,
    NSRelationshipDescription,
};
pub use self::__NSSaveChangesRequest::NSSaveChangesRequest;
