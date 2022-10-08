use super::__exported::NSArray;
use super::__exported::NSDictionary;
use crate::Foundation::generated::NSEnumerator::*;
use crate::Foundation::generated::NSPointerFunctions::*;
use crate::Foundation::generated::NSString::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
pub type NSMapTableOptions = NSUInteger;
__inner_extern_class!(
    #[derive(Debug)]
    pub struct NSMapTable<KeyType: Message, ObjectType: Message>;
    unsafe impl<KeyType: Message, ObjectType: Message> ClassType for NSMapTable<KeyType, ObjectType> {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl<KeyType: Message, ObjectType: Message> NSMapTable<KeyType, ObjectType> {
        # [method_id (initWithKeyOptions : valueOptions : capacity :)]
        pub unsafe fn initWithKeyOptions_valueOptions_capacity(
            &self,
            keyOptions: NSPointerFunctionsOptions,
            valueOptions: NSPointerFunctionsOptions,
            initialCapacity: NSUInteger,
        ) -> Id<Self, Shared>;
        # [method_id (initWithKeyPointerFunctions : valuePointerFunctions : capacity :)]
        pub unsafe fn initWithKeyPointerFunctions_valuePointerFunctions_capacity(
            &self,
            keyFunctions: &NSPointerFunctions,
            valueFunctions: &NSPointerFunctions,
            initialCapacity: NSUInteger,
        ) -> Id<Self, Shared>;
        # [method_id (mapTableWithKeyOptions : valueOptions :)]
        pub unsafe fn mapTableWithKeyOptions_valueOptions(
            keyOptions: NSPointerFunctionsOptions,
            valueOptions: NSPointerFunctionsOptions,
        ) -> Id<NSMapTable<KeyType, ObjectType>, Shared>;
        #[method_id(mapTableWithStrongToStrongObjects)]
        pub unsafe fn mapTableWithStrongToStrongObjects() -> Id<Object, Shared>;
        #[method_id(mapTableWithWeakToStrongObjects)]
        pub unsafe fn mapTableWithWeakToStrongObjects() -> Id<Object, Shared>;
        #[method_id(mapTableWithStrongToWeakObjects)]
        pub unsafe fn mapTableWithStrongToWeakObjects() -> Id<Object, Shared>;
        #[method_id(mapTableWithWeakToWeakObjects)]
        pub unsafe fn mapTableWithWeakToWeakObjects() -> Id<Object, Shared>;
        #[method_id(strongToStrongObjectsMapTable)]
        pub unsafe fn strongToStrongObjectsMapTable() -> Id<NSMapTable<KeyType, ObjectType>, Shared>;
        #[method_id(weakToStrongObjectsMapTable)]
        pub unsafe fn weakToStrongObjectsMapTable() -> Id<NSMapTable<KeyType, ObjectType>, Shared>;
        #[method_id(strongToWeakObjectsMapTable)]
        pub unsafe fn strongToWeakObjectsMapTable() -> Id<NSMapTable<KeyType, ObjectType>, Shared>;
        #[method_id(weakToWeakObjectsMapTable)]
        pub unsafe fn weakToWeakObjectsMapTable() -> Id<NSMapTable<KeyType, ObjectType>, Shared>;
        #[method_id(keyPointerFunctions)]
        pub unsafe fn keyPointerFunctions(&self) -> Id<NSPointerFunctions, Shared>;
        #[method_id(valuePointerFunctions)]
        pub unsafe fn valuePointerFunctions(&self) -> Id<NSPointerFunctions, Shared>;
        # [method_id (objectForKey :)]
        pub unsafe fn objectForKey(&self, aKey: Option<&KeyType>)
            -> Option<Id<ObjectType, Shared>>;
        # [method (removeObjectForKey :)]
        pub unsafe fn removeObjectForKey(&self, aKey: Option<&KeyType>);
        # [method (setObject : forKey :)]
        pub unsafe fn setObject_forKey(
            &self,
            anObject: Option<&ObjectType>,
            aKey: Option<&KeyType>,
        );
        #[method(count)]
        pub unsafe fn count(&self) -> NSUInteger;
        #[method_id(keyEnumerator)]
        pub unsafe fn keyEnumerator(&self) -> Id<NSEnumerator<KeyType>, Shared>;
        #[method_id(objectEnumerator)]
        pub unsafe fn objectEnumerator(&self) -> Option<Id<NSEnumerator<ObjectType>, Shared>>;
        #[method(removeAllObjects)]
        pub unsafe fn removeAllObjects(&self);
        #[method_id(dictionaryRepresentation)]
        pub unsafe fn dictionaryRepresentation(
            &self,
        ) -> Id<NSDictionary<KeyType, ObjectType>, Shared>;
    }
);
