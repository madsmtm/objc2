use super::__exported::NSDate;
use crate::Foundation::generated::NSObject::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSDistributedLock;
    unsafe impl ClassType for NSDistributedLock {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSDistributedLock {
        #[method_id(lockWithPath:)]
        pub unsafe fn lockWithPath(path: &NSString) -> Option<Id<NSDistributedLock, Shared>>;
        #[method_id(init)]
        pub unsafe fn init(&self) -> Id<Self, Shared>;
        #[method_id(initWithPath:)]
        pub unsafe fn initWithPath(&self, path: &NSString) -> Option<Id<Self, Shared>>;
        #[method(tryLock)]
        pub unsafe fn tryLock(&self) -> bool;
        #[method(unlock)]
        pub unsafe fn unlock(&self);
        #[method(breakLock)]
        pub unsafe fn breakLock(&self);
        #[method_id(lockDate)]
        pub unsafe fn lockDate(&self) -> Id<NSDate, Shared>;
    }
);
