//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreData::*;
use crate::Foundation::*;

pub type NSBatchInsertRequestResultType = NSUInteger;
pub const NSBatchInsertRequestResultTypeStatusOnly: NSBatchInsertRequestResultType = 0x0;
pub const NSBatchInsertRequestResultTypeObjectIDs: NSBatchInsertRequestResultType = 0x1;
pub const NSBatchInsertRequestResultTypeCount: NSBatchInsertRequestResultType = 0x2;

pub type NSBatchUpdateRequestResultType = NSUInteger;
pub const NSStatusOnlyResultType: NSBatchUpdateRequestResultType = 0x0;
pub const NSUpdatedObjectIDsResultType: NSBatchUpdateRequestResultType = 0x1;
pub const NSUpdatedObjectsCountResultType: NSBatchUpdateRequestResultType = 0x2;

pub type NSBatchDeleteRequestResultType = NSUInteger;
pub const NSBatchDeleteResultTypeStatusOnly: NSBatchDeleteRequestResultType = 0x0;
pub const NSBatchDeleteResultTypeObjectIDs: NSBatchDeleteRequestResultType = 0x1;
pub const NSBatchDeleteResultTypeCount: NSBatchDeleteRequestResultType = 0x2;

pub type NSPersistentHistoryResultType = NSInteger;
pub const NSPersistentHistoryResultTypeStatusOnly: NSPersistentHistoryResultType = 0x0;
pub const NSPersistentHistoryResultTypeObjectIDs: NSPersistentHistoryResultType = 0x1;
pub const NSPersistentHistoryResultTypeCount: NSPersistentHistoryResultType = 0x2;
pub const NSPersistentHistoryResultTypeTransactionsOnly: NSPersistentHistoryResultType = 0x3;
pub const NSPersistentHistoryResultTypeChangesOnly: NSPersistentHistoryResultType = 0x4;
pub const NSPersistentHistoryResultTypeTransactionsAndChanges: NSPersistentHistoryResultType = 0x5;

extern_class!(
    #[derive(Debug)]
    pub struct NSPersistentStoreResult;

    unsafe impl ClassType for NSPersistentStoreResult {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl NSPersistentStoreResult {}
);

extern_class!(
    #[derive(Debug)]
    pub struct NSPersistentStoreAsynchronousResult;

    unsafe impl ClassType for NSPersistentStoreAsynchronousResult {
        type Super = NSPersistentStoreResult;
    }
);

extern_methods!(
    unsafe impl NSPersistentStoreAsynchronousResult {
        #[method_id(@__retain_semantics Other managedObjectContext)]
        pub unsafe fn managedObjectContext(&self) -> Id<NSManagedObjectContext, Shared>;

        #[method_id(@__retain_semantics Other operationError)]
        pub unsafe fn operationError(&self) -> Option<Id<NSError, Shared>>;

        #[method_id(@__retain_semantics Other progress)]
        pub unsafe fn progress(&self) -> Option<Id<NSProgress, Shared>>;

        #[method(cancel)]
        pub unsafe fn cancel(&self);
    }
);

__inner_extern_class!(
    #[derive(Debug)]
    pub struct NSAsynchronousFetchResult<ResultType: Message = Object> {
        _inner0: PhantomData<*mut ResultType>,
    }

    unsafe impl<ResultType: Message> ClassType for NSAsynchronousFetchResult<ResultType> {
        type Super = NSPersistentStoreAsynchronousResult;
    }
);

extern_methods!(
    unsafe impl<ResultType: Message> NSAsynchronousFetchResult<ResultType> {
        #[method_id(@__retain_semantics Other fetchRequest)]
        pub unsafe fn fetchRequest(&self) -> Id<NSAsynchronousFetchRequest<ResultType>, Shared>;

        #[method_id(@__retain_semantics Other finalResult)]
        pub unsafe fn finalResult(&self) -> Option<Id<NSArray<ResultType>, Shared>>;
    }
);

extern_class!(
    #[derive(Debug)]
    pub struct NSBatchInsertResult;

    unsafe impl ClassType for NSBatchInsertResult {
        type Super = NSPersistentStoreResult;
    }
);

extern_methods!(
    unsafe impl NSBatchInsertResult {
        #[method_id(@__retain_semantics Other result)]
        pub unsafe fn result(&self) -> Option<Id<Object, Shared>>;

        #[method(resultType)]
        pub unsafe fn resultType(&self) -> NSBatchInsertRequestResultType;
    }
);

extern_class!(
    #[derive(Debug)]
    pub struct NSBatchUpdateResult;

    unsafe impl ClassType for NSBatchUpdateResult {
        type Super = NSPersistentStoreResult;
    }
);

extern_methods!(
    unsafe impl NSBatchUpdateResult {
        #[method_id(@__retain_semantics Other result)]
        pub unsafe fn result(&self) -> Option<Id<Object, Shared>>;

        #[method(resultType)]
        pub unsafe fn resultType(&self) -> NSBatchUpdateRequestResultType;
    }
);

extern_class!(
    #[derive(Debug)]
    pub struct NSBatchDeleteResult;

    unsafe impl ClassType for NSBatchDeleteResult {
        type Super = NSPersistentStoreResult;
    }
);

extern_methods!(
    unsafe impl NSBatchDeleteResult {
        #[method_id(@__retain_semantics Other result)]
        pub unsafe fn result(&self) -> Option<Id<Object, Shared>>;

        #[method(resultType)]
        pub unsafe fn resultType(&self) -> NSBatchDeleteRequestResultType;
    }
);

extern_class!(
    #[derive(Debug)]
    pub struct NSPersistentHistoryResult;

    unsafe impl ClassType for NSPersistentHistoryResult {
        type Super = NSPersistentStoreResult;
    }
);

extern_methods!(
    unsafe impl NSPersistentHistoryResult {
        #[method_id(@__retain_semantics Other result)]
        pub unsafe fn result(&self) -> Option<Id<Object, Shared>>;

        #[method(resultType)]
        pub unsafe fn resultType(&self) -> NSPersistentHistoryResultType;
    }
);

pub type NSPersistentCloudKitContainerEventResultType = NSInteger;
pub const NSPersistentCloudKitContainerEventResultTypeEvents:
    NSPersistentCloudKitContainerEventResultType = 0;
pub const NSPersistentCloudKitContainerEventResultTypeCountEvents:
    NSPersistentCloudKitContainerEventResultType = 1;

extern_class!(
    #[derive(Debug)]
    pub struct NSPersistentCloudKitContainerEventResult;

    unsafe impl ClassType for NSPersistentCloudKitContainerEventResult {
        type Super = NSPersistentStoreResult;
    }
);

extern_methods!(
    unsafe impl NSPersistentCloudKitContainerEventResult {
        #[method_id(@__retain_semantics Other result)]
        pub unsafe fn result(&self) -> Option<Id<Object, Shared>>;

        #[method(resultType)]
        pub unsafe fn resultType(&self) -> NSPersistentCloudKitContainerEventResultType;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self, Shared>;
    }
);
