use super::{NSObject, NSString};
use crate::rc::{Id, Owned, Shared};
use crate::{extern_class, extern_methods, extern_protocol, ClassType, ConformsTo, ProtocolType};

// TODO: Proper Send/Sync impls here

extern_protocol!(
    pub struct NSLocking;

    unsafe impl ProtocolType for NSLocking {
        #[method(lock)]
        pub unsafe fn lock(&self);

        #[method(unlock)]
        pub unsafe fn unlock(&self);
    }
);

extern_class!(
    #[derive(Debug)]
    pub struct NSLock;

    unsafe impl ClassType for NSLock {
        type Super = NSObject;
    }
);

unsafe impl ConformsTo<NSLocking> for NSLock {}

extern_methods!(
    unsafe impl NSLock {
        #[method_id(new)]
        pub fn new() -> Id<Self, Owned>;

        #[method(tryLock)]
        pub unsafe fn try_lock(&self) -> bool;

        #[method_id(name)]
        pub fn name(&self) -> Option<Id<NSString, Shared>>;

        #[method(setName:)]
        pub fn set_name(&mut self, name: Option<&NSString>);
    }
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lock_unlock() {
        let lock = NSLock::new();
        let locking: &NSLocking = lock.as_protocol();
        unsafe {
            locking.lock();
            assert!(!lock.try_lock());
            locking.unlock();
            assert!(lock.try_lock());
            locking.unlock();
        }
    }
}
