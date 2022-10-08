use super::__exported::NSDate;
use crate::Foundation::generated::NSObject::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, msg_send, msg_send_id, ClassType};
pub type NSLocking = NSObject;
extern_class!(
    #[derive(Debug)]
    pub struct NSLock;
    unsafe impl ClassType for NSLock {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSLock {
        pub unsafe fn tryLock(&self) -> bool {
            msg_send![self, tryLock]
        }
        pub unsafe fn lockBeforeDate(&self, limit: &NSDate) -> bool {
            msg_send![self, lockBeforeDate: limit]
        }
        pub unsafe fn name(&self) -> Option<Id<NSString, Shared>> {
            msg_send_id![self, name]
        }
        pub unsafe fn setName(&self, name: Option<&NSString>) {
            msg_send![self, setName: name]
        }
    }
);
extern_class!(
    #[derive(Debug)]
    pub struct NSConditionLock;
    unsafe impl ClassType for NSConditionLock {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSConditionLock {
        pub unsafe fn initWithCondition(&self, condition: NSInteger) -> Id<Self, Shared> {
            msg_send_id![self, initWithCondition: condition]
        }
        pub unsafe fn condition(&self) -> NSInteger {
            msg_send![self, condition]
        }
        pub unsafe fn lockWhenCondition(&self, condition: NSInteger) {
            msg_send![self, lockWhenCondition: condition]
        }
        pub unsafe fn tryLock(&self) -> bool {
            msg_send![self, tryLock]
        }
        pub unsafe fn tryLockWhenCondition(&self, condition: NSInteger) -> bool {
            msg_send![self, tryLockWhenCondition: condition]
        }
        pub unsafe fn unlockWithCondition(&self, condition: NSInteger) {
            msg_send![self, unlockWithCondition: condition]
        }
        pub unsafe fn lockBeforeDate(&self, limit: &NSDate) -> bool {
            msg_send![self, lockBeforeDate: limit]
        }
        pub unsafe fn lockWhenCondition_beforeDate(
            &self,
            condition: NSInteger,
            limit: &NSDate,
        ) -> bool {
            msg_send![self, lockWhenCondition: condition, beforeDate: limit]
        }
        pub unsafe fn name(&self) -> Option<Id<NSString, Shared>> {
            msg_send_id![self, name]
        }
        pub unsafe fn setName(&self, name: Option<&NSString>) {
            msg_send![self, setName: name]
        }
    }
);
extern_class!(
    #[derive(Debug)]
    pub struct NSRecursiveLock;
    unsafe impl ClassType for NSRecursiveLock {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSRecursiveLock {
        pub unsafe fn tryLock(&self) -> bool {
            msg_send![self, tryLock]
        }
        pub unsafe fn lockBeforeDate(&self, limit: &NSDate) -> bool {
            msg_send![self, lockBeforeDate: limit]
        }
        pub unsafe fn name(&self) -> Option<Id<NSString, Shared>> {
            msg_send_id![self, name]
        }
        pub unsafe fn setName(&self, name: Option<&NSString>) {
            msg_send![self, setName: name]
        }
    }
);
extern_class!(
    #[derive(Debug)]
    pub struct NSCondition;
    unsafe impl ClassType for NSCondition {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSCondition {
        pub unsafe fn wait(&self) {
            msg_send![self, wait]
        }
        pub unsafe fn waitUntilDate(&self, limit: &NSDate) -> bool {
            msg_send![self, waitUntilDate: limit]
        }
        pub unsafe fn signal(&self) {
            msg_send![self, signal]
        }
        pub unsafe fn broadcast(&self) {
            msg_send![self, broadcast]
        }
        pub unsafe fn name(&self) -> Option<Id<NSString, Shared>> {
            msg_send_id![self, name]
        }
        pub unsafe fn setName(&self, name: Option<&NSString>) {
            msg_send![self, setName: name]
        }
    }
);
