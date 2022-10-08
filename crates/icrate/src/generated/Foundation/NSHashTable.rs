use super::__exported::NSArray;
use super::__exported::NSSet;
use crate::Foundation::generated::NSEnumerator::*;
use crate::Foundation::generated::NSPointerFunctions::*;
use crate::Foundation::generated::NSString::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, msg_send, msg_send_id, ClassType};
pub type NSHashTableOptions = NSUInteger;
__inner_extern_class!(
    #[derive(Debug)]
    pub struct NSHashTable<ObjectType: Message>;
    unsafe impl<ObjectType: Message> ClassType for NSHashTable<ObjectType> {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl<ObjectType: Message> NSHashTable<ObjectType> {
        pub unsafe fn initWithOptions_capacity(
            &self,
            options: NSPointerFunctionsOptions,
            initialCapacity: NSUInteger,
        ) -> Id<Self, Shared> {
            msg_send_id![self, initWithOptions: options, capacity: initialCapacity]
        }
        pub unsafe fn initWithPointerFunctions_capacity(
            &self,
            functions: &NSPointerFunctions,
            initialCapacity: NSUInteger,
        ) -> Id<Self, Shared> {
            msg_send_id![
                self,
                initWithPointerFunctions: functions,
                capacity: initialCapacity
            ]
        }
        pub unsafe fn hashTableWithOptions(
            options: NSPointerFunctionsOptions,
        ) -> Id<NSHashTable<ObjectType>, Shared> {
            msg_send_id![Self::class(), hashTableWithOptions: options]
        }
        pub unsafe fn hashTableWithWeakObjects() -> Id<Object, Shared> {
            msg_send_id![Self::class(), hashTableWithWeakObjects]
        }
        pub unsafe fn weakObjectsHashTable() -> Id<NSHashTable<ObjectType>, Shared> {
            msg_send_id![Self::class(), weakObjectsHashTable]
        }
        pub unsafe fn pointerFunctions(&self) -> Id<NSPointerFunctions, Shared> {
            msg_send_id![self, pointerFunctions]
        }
        pub unsafe fn count(&self) -> NSUInteger {
            msg_send![self, count]
        }
        pub unsafe fn member(&self, object: Option<&ObjectType>) -> Option<Id<ObjectType, Shared>> {
            msg_send_id![self, member: object]
        }
        pub unsafe fn objectEnumerator(&self) -> Id<NSEnumerator<ObjectType>, Shared> {
            msg_send_id![self, objectEnumerator]
        }
        pub unsafe fn addObject(&self, object: Option<&ObjectType>) {
            msg_send![self, addObject: object]
        }
        pub unsafe fn removeObject(&self, object: Option<&ObjectType>) {
            msg_send![self, removeObject: object]
        }
        pub unsafe fn removeAllObjects(&self) {
            msg_send![self, removeAllObjects]
        }
        pub unsafe fn allObjects(&self) -> Id<NSArray<ObjectType>, Shared> {
            msg_send_id![self, allObjects]
        }
        pub unsafe fn anyObject(&self) -> Option<Id<ObjectType, Shared>> {
            msg_send_id![self, anyObject]
        }
        pub unsafe fn containsObject(&self, anObject: Option<&ObjectType>) -> bool {
            msg_send![self, containsObject: anObject]
        }
        pub unsafe fn intersectsHashTable(&self, other: &NSHashTable<ObjectType>) -> bool {
            msg_send![self, intersectsHashTable: other]
        }
        pub unsafe fn isEqualToHashTable(&self, other: &NSHashTable<ObjectType>) -> bool {
            msg_send![self, isEqualToHashTable: other]
        }
        pub unsafe fn isSubsetOfHashTable(&self, other: &NSHashTable<ObjectType>) -> bool {
            msg_send![self, isSubsetOfHashTable: other]
        }
        pub unsafe fn intersectHashTable(&self, other: &NSHashTable<ObjectType>) {
            msg_send![self, intersectHashTable: other]
        }
        pub unsafe fn unionHashTable(&self, other: &NSHashTable<ObjectType>) {
            msg_send![self, unionHashTable: other]
        }
        pub unsafe fn minusHashTable(&self, other: &NSHashTable<ObjectType>) {
            msg_send![self, minusHashTable: other]
        }
        pub unsafe fn setRepresentation(&self) -> Id<NSSet<ObjectType>, Shared> {
            msg_send_id![self, setRepresentation]
        }
    }
);
