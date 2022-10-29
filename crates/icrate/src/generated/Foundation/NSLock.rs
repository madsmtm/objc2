#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
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
        #[method(tryLock)]
        pub unsafe fn tryLock(&self) -> bool;
        #[method(lockBeforeDate:)]
        pub unsafe fn lockBeforeDate(&self, limit: &NSDate) -> bool;
        #[method_id(name)]
        pub unsafe fn name(&self) -> Option<Id<NSString, Shared>>;
        #[method(setName:)]
        pub unsafe fn setName(&self, name: Option<&NSString>);
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
        #[method_id(initWithCondition:)]
        pub unsafe fn initWithCondition(&self, condition: NSInteger) -> Id<Self, Shared>;
        #[method(condition)]
        pub unsafe fn condition(&self) -> NSInteger;
        #[method(lockWhenCondition:)]
        pub unsafe fn lockWhenCondition(&self, condition: NSInteger);
        #[method(tryLock)]
        pub unsafe fn tryLock(&self) -> bool;
        #[method(tryLockWhenCondition:)]
        pub unsafe fn tryLockWhenCondition(&self, condition: NSInteger) -> bool;
        #[method(unlockWithCondition:)]
        pub unsafe fn unlockWithCondition(&self, condition: NSInteger);
        #[method(lockBeforeDate:)]
        pub unsafe fn lockBeforeDate(&self, limit: &NSDate) -> bool;
        #[method(lockWhenCondition:beforeDate:)]
        pub unsafe fn lockWhenCondition_beforeDate(
            &self,
            condition: NSInteger,
            limit: &NSDate,
        ) -> bool;
        #[method_id(name)]
        pub unsafe fn name(&self) -> Option<Id<NSString, Shared>>;
        #[method(setName:)]
        pub unsafe fn setName(&self, name: Option<&NSString>);
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
        #[method(tryLock)]
        pub unsafe fn tryLock(&self) -> bool;
        #[method(lockBeforeDate:)]
        pub unsafe fn lockBeforeDate(&self, limit: &NSDate) -> bool;
        #[method_id(name)]
        pub unsafe fn name(&self) -> Option<Id<NSString, Shared>>;
        #[method(setName:)]
        pub unsafe fn setName(&self, name: Option<&NSString>);
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
        #[method(wait)]
        pub unsafe fn wait(&self);
        #[method(waitUntilDate:)]
        pub unsafe fn waitUntilDate(&self, limit: &NSDate) -> bool;
        #[method(signal)]
        pub unsafe fn signal(&self);
        #[method(broadcast)]
        pub unsafe fn broadcast(&self);
        #[method_id(name)]
        pub unsafe fn name(&self) -> Option<Id<NSString, Shared>>;
        #[method(setName:)]
        pub unsafe fn setName(&self, name: Option<&NSString>);
    }
);
