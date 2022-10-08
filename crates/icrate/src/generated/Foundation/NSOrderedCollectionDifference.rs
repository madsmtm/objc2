use super::__exported::NSArray;
use crate::Foundation::generated::NSEnumerator::*;
use crate::Foundation::generated::NSIndexSet::*;
use crate::Foundation::generated::NSOrderedCollectionChange::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
__inner_extern_class!(
    #[derive(Debug)]
    pub struct NSOrderedCollectionDifference<ObjectType: Message>;
    unsafe impl<ObjectType: Message> ClassType for NSOrderedCollectionDifference<ObjectType> {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl<ObjectType: Message> NSOrderedCollectionDifference<ObjectType> {
        # [method_id (initWithChanges :)]
        pub unsafe fn initWithChanges(
            &self,
            changes: &NSArray<NSOrderedCollectionChange<ObjectType>>,
        ) -> Id<Self, Shared>;
        # [method_id (initWithInsertIndexes : insertedObjects : removeIndexes : removedObjects : additionalChanges :)]
        pub unsafe fn initWithInsertIndexes_insertedObjects_removeIndexes_removedObjects_additionalChanges(
            &self,
            inserts: &NSIndexSet,
            insertedObjects: Option<&NSArray<ObjectType>>,
            removes: &NSIndexSet,
            removedObjects: Option<&NSArray<ObjectType>>,
            changes: &NSArray<NSOrderedCollectionChange<ObjectType>>,
        ) -> Id<Self, Shared>;
        # [method_id (initWithInsertIndexes : insertedObjects : removeIndexes : removedObjects :)]
        pub unsafe fn initWithInsertIndexes_insertedObjects_removeIndexes_removedObjects(
            &self,
            inserts: &NSIndexSet,
            insertedObjects: Option<&NSArray<ObjectType>>,
            removes: &NSIndexSet,
            removedObjects: Option<&NSArray<ObjectType>>,
        ) -> Id<Self, Shared>;
        #[method_id(insertions)]
        pub unsafe fn insertions(
            &self,
        ) -> Id<NSArray<NSOrderedCollectionChange<ObjectType>>, Shared>;
        #[method_id(removals)]
        pub unsafe fn removals(&self)
            -> Id<NSArray<NSOrderedCollectionChange<ObjectType>>, Shared>;
        #[method(hasChanges)]
        pub unsafe fn hasChanges(&self) -> bool;
        # [method_id (differenceByTransformingChangesWithBlock :)]
        pub unsafe fn differenceByTransformingChangesWithBlock(
            &self,
            block: TodoBlock,
        ) -> Id<NSOrderedCollectionDifference<Object>, Shared>;
        #[method_id(inverseDifference)]
        pub unsafe fn inverseDifference(&self) -> Id<Self, Shared>;
    }
);
