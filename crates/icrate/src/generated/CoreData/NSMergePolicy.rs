//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_static!(NSErrorMergePolicy: &'static Object);

extern_static!(NSMergeByPropertyStoreTrumpMergePolicy: &'static Object);

extern_static!(NSMergeByPropertyObjectTrumpMergePolicy: &'static Object);

extern_static!(NSOverwriteMergePolicy: &'static Object);

extern_static!(NSRollbackMergePolicy: &'static Object);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSMergePolicyType {
        NSErrorMergePolicyType = 0x00,
        NSMergeByPropertyStoreTrumpMergePolicyType = 0x01,
        NSMergeByPropertyObjectTrumpMergePolicyType = 0x02,
        NSOverwriteMergePolicyType = 0x03,
        NSRollbackMergePolicyType = 0x04,
    }
);

extern_class!(
    #[derive(Debug)]
    pub struct NSMergeConflict;

    unsafe impl ClassType for NSMergeConflict {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl NSMergeConflict {
        #[method_id(@__retain_semantics Other sourceObject)]
        pub unsafe fn sourceObject(&self) -> Id<NSManagedObject, Shared>;

        #[method_id(@__retain_semantics Other objectSnapshot)]
        pub unsafe fn objectSnapshot(&self) -> Option<Id<NSDictionary<NSString, Object>, Shared>>;

        #[method_id(@__retain_semantics Other cachedSnapshot)]
        pub unsafe fn cachedSnapshot(&self) -> Option<Id<NSDictionary<NSString, Object>, Shared>>;

        #[method_id(@__retain_semantics Other persistedSnapshot)]
        pub unsafe fn persistedSnapshot(
            &self,
        ) -> Option<Id<NSDictionary<NSString, Object>, Shared>>;

        #[method(newVersionNumber)]
        pub unsafe fn newVersionNumber(&self) -> NSUInteger;

        #[method(oldVersionNumber)]
        pub unsafe fn oldVersionNumber(&self) -> NSUInteger;

        #[method_id(@__retain_semantics Init initWithSource:newVersion:oldVersion:cachedSnapshot:persistedSnapshot:)]
        pub unsafe fn initWithSource_newVersion_oldVersion_cachedSnapshot_persistedSnapshot(
            this: Option<Allocated<Self>>,
            srcObject: &NSManagedObject,
            newvers: NSUInteger,
            oldvers: NSUInteger,
            cachesnap: Option<&NSDictionary<NSString, Object>>,
            persnap: Option<&NSDictionary<NSString, Object>>,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;
    }
);

extern_class!(
    #[derive(Debug)]
    pub struct NSConstraintConflict;

    unsafe impl ClassType for NSConstraintConflict {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl NSConstraintConflict {
        #[method_id(@__retain_semantics Other constraint)]
        pub unsafe fn constraint(&self) -> Id<NSArray<NSString>, Shared>;

        #[method_id(@__retain_semantics Other constraintValues)]
        pub unsafe fn constraintValues(&self) -> Id<NSDictionary<NSString, Object>, Shared>;

        #[method_id(@__retain_semantics Other databaseObject)]
        pub unsafe fn databaseObject(&self) -> Option<Id<NSManagedObject, Shared>>;

        #[method_id(@__retain_semantics Other databaseSnapshot)]
        pub unsafe fn databaseSnapshot(&self)
            -> Option<Id<NSDictionary<NSString, Object>, Shared>>;

        #[method_id(@__retain_semantics Other conflictingObjects)]
        pub unsafe fn conflictingObjects(&self) -> Id<NSArray<NSManagedObject>, Shared>;

        #[method_id(@__retain_semantics Other conflictingSnapshots)]
        pub unsafe fn conflictingSnapshots(&self) -> Id<NSArray<NSDictionary>, Shared>;

        #[method_id(@__retain_semantics Init initWithConstraint:databaseObject:databaseSnapshot:conflictingObjects:conflictingSnapshots:)]
        pub unsafe fn initWithConstraint_databaseObject_databaseSnapshot_conflictingObjects_conflictingSnapshots(
            this: Option<Allocated<Self>>,
            contraint: &NSArray<NSString>,
            databaseObject: Option<&NSManagedObject>,
            databaseSnapshot: Option<&NSDictionary>,
            conflictingObjects: &NSArray<NSManagedObject>,
            conflictingSnapshots: &NSArray,
        ) -> Id<Self, Shared>;
    }
);

extern_class!(
    #[derive(Debug)]
    pub struct NSMergePolicy;

    unsafe impl ClassType for NSMergePolicy {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl NSMergePolicy {
        #[method_id(@__retain_semantics Other errorMergePolicy)]
        pub unsafe fn errorMergePolicy() -> Id<NSMergePolicy, Shared>;

        #[method_id(@__retain_semantics Other rollbackMergePolicy)]
        pub unsafe fn rollbackMergePolicy() -> Id<NSMergePolicy, Shared>;

        #[method_id(@__retain_semantics Other overwriteMergePolicy)]
        pub unsafe fn overwriteMergePolicy() -> Id<NSMergePolicy, Shared>;

        #[method_id(@__retain_semantics Other mergeByPropertyObjectTrumpMergePolicy)]
        pub unsafe fn mergeByPropertyObjectTrumpMergePolicy() -> Id<NSMergePolicy, Shared>;

        #[method_id(@__retain_semantics Other mergeByPropertyStoreTrumpMergePolicy)]
        pub unsafe fn mergeByPropertyStoreTrumpMergePolicy() -> Id<NSMergePolicy, Shared>;

        #[method(mergeType)]
        pub unsafe fn mergeType(&self) -> NSMergePolicyType;

        #[method_id(@__retain_semantics Init initWithMergeType:)]
        pub unsafe fn initWithMergeType(
            this: Option<Allocated<Self>>,
            ty: NSMergePolicyType,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;

        #[method(resolveConflicts:error:)]
        pub unsafe fn resolveConflicts_error(
            &self,
            list: &NSArray,
        ) -> Result<(), Id<NSError, Shared>>;

        #[method(resolveOptimisticLockingVersionConflicts:error:)]
        pub unsafe fn resolveOptimisticLockingVersionConflicts_error(
            &self,
            list: &NSArray<NSMergeConflict>,
        ) -> Result<(), Id<NSError, Shared>>;

        #[method(resolveConstraintConflicts:error:)]
        pub unsafe fn resolveConstraintConflicts_error(
            &self,
            list: &NSArray<NSConstraintConflict>,
        ) -> Result<(), Id<NSError, Shared>>;
    }
);
