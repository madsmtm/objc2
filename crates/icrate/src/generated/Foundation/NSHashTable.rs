//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

pub static NSHashTableStrongMemory: NSPointerFunctionsOptions = NSPointerFunctionsStrongMemory;

pub static NSHashTableZeroingWeakMemory: NSPointerFunctionsOptions =
    NSPointerFunctionsZeroingWeakMemory;

pub static NSHashTableCopyIn: NSPointerFunctionsOptions = NSPointerFunctionsCopyIn;

pub static NSHashTableObjectPointerPersonality: NSPointerFunctionsOptions =
    NSPointerFunctionsObjectPointerPersonality;

pub static NSHashTableWeakMemory: NSPointerFunctionsOptions = NSPointerFunctionsWeakMemory;

pub type NSHashTableOptions = NSUInteger;

__inner_extern_class!(
    #[derive(Debug)]
    pub struct NSHashTable<ObjectType: Message = Object> {
        _inner0: PhantomData<*mut ObjectType>,
    }

    unsafe impl<ObjectType: Message> ClassType for NSHashTable<ObjectType> {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl<ObjectType: Message> NSHashTable<ObjectType> {
        #[method_id(initWithOptions:capacity:)]
        pub unsafe fn initWithOptions_capacity(
            &self,
            options: NSPointerFunctionsOptions,
            initialCapacity: NSUInteger,
        ) -> Id<Self, Shared>;

        #[method_id(initWithPointerFunctions:capacity:)]
        pub unsafe fn initWithPointerFunctions_capacity(
            &self,
            functions: &NSPointerFunctions,
            initialCapacity: NSUInteger,
        ) -> Id<Self, Shared>;

        #[method_id(hashTableWithOptions:)]
        pub unsafe fn hashTableWithOptions(
            options: NSPointerFunctionsOptions,
        ) -> Id<NSHashTable<ObjectType>, Shared>;

        #[method_id(hashTableWithWeakObjects)]
        pub unsafe fn hashTableWithWeakObjects() -> Id<Object, Shared>;

        #[method_id(weakObjectsHashTable)]
        pub unsafe fn weakObjectsHashTable() -> Id<NSHashTable<ObjectType>, Shared>;

        #[method_id(pointerFunctions)]
        pub unsafe fn pointerFunctions(&self) -> Id<NSPointerFunctions, Shared>;

        #[method(count)]
        pub unsafe fn count(&self) -> NSUInteger;

        #[method_id(member:)]
        pub unsafe fn member(&self, object: Option<&ObjectType>) -> Option<Id<ObjectType, Shared>>;

        #[method_id(objectEnumerator)]
        pub unsafe fn objectEnumerator(&self) -> Id<NSEnumerator<ObjectType>, Shared>;

        #[method(addObject:)]
        pub unsafe fn addObject(&self, object: Option<&ObjectType>);

        #[method(removeObject:)]
        pub unsafe fn removeObject(&self, object: Option<&ObjectType>);

        #[method(removeAllObjects)]
        pub unsafe fn removeAllObjects(&self);

        #[method_id(allObjects)]
        pub unsafe fn allObjects(&self) -> Id<NSArray<ObjectType>, Shared>;

        #[method_id(anyObject)]
        pub unsafe fn anyObject(&self) -> Option<Id<ObjectType, Shared>>;

        #[method(containsObject:)]
        pub unsafe fn containsObject(&self, anObject: Option<&ObjectType>) -> bool;

        #[method(intersectsHashTable:)]
        pub unsafe fn intersectsHashTable(&self, other: &NSHashTable<ObjectType>) -> bool;

        #[method(isEqualToHashTable:)]
        pub unsafe fn isEqualToHashTable(&self, other: &NSHashTable<ObjectType>) -> bool;

        #[method(isSubsetOfHashTable:)]
        pub unsafe fn isSubsetOfHashTable(&self, other: &NSHashTable<ObjectType>) -> bool;

        #[method(intersectHashTable:)]
        pub unsafe fn intersectHashTable(&self, other: &NSHashTable<ObjectType>);

        #[method(unionHashTable:)]
        pub unsafe fn unionHashTable(&self, other: &NSHashTable<ObjectType>);

        #[method(minusHashTable:)]
        pub unsafe fn minusHashTable(&self, other: &NSHashTable<ObjectType>);

        #[method_id(setRepresentation)]
        pub unsafe fn setRepresentation(&self) -> Id<NSSet<ObjectType>, Shared>;
    }
);

extern "C" {
    pub static NSIntegerHashCallBacks: NSHashTableCallBacks;
}

extern "C" {
    pub static NSNonOwnedPointerHashCallBacks: NSHashTableCallBacks;
}

extern "C" {
    pub static NSNonRetainedObjectHashCallBacks: NSHashTableCallBacks;
}

extern "C" {
    pub static NSObjectHashCallBacks: NSHashTableCallBacks;
}

extern "C" {
    pub static NSOwnedObjectIdentityHashCallBacks: NSHashTableCallBacks;
}

extern "C" {
    pub static NSOwnedPointerHashCallBacks: NSHashTableCallBacks;
}

extern "C" {
    pub static NSPointerToStructHashCallBacks: NSHashTableCallBacks;
}

extern "C" {
    pub static NSIntHashCallBacks: NSHashTableCallBacks;
}
