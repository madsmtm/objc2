use super::__exported::NSString;
use crate::Foundation::generated::NSObject::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, msg_send, msg_send_id, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSCache;
    unsafe impl ClassType for NSCache {
        type Super = NSObject;
    }
);
impl NSCache {
    pub unsafe fn objectForKey(&self, key: &KeyType) -> Option<Id<ObjectType, Shared>> {
        msg_send_id![self, objectForKey: key]
    }
    pub unsafe fn setObject_forKey(&self, obj: &ObjectType, key: &KeyType) {
        msg_send![self, setObject: obj, forKey: key]
    }
    pub unsafe fn setObject_forKey_cost(&self, obj: &ObjectType, key: &KeyType, g: NSUInteger) {
        msg_send![self, setObject: obj, forKey: key, cost: g]
    }
    pub unsafe fn removeObjectForKey(&self, key: &KeyType) {
        msg_send![self, removeObjectForKey: key]
    }
    pub unsafe fn removeAllObjects(&self) {
        msg_send![self, removeAllObjects]
    }
    pub unsafe fn name(&self) -> Id<NSString, Shared> {
        msg_send_id![self, name]
    }
    pub unsafe fn setName(&self, name: &NSString) {
        msg_send![self, setName: name]
    }
    pub unsafe fn delegate(&self) -> TodoGenerics {
        msg_send![self, delegate]
    }
    pub unsafe fn setDelegate(&self, delegate: TodoGenerics) {
        msg_send![self, setDelegate: delegate]
    }
    pub unsafe fn totalCostLimit(&self) -> NSUInteger {
        msg_send![self, totalCostLimit]
    }
    pub unsafe fn setTotalCostLimit(&self, totalCostLimit: NSUInteger) {
        msg_send![self, setTotalCostLimit: totalCostLimit]
    }
    pub unsafe fn countLimit(&self) -> NSUInteger {
        msg_send![self, countLimit]
    }
    pub unsafe fn setCountLimit(&self, countLimit: NSUInteger) {
        msg_send![self, setCountLimit: countLimit]
    }
    pub unsafe fn evictsObjectsWithDiscardedContent(&self) -> bool {
        msg_send![self, evictsObjectsWithDiscardedContent]
    }
    pub unsafe fn setEvictsObjectsWithDiscardedContent(
        &self,
        evictsObjectsWithDiscardedContent: bool,
    ) {
        msg_send![
            self,
            setEvictsObjectsWithDiscardedContent: evictsObjectsWithDiscardedContent
        ]
    }
}
pub type NSCacheDelegate = NSObject;
