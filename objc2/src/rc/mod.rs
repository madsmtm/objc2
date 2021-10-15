//! Utilities for reference counting Objective-C objects.
//!
//! The utilities of the `rc` module provide ARC-like semantics for working
//! with Objective-C's reference counted objects in Rust.
//!
//! A smart pointer [`Id`] is provided to ensure that Objective-C objects are
//! retained and released at the proper times.
//!
//! To enforce aliasing rules, an `Id` can be either owned or shared; if it is
//! owned, meaning the `Id` is the only reference to the object, it can be
//! mutably dereferenced. An owned `Id` can be downgraded to a shared `Id`
//! which can be cloned to allow multiple references.
//!
//! Weak references may be created using the [`WeakId`] struct.
//!
//! See [the clang documentation][clang-arc] and [the Apple article on memory
//! management][mem-mgmt] (similar document exists [for Core Foundation][mem-cf])
//! for more information on automatic and manual reference counting.
//!
//! It can also be useful to [enable Malloc Debugging][mem-debug] if you're trying
//! to figure out if/where your application has memory errors and leaks.
//!
//!
//! [clang-arc]: https://clang.llvm.org/docs/AutomaticReferenceCounting.html
//! [mem-mgmt]: https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/MemoryMgmt/Articles/MemoryMgmt.html
//! [mem-cf]: https://developer.apple.com/library/archive/documentation/CoreFoundation/Conceptual/CFMemoryMgmt/CFMemoryMgmt.html
//! [mem-debug]: https://developer.apple.com/library/archive/documentation/Performance/Conceptual/ManagingMemory/Articles/MallocDebug.html

mod autorelease;
mod id;
mod id_forwarding_impls;
mod id_traits;
mod ownership;
mod weak_id;

pub use self::autorelease::{autoreleasepool, AutoreleasePool, AutoreleaseSafe};
pub use self::id::Id;
pub use self::id_traits::{IdSlice, IdSliceMut};
pub use self::ownership::{Owned, Ownership, Shared};
pub use self::weak_id::WeakId;

#[cfg(test)]
mod tests {
    use core::mem::size_of;

    use super::{Id, Owned, Shared, WeakId};

    pub struct TestType {
        _data: [u8; 0], // TODO: `UnsafeCell`?
    }

    #[test]
    fn test_size_of() {
        assert_eq!(size_of::<Id<TestType, Owned>>(), size_of::<&TestType>());
        assert_eq!(size_of::<Id<TestType, Shared>>(), size_of::<&TestType>());
        assert_eq!(
            size_of::<Option<Id<TestType, Owned>>>(),
            size_of::<&TestType>()
        );
        assert_eq!(
            size_of::<Option<Id<TestType, Shared>>>(),
            size_of::<&TestType>()
        );

        assert_eq!(
            size_of::<Option<WeakId<TestType>>>(),
            size_of::<*const ()>()
        );
    }
}
