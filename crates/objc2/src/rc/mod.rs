//! # Reference counting utilities.
//!
//! The types in this module provide roughly the same benefits as ARC does to
//! Objective-C.
//!
//! Most importantly, a smart pointer [`Id`] is provided to ensure that
//! objects are correctly retained and released when created and dropped,
//! respectively. This ties in strongly with the [`msg_send_id!`] macro.
//!
//! Weak references may be created using the [`WeakId`] struct; these will not
//! retain the object, but one can attempt to load them and obtain an `Id`, or
//! safely fail if the object has been deallocated.
//!
//! See [the clang documentation][clang-arc] and [the Apple article on memory
//! management][mem-mgmt] (similar document exists [for Core Foundation][cf])
//! for more information on automatic and manual reference counting.
//!
//! It can also be useful to [enable Malloc Debugging][mem-debug] if you're trying
//! to figure out if/where your application has memory errors and leaks.
//!
//! [`msg_send_id!`]: crate::msg_send_id
//! [clang-arc]: https://clang.llvm.org/docs/AutomaticReferenceCounting.html
//! [mem-mgmt]: https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/MemoryMgmt/Articles/MemoryMgmt.html
//! [cf]: https://developer.apple.com/library/archive/documentation/CoreFoundation/Conceptual/CFMemoryMgmt/CFMemoryMgmt.html
//! [mem-debug]: https://developer.apple.com/library/archive/documentation/Performance/Conceptual/ManagingMemory/Articles/MallocDebug.html
//!
//!
//! ## Example
//!
//! ```
//! use objc2::rc::{autoreleasepool, Id, WeakId};
//! use objc2::runtime::NSObject;
//!
//! // Id will release the object when dropped
//! let obj: Id<NSObject> = NSObject::new();
//!
//! // Cloning retains the object an additional time
//! let cloned = obj.clone();
//! autoreleasepool(|pool| {
//!     // Autorelease consumes the Id, but won't actually
//!     // release it until the end of the autoreleasepool
//!     let obj_ref: &NSObject = Id::autorelease(cloned, pool);
//! });
//!
//! // Weak references won't retain the object
//! let weak = WeakId::from_id(&obj);
//! drop(obj);
//! assert!(weak.load().is_none());
//! ```

mod allocated_partial_init;
mod autorelease;
mod id;
mod id_forwarding_impls;
mod id_traits;
mod test_object;
mod weak_id;

pub use self::allocated_partial_init::{Allocated, PartialInit};
pub use self::autorelease::{
    autoreleasepool, autoreleasepool_leaking, AutoreleasePool, AutoreleaseSafe,
};
pub use self::id::Id;
pub use self::id_traits::{DefaultId, IdFromIterator, IdIntoIterator};
#[doc(hidden)]
pub use self::test_object::{__RcTestObject, __ThreadTestData};
pub use self::weak_id::WeakId;
