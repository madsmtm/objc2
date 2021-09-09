/*!
Utilities for reference counting Objective-C objects.

The utilities of the `rc` module provide ARC-like semantics for working with
Objective-C's reference counted objects in Rust.
A `StrongPtr` retains an object and releases the object when dropped.
A `WeakPtr` will not retain the object, but can be upgraded to a `StrongPtr`
and safely fails if the object has been deallocated.

These utilities are not intended to provide a fully safe interface, but can be
useful when writing higher-level Rust wrappers for Objective-C code.

See [the clang documentation][clang-arc] and [the Apple article on memory
management][mem-mgmt] (similar document exists [for Core Foundation][mem-cf])
for more information on automatic and manual reference counting.

It can also be useful to [enable Malloc Debugging][mem-debug] if you're trying
to figure out if/where your application has memory errors and leaks.


[clang-arc]: https://clang.llvm.org/docs/AutomaticReferenceCounting.html
[mem-mgmt]: https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/MemoryMgmt/Articles/MemoryMgmt.html
[mem-cf]: https://developer.apple.com/library/archive/documentation/CoreFoundation/Conceptual/CFMemoryMgmt/CFMemoryMgmt.html
[mem-debug]: https://developer.apple.com/library/archive/documentation/Performance/Conceptual/ManagingMemory/Articles/MallocDebug.html

# Example

``` no_run
# use objc2::{class, msg_send};
# use objc2::rc::{autoreleasepool, StrongPtr};
// StrongPtr will release the object when dropped
let obj = unsafe {
    StrongPtr::new(msg_send![class!(NSObject), new])
};

// Cloning retains the object an additional time
let cloned = obj.clone();
autoreleasepool(|_| {
    // Autorelease consumes the StrongPtr, but won't
    // actually release until the end of an autoreleasepool
    cloned.autorelease();
});

// Weak references won't retain the object
let weak = obj.weak();
drop(obj);
assert!(weak.load().is_null());
```
*/

mod autorelease;
mod strong;
mod weak;

pub use self::autorelease::{autoreleasepool, AutoreleasePool, AutoreleaseSafe};
pub use self::strong::StrongPtr;
pub use self::weak::WeakPtr;

// These tests use NSObject, which isn't present for GNUstep
#[cfg(all(test, target_vendor = "apple"))]
mod tests {
    use super::autoreleasepool;
    use super::StrongPtr;
    use crate::runtime::Object;

    #[test]
    fn test_strong_clone() {
        fn retain_count(obj: *mut Object) -> usize {
            unsafe { msg_send![obj, retainCount] }
        }

        let obj = unsafe { StrongPtr::new(msg_send![class!(NSObject), new]) };
        assert!(retain_count(*obj) == 1);

        let cloned = obj.clone();
        assert!(retain_count(*cloned) == 2);
        assert!(retain_count(*obj) == 2);

        drop(obj);
        assert!(retain_count(*cloned) == 1);
    }

    #[test]
    fn test_weak() {
        let obj = unsafe { StrongPtr::new(msg_send![class!(NSObject), new]) };
        let weak = obj.weak();

        let strong = weak.load();
        assert!(*strong == *obj);
        drop(strong);

        drop(obj);
        assert!(weak.load().is_null());
    }

    #[test]
    fn test_weak_copy() {
        let obj = unsafe { StrongPtr::new(msg_send![class!(NSObject), new]) };
        let weak = obj.weak();

        let weak2 = weak.clone();

        let strong = weak.load();
        let strong2 = weak2.load();
        assert!(*strong == *obj);
        assert!(*strong2 == *obj);
    }

    #[test]
    fn test_autorelease() {
        let obj = unsafe { StrongPtr::new(msg_send![class!(NSObject), new]) };

        fn retain_count(obj: *mut Object) -> usize {
            unsafe { msg_send![obj, retainCount] }
        }
        let cloned = obj.clone();

        autoreleasepool(|_| {
            obj.autorelease();
            assert!(retain_count(*cloned) == 2);
        });

        // make sure that the autoreleased value has been released
        assert!(retain_count(*cloned) == 1);
    }
}
