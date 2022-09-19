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
    pub unsafe fn initWithChanges(&self, changes: TodoGenerics) -> Id<Self, Shared> {
        msg_send_id![self, initWithChanges: changes]
    }
    pub unsafe fn initWithInsertIndexes_insertedObjects_removeIndexes_removedObjects_additionalChanges(
        &self,
        inserts: &NSIndexSet,
        insertedObjects: TodoGenerics,
        removes: &NSIndexSet,
        removedObjects: TodoGenerics,
        changes: TodoGenerics,
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
        insertedObjects: TodoGenerics,
        removes: &NSIndexSet,
        removedObjects: TodoGenerics,
    ) -> Id<Self, Shared> {
        msg_send_id![
            self,
            initWithInsertIndexes: inserts,
            insertedObjects: insertedObjects,
            removeIndexes: removes,
            removedObjects: removedObjects
        ]
    }
    pub unsafe fn differenceByTransformingChangesWithBlock(
        &self,
        block: TodoBlock,
    ) -> TodoGenerics {
        msg_send![self, differenceByTransformingChangesWithBlock: block]
    }
    pub unsafe fn inverseDifference(&self) -> Id<Self, Shared> {
        msg_send_id![self, inverseDifference]
    }
    pub unsafe fn insertions(&self) -> TodoGenerics {
        msg_send![self, insertions]
    }
    pub unsafe fn removals(&self) -> TodoGenerics {
        msg_send![self, removals]
    }
    pub unsafe fn hasChanges(&self) -> bool {
        msg_send![self, hasChanges]
    }
}
