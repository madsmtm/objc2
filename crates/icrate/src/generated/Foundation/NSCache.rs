use super::__exported::NSString;
use crate::Foundation::generated::NSObject::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
__inner_extern_class!(
    #[derive(Debug)]
    pub struct NSCache<KeyType: Message, ObjectType: Message>;
    unsafe impl<KeyType: Message, ObjectType: Message> ClassType for NSCache<KeyType, ObjectType> {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl<KeyType: Message, ObjectType: Message> NSCache<KeyType, ObjectType> {
        #[method_id(name)]
        pub unsafe fn name(&self) -> Id<NSString, Shared>;
        # [method (setName :)]
        pub unsafe fn setName(&self, name: &NSString);
        #[method_id(delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<NSCacheDelegate, Shared>>;
        # [method (setDelegate :)]
        pub unsafe fn setDelegate(&self, delegate: Option<&NSCacheDelegate>);
        # [method_id (objectForKey :)]
        pub unsafe fn objectForKey(&self, key: &KeyType) -> Option<Id<ObjectType, Shared>>;
        # [method (setObject : forKey :)]
        pub unsafe fn setObject_forKey(&self, obj: &ObjectType, key: &KeyType);
        # [method (setObject : forKey : cost :)]
        pub unsafe fn setObject_forKey_cost(&self, obj: &ObjectType, key: &KeyType, g: NSUInteger);
        # [method (removeObjectForKey :)]
        pub unsafe fn removeObjectForKey(&self, key: &KeyType);
        #[method(removeAllObjects)]
        pub unsafe fn removeAllObjects(&self);
        #[method(totalCostLimit)]
        pub unsafe fn totalCostLimit(&self) -> NSUInteger;
        # [method (setTotalCostLimit :)]
        pub unsafe fn setTotalCostLimit(&self, totalCostLimit: NSUInteger);
        #[method(countLimit)]
        pub unsafe fn countLimit(&self) -> NSUInteger;
        # [method (setCountLimit :)]
        pub unsafe fn setCountLimit(&self, countLimit: NSUInteger);
        #[method(evictsObjectsWithDiscardedContent)]
        pub unsafe fn evictsObjectsWithDiscardedContent(&self) -> bool;
        # [method (setEvictsObjectsWithDiscardedContent :)]
        pub unsafe fn setEvictsObjectsWithDiscardedContent(
            &self,
            evictsObjectsWithDiscardedContent: bool,
        );
    }
);
pub type NSCacheDelegate = NSObject;
