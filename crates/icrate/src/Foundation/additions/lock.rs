use objc2::rc::{Id, Owned};
use objc2::ClassType;
use objc2::{extern_methods, ConformsTo};

use crate::Foundation::{NSLock, NSLocking};

// TODO: Proper Send/Sync impls here

unsafe impl ConformsTo<NSLocking> for NSLock {}

extern_methods!(
    unsafe impl NSLock {
        #[method_id(new)]
        pub fn new() -> Id<Self, Owned>;
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
            assert!(!lock.tryLock());
            locking.unlock();
            assert!(lock.tryLock());
            locking.unlock();
        }
    }
}
