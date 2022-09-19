use super::__exported::NSArray;
use super::__exported::NSDictionary;
use crate::Foundation::generated::NSEnumerator::*;
use crate::Foundation::generated::NSPointerFunctions::*;
use crate::Foundation::generated::NSString::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, msg_send, msg_send_id, ClassType};
__inner_extern_class!(
    #[derive(Debug)]
    pub struct NSMapTable<KeyType: Message, ObjectType: Message>;
    unsafe impl<KeyType: Message, ObjectType: Message> ClassType for NSMapTable<KeyType, ObjectType> {
        type Super = NSObject;
    }
);
impl<KeyType: Message, ObjectType: Message> NSMapTable<KeyType, ObjectType> {
    pub unsafe fn initWithKeyOptions_valueOptions_capacity(
        &self,
        keyOptions: NSPointerFunctionsOptions,
        valueOptions: NSPointerFunctionsOptions,
        initialCapacity: NSUInteger,
    ) -> Id<Self, Shared> {
        msg_send_id![
            self,
            initWithKeyOptions: keyOptions,
            valueOptions: valueOptions,
            capacity: initialCapacity
        ]
    }
    pub unsafe fn initWithKeyPointerFunctions_valuePointerFunctions_capacity(
        &self,
        keyFunctions: &NSPointerFunctions,
        valueFunctions: &NSPointerFunctions,
        initialCapacity: NSUInteger,
    ) -> Id<Self, Shared> {
        msg_send_id![
            self,
            initWithKeyPointerFunctions: keyFunctions,
            valuePointerFunctions: valueFunctions,
            capacity: initialCapacity
        ]
    }
    pub unsafe fn mapTableWithKeyOptions_valueOptions(
        keyOptions: NSPointerFunctionsOptions,
        valueOptions: NSPointerFunctionsOptions,
    ) -> TodoGenerics {
        msg_send![
            Self::class(),
            mapTableWithKeyOptions: keyOptions,
            valueOptions: valueOptions
        ]
    }
    pub unsafe fn mapTableWithStrongToStrongObjects() -> Id<Object, Shared> {
        msg_send_id![Self::class(), mapTableWithStrongToStrongObjects]
    }
    pub unsafe fn mapTableWithWeakToStrongObjects() -> Id<Object, Shared> {
        msg_send_id![Self::class(), mapTableWithWeakToStrongObjects]
    }
    pub unsafe fn mapTableWithStrongToWeakObjects() -> Id<Object, Shared> {
        msg_send_id![Self::class(), mapTableWithStrongToWeakObjects]
    }
    pub unsafe fn mapTableWithWeakToWeakObjects() -> Id<Object, Shared> {
        msg_send_id![Self::class(), mapTableWithWeakToWeakObjects]
    }
    pub unsafe fn strongToStrongObjectsMapTable() -> TodoGenerics {
        msg_send![Self::class(), strongToStrongObjectsMapTable]
    }
    pub unsafe fn weakToStrongObjectsMapTable() -> TodoGenerics {
        msg_send![Self::class(), weakToStrongObjectsMapTable]
    }
    pub unsafe fn strongToWeakObjectsMapTable() -> TodoGenerics {
        msg_send![Self::class(), strongToWeakObjectsMapTable]
    }
    pub unsafe fn weakToWeakObjectsMapTable() -> TodoGenerics {
        msg_send![Self::class(), weakToWeakObjectsMapTable]
    }
    pub unsafe fn objectForKey(&self, aKey: Option<&KeyType>) -> Option<Id<ObjectType, Shared>> {
        msg_send_id![self, objectForKey: aKey]
    }
    pub unsafe fn removeObjectForKey(&self, aKey: Option<&KeyType>) {
        msg_send![self, removeObjectForKey: aKey]
    }
    pub unsafe fn setObject_forKey(&self, anObject: Option<&ObjectType>, aKey: Option<&KeyType>) {
        msg_send![self, setObject: anObject, forKey: aKey]
    }
    pub unsafe fn keyEnumerator(&self) -> TodoGenerics {
        msg_send![self, keyEnumerator]
    }
    pub unsafe fn objectEnumerator(&self) -> TodoGenerics {
        msg_send![self, objectEnumerator]
    }
    pub unsafe fn removeAllObjects(&self) {
        msg_send![self, removeAllObjects]
    }
    pub unsafe fn dictionaryRepresentation(&self) -> TodoGenerics {
        msg_send![self, dictionaryRepresentation]
    }
    pub unsafe fn keyPointerFunctions(&self) -> Id<NSPointerFunctions, Shared> {
        msg_send_id![self, keyPointerFunctions]
    }
    pub unsafe fn valuePointerFunctions(&self) -> Id<NSPointerFunctions, Shared> {
        msg_send_id![self, valuePointerFunctions]
    }
    pub unsafe fn count(&self) -> NSUInteger {
        msg_send![self, count]
    }
}
