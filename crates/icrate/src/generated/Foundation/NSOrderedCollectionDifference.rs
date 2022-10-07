use super::__exported::NSArray;
use crate::Foundation::generated::NSEnumerator::*;
use crate::Foundation::generated::NSIndexSet::*;
use crate::Foundation::generated::NSOrderedCollectionChange::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, msg_send, msg_send_id, ClassType};
__inner_extern_class!(
    #[derive(Debug)]
    pub struct NSOrderedCollectionDifference<ObjectType: Message>;
    unsafe impl<ObjectType: Message> ClassType for NSOrderedCollectionDifference<ObjectType> {
        type Super = NSObject;
    }
);
impl<ObjectType: Message> NSOrderedCollectionDifference<ObjectType> {
    pub unsafe fn initWithChanges(
        &self,
        changes: &NSArray<NSOrderedCollectionChange<ObjectType>>,
    ) -> Id<Self, Shared> {
        msg_send_id![self, initWithChanges: changes]
    }
    pub unsafe fn initWithInsertIndexes_insertedObjects_removeIndexes_removedObjects_additionalChanges(
        &self,
        inserts: &NSIndexSet,
        insertedObjects: Option<&NSArray<ObjectType>>,
        removes: &NSIndexSet,
        removedObjects: Option<&NSArray<ObjectType>>,
        changes: &NSArray<NSOrderedCollectionChange<ObjectType>>,
    ) -> Id<Self, Shared> {
        msg_send_id![
            self,
            initWithInsertIndexes: inserts,
            insertedObjects: insertedObjects,
            removeIndexes: removes,
            removedObjects: removedObjects,
            additionalChanges: changes
        ]
    }
    pub unsafe fn initWithInsertIndexes_insertedObjects_removeIndexes_removedObjects(
        &self,
        inserts: &NSIndexSet,
        insertedObjects: Option<&NSArray<ObjectType>>,
        removes: &NSIndexSet,
        removedObjects: Option<&NSArray<ObjectType>>,
    ) -> Id<Self, Shared> {
        msg_send_id![
            self,
            initWithInsertIndexes: inserts,
            insertedObjects: insertedObjects,
            removeIndexes: removes,
            removedObjects: removedObjects
        ]
    }
    pub unsafe fn insertions(&self) -> Id<NSArray<NSOrderedCollectionChange<ObjectType>>, Shared> {
        msg_send_id![self, insertions]
    }
    pub unsafe fn removals(&self) -> Id<NSArray<NSOrderedCollectionChange<ObjectType>>, Shared> {
        msg_send_id![self, removals]
    }
    pub unsafe fn hasChanges(&self) -> bool {
        msg_send![self, hasChanges]
    }
    pub unsafe fn differenceByTransformingChangesWithBlock(
        &self,
        block: TodoBlock,
    ) -> Id<NSOrderedCollectionDifference<Object>, Shared> {
        msg_send_id![self, differenceByTransformingChangesWithBlock: block]
    }
    pub unsafe fn inverseDifference(&self) -> Id<Self, Shared> {
        msg_send_id![self, inverseDifference]
    }
}
