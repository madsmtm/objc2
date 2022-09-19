use crate::Foundation::generated::NSObject::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, msg_send, msg_send_id, ClassType};
__inner_extern_class!(
    #[derive(Debug)]
    pub struct NSOrderedCollectionChange<ObjectType: Message>;
    unsafe impl<ObjectType: Message> ClassType for NSOrderedCollectionChange<ObjectType> {
        type Super = NSObject;
    }
);
impl<ObjectType: Message> NSOrderedCollectionChange<ObjectType> {
    pub unsafe fn changeWithObject_type_index(
        anObject: Option<&ObjectType>,
        type_: NSCollectionChangeType,
        index: NSUInteger,
    ) -> TodoGenerics {
        msg_send ! [Self :: class () , changeWithObject : anObject , type : type_ , index : index]
    }
    pub unsafe fn changeWithObject_type_index_associatedIndex(
        anObject: Option<&ObjectType>,
        type_: NSCollectionChangeType,
        index: NSUInteger,
        associatedIndex: NSUInteger,
    ) -> TodoGenerics {
        msg_send ! [Self :: class () , changeWithObject : anObject , type : type_ , index : index , associatedIndex : associatedIndex]
    }
    pub unsafe fn init(&self) -> Id<Object, Shared> {
        msg_send_id![self, init]
    }
    pub unsafe fn initWithObject_type_index(
        &self,
        anObject: Option<&ObjectType>,
        type_: NSCollectionChangeType,
        index: NSUInteger,
    ) -> Id<Self, Shared> {
        msg_send_id ! [self , initWithObject : anObject , type : type_ , index : index]
    }
    pub unsafe fn initWithObject_type_index_associatedIndex(
        &self,
        anObject: Option<&ObjectType>,
        type_: NSCollectionChangeType,
        index: NSUInteger,
        associatedIndex: NSUInteger,
    ) -> Id<Self, Shared> {
        msg_send_id ! [self , initWithObject : anObject , type : type_ , index : index , associatedIndex : associatedIndex]
    }
    pub unsafe fn object(&self) -> Option<Id<ObjectType, Shared>> {
        msg_send_id![self, object]
    }
    pub unsafe fn changeType(&self) -> NSCollectionChangeType {
        msg_send![self, changeType]
    }
    pub unsafe fn index(&self) -> NSUInteger {
        msg_send![self, index]
    }
    pub unsafe fn associatedIndex(&self) -> NSUInteger {
        msg_send![self, associatedIndex]
    }
}
