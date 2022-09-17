extern_class!(
    #[derive(Debug)]
    struct NSOrderedCollectionChange;
    unsafe impl ClassType for NSOrderedCollectionChange {
        type Super = NSObject;
    }
);
impl NSOrderedCollectionChange {
    pub unsafe fn changeWithObject_type_index(
        anObject: ObjectType,
        type_: NSCollectionChangeType,
        index: NSUInteger,
    ) -> TodoGenerics {
        msg_send ! [Self :: class () , changeWithObject : anObject , type : type_ , index : index]
    }
    pub unsafe fn changeWithObject_type_index_associatedIndex(
        anObject: ObjectType,
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
        anObject: ObjectType,
        type_: NSCollectionChangeType,
        index: NSUInteger,
    ) -> Id<Self, Shared> {
        msg_send_id ! [self , initWithObject : anObject , type : type_ , index : index]
    }
    pub unsafe fn initWithObject_type_index_associatedIndex(
        &self,
        anObject: ObjectType,
        type_: NSCollectionChangeType,
        index: NSUInteger,
        associatedIndex: NSUInteger,
    ) -> Id<Self, Shared> {
        msg_send_id ! [self , initWithObject : anObject , type : type_ , index : index , associatedIndex : associatedIndex]
    }
    pub unsafe fn object(&self) -> ObjectType {
        msg_send![self, object]
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
