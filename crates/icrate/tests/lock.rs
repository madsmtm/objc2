#![cfg(feature = "Foundation_NSLock")]
use objc2::ConformsTo;

use icrate::Foundation::{NSLock, NSLocking};

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
