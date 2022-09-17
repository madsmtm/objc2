use super::__exported::NSArray;
use super::__exported::NSSet;
use crate::Foundation::generated::NSEnumerator::*;
use crate::Foundation::generated::NSPointerFunctions::*;
use crate::Foundation::generated::NSString::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, msg_send, msg_send_id, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSHashTable;
    unsafe impl ClassType for NSHashTable {
        type Super = NSObject;
    }
);
impl NSHashTable {
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
    pub unsafe fn hashTableWithOptions(options: NSPointerFunctionsOptions) -> TodoGenerics {
        msg_send![Self::class(), hashTableWithOptions: options]
    }
    pub unsafe fn hashTableWithWeakObjects() -> Id<Object, Shared> {
        msg_send_id![Self::class(), hashTableWithWeakObjects]
    }
    pub unsafe fn weakObjectsHashTable() -> TodoGenerics {
        msg_send![Self::class(), weakObjectsHashTable]
    }
    pub unsafe fn member(&self, object: ObjectType) -> ObjectType {
        msg_send![self, member: object]
    }
    pub unsafe fn objectEnumerator(&self) -> TodoGenerics {
        msg_send![self, objectEnumerator]
    }
    pub unsafe fn addObject(&self, object: ObjectType) {
        msg_send![self, addObject: object]
    }
    pub unsafe fn removeObject(&self, object: ObjectType) {
        msg_send![self, removeObject: object]
    }
    pub unsafe fn removeAllObjects(&self) {
        msg_send![self, removeAllObjects]
    }
    pub unsafe fn containsObject(&self, anObject: ObjectType) -> bool {
        msg_send![self, containsObject: anObject]
    }
    pub unsafe fn intersectsHashTable(&self, other: TodoGenerics) -> bool {
        msg_send![self, intersectsHashTable: other]
    }
    pub unsafe fn isEqualToHashTable(&self, other: TodoGenerics) -> bool {
        msg_send![self, isEqualToHashTable: other]
    }
    pub unsafe fn isSubsetOfHashTable(&self, other: TodoGenerics) -> bool {
        msg_send![self, isSubsetOfHashTable: other]
    }
    pub unsafe fn intersectHashTable(&self, other: TodoGenerics) {
        msg_send![self, intersectHashTable: other]
    }
    pub unsafe fn unionHashTable(&self, other: TodoGenerics) {
        msg_send![self, unionHashTable: other]
    }
    pub unsafe fn minusHashTable(&self, other: TodoGenerics) {
        msg_send![self, minusHashTable: other]
    }
    pub unsafe fn pointerFunctions(&self) -> Id<NSPointerFunctions, Shared> {
        msg_send_id![self, pointerFunctions]
    }
    pub unsafe fn count(&self) -> NSUInteger {
        msg_send![self, count]
    }
    pub unsafe fn allObjects(&self) -> TodoGenerics {
        msg_send![self, allObjects]
    }
    pub unsafe fn anyObject(&self) -> ObjectType {
        msg_send![self, anyObject]
    }
    pub unsafe fn setRepresentation(&self) -> TodoGenerics {
        msg_send![self, setRepresentation]
    }
}
