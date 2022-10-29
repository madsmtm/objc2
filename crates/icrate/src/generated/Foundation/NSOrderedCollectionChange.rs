#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
__inner_extern_class!(
    #[derive(Debug)]
    pub struct NSOrderedCollectionChange<ObjectType: Message>;
    unsafe impl<ObjectType: Message> ClassType for NSOrderedCollectionChange<ObjectType> {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl<ObjectType: Message> NSOrderedCollectionChange<ObjectType> {
        #[method_id(changeWithObject:type:index:)]
        pub unsafe fn changeWithObject_type_index(
            anObject: Option<&ObjectType>,
            type_: NSCollectionChangeType,
            index: NSUInteger,
        ) -> Id<NSOrderedCollectionChange<ObjectType>, Shared>;
        #[method_id(changeWithObject:type:index:associatedIndex:)]
        pub unsafe fn changeWithObject_type_index_associatedIndex(
            anObject: Option<&ObjectType>,
            type_: NSCollectionChangeType,
            index: NSUInteger,
            associatedIndex: NSUInteger,
        ) -> Id<NSOrderedCollectionChange<ObjectType>, Shared>;
        #[method_id(object)]
        pub unsafe fn object(&self) -> Option<Id<ObjectType, Shared>>;
        #[method(changeType)]
        pub unsafe fn changeType(&self) -> NSCollectionChangeType;
        #[method(index)]
        pub unsafe fn index(&self) -> NSUInteger;
        #[method(associatedIndex)]
        pub unsafe fn associatedIndex(&self) -> NSUInteger;
        #[method_id(init)]
        pub unsafe fn init(&self) -> Id<Object, Shared>;
        #[method_id(initWithObject:type:index:)]
        pub unsafe fn initWithObject_type_index(
            &self,
            anObject: Option<&ObjectType>,
            type_: NSCollectionChangeType,
            index: NSUInteger,
        ) -> Id<Self, Shared>;
        #[method_id(initWithObject:type:index:associatedIndex:)]
        pub unsafe fn initWithObject_type_index_associatedIndex(
            &self,
            anObject: Option<&ObjectType>,
            type_: NSCollectionChangeType,
            index: NSUInteger,
            associatedIndex: NSUInteger,
        ) -> Id<Self, Shared>;
    }
);
