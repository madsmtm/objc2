// The header `CoreFoundation/CFBase.h` contains:
//
// #if defined(__WIN64__) && !defined(__LLP64__)
// #define __LLP64__ 1
// #endif
//
// #if __LLP64__
// typedef unsigned long long CFTypeID;
// typedef unsigned long long CFOptionFlags;
// typedef unsigned long long CFHashCode;
// typedef signed long long CFIndex;
// #else
// typedef unsigned long CFTypeID;
// typedef unsigned long CFOptionFlags;
// typedef unsigned long CFHashCode;
// typedef signed long CFIndex;
// #endif
//
// Looking at the corresponding Rust definitions for longs:
// <https://doc.rust-lang.org/1.83.0/src/core/ffi/mod.rs.html#168-179>
// cfg_if! {
//     if #[cfg(all(target_pointer_width = "64", not(windows)))] {
//         pub type c_long = i64;
//         pub type c_ulong = u64;
//     } else {
//         // The minimal size of `long` in the C standard is 32 bits
//         pub type c_long = i32;
//         pub type c_ulong = u32;
//     }
// }
// <https://doc.rust-lang.org/1.83.0/src/core/ffi/mod.rs.html#65-66>
// pub type c_longlong = i64;
// pub type c_ulonglong = u64;
//
// It becomes easy to convince ourselves that combined, these amount to making
// these types be 32-bit on systems with 32-bit pointers and 64-bit on systems
// with 64-bit pointers.
//
// That means we can use `isize`/`usize`, which is more ergonomic.

use core::cell::UnsafeCell;
use core::convert::AsRef;
use core::fmt;
use core::hash;
use core::marker::{PhantomData, PhantomPinned};

use crate::{CFEqual, CFHash};

/// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/cftypeid?language=objc)
pub type CFTypeID = usize;

/// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/cfoptionflags?language=objc)
pub type CFOptionFlags = usize;

/// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/cfhashcode?language=objc)
pub type CFHashCode = usize;

/// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/cfindex?language=objc)
pub type CFIndex = isize;

// Manually define CFType

/// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/cftype?language=objc)
#[repr(C)]
pub struct CFType {
    inner: [u8; 0],
    _p: UnsafeCell<PhantomData<(*const UnsafeCell<()>, PhantomPinned)>>,
}

// Reflexive AsRef impl.
impl AsRef<Self> for CFType {
    #[inline]
    fn as_ref(&self) -> &Self {
        self
    }
}

impl fmt::Debug for CFType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO: Use `CFCopyDescription` here
        f.debug_struct("CFType").finish_non_exhaustive()
    }
}

// Equality in CF has approximately the same semantics as Rust equality.
//
// From the docs:
// > Equality is something specific to each Core Foundation opaque type. For
// > example, two CFNumber objects are equal if the numeric values they
// > represent are equal. Two CFString objects are equal if they represent
// > identical sequences of characters, regardless of encoding.
impl PartialEq for CFType {
    #[inline]
    #[doc(alias = "CFEqual")]
    fn eq(&self, other: &Self) -> bool {
        CFEqual(Some(self), Some(other)) != 0
    }
}

// Similar to NSObject, most types' equality is reflexive.
impl Eq for CFType {}

// From the documentation for CFHash:
// > Two objects that are equal (as determined by the `CFEqual` function) have
// > the same hashing value. However, the converse is not true: two objects
// > with the same hashing value might not be equal. That is, hashing values
// > are not necessarily unique.
//
// I.e. the same semantics as Rust's `Hash`.
impl hash::Hash for CFType {
    #[doc(alias = "CFHash")]
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        CFHash(Some(self)).hash(state);
    }
}

crate::__cf_type_objc2!(CFType, crate::__cf_macro_helpers::Encoding::Void);

// NOTE: impl AsRef<CFType> for AnyObject would probably not be valid, since
// not all Objective-C objects can be used as CoreFoundation objects (?)
