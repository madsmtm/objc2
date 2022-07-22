#[cfg(feature = "block")]
use alloc::vec::Vec;
use core::ffi::c_void;
use core::ops::Index;
use core::panic::{RefUnwindSafe, UnwindSafe};
use core::slice::{self, SliceIndex};

use objc2::rc::{DefaultId, Id, Shared};
use objc2::runtime::{Class, Object};
use objc2::{msg_send, msg_send_id};

use crate::{extern_class, NSCopying, NSMutableCopying, NSMutableData, NSObject};

extern_class! {
    /// A static byte buffer in memory.
    ///
    /// This is similar to a [`slice`][`prim@slice`] of [`u8`].
    ///
    /// See [Apple's documentation](https://developer.apple.com/documentation/foundation/nsdata?language=objc).
    #[derive(Debug, PartialEq, Eq, Hash)]
    unsafe pub struct NSData: NSObject;
}

// SAFETY: `NSData` is immutable and `NSMutableData` can only be mutated from
// `&mut` methods.
unsafe impl Sync for NSData {}
unsafe impl Send for NSData {}

impl UnwindSafe for NSData {}
impl RefUnwindSafe for NSData {}

impl NSData {
    pub fn new() -> Id<Self, Shared> {
        unsafe { msg_send_id![Self::class(), new].unwrap() }
    }

    #[doc(alias = "length")]
    pub fn len(&self) -> usize {
        unsafe { msg_send![self, length] }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn bytes(&self) -> &[u8] {
        let ptr: *const c_void = unsafe { msg_send![self, bytes] };
        let ptr: *const u8 = ptr.cast();
        // The bytes pointer may be null for length zero
        if ptr.is_null() {
            &[]
        } else {
            unsafe { slice::from_raw_parts(ptr, self.len()) }
        }
    }

    pub fn with_bytes(bytes: &[u8]) -> Id<Self, Shared> {
        unsafe { Id::new(data_with_bytes(Self::class(), bytes).cast()).unwrap() }
    }

    #[cfg(feature = "block")]
    pub fn from_vec(bytes: Vec<u8>) -> Id<Self, Shared> {
        // GNUStep's NSData `initWithBytesNoCopy:length:deallocator:` has a
        // bug; it forgets to assign the input buffer and length to the
        // instance before it swizzles to NSDataWithDeallocatorBlock.
        // See https://github.com/gnustep/libs-base/pull/213
        // So we just use NSDataWithDeallocatorBlock directly.
        //
        // NSMutableData does not have this problem.
        #[cfg(feature = "gnustep-1-7")]
        let cls = objc2::class!(NSDataWithDeallocatorBlock);
        #[cfg(not(feature = "gnustep-1-7"))]
        let cls = Self::class();

        unsafe { Id::new(data_from_vec(cls, bytes).cast()).unwrap() }
    }
}

unsafe impl NSCopying for NSData {
    type Ownership = Shared;
    type Output = NSData;
}

unsafe impl NSMutableCopying for NSData {
    type Output = NSMutableData;
}

impl alloc::borrow::ToOwned for NSData {
    type Owned = Id<NSData, Shared>;
    fn to_owned(&self) -> Self::Owned {
        self.copy()
    }
}

impl AsRef<[u8]> for NSData {
    fn as_ref(&self) -> &[u8] {
        self.bytes()
    }
}

// Note: We don't implement `Borrow<[u8]>` since we can't guarantee that `Eq`,
// `Ord` and `Hash` are equal for `NSData` vs. `[u8]`!

impl<I: SliceIndex<[u8]>> Index<I> for NSData {
    type Output = I::Output;

    #[inline]
    fn index(&self, index: I) -> &Self::Output {
        // Replaces the need for getBytes:range:
        Index::index(self.bytes(), index)
    }
}

impl DefaultId for NSData {
    type Ownership = Shared;

    #[inline]
    fn default_id() -> Id<Self, Self::Ownership> {
        Self::new()
    }
}

pub(crate) unsafe fn data_with_bytes(cls: &Class, bytes: &[u8]) -> *mut Object {
    let bytes_ptr: *const c_void = bytes.as_ptr().cast();
    unsafe {
        let obj: *mut Object = msg_send![cls, alloc];
        msg_send![
            obj,
            initWithBytes: bytes_ptr,
            length: bytes.len(),
        ]
    }
}

#[cfg(feature = "block")]
pub(crate) unsafe fn data_from_vec(cls: &Class, bytes: Vec<u8>) -> *mut Object {
    use core::mem::ManuallyDrop;

    use block2::{Block, ConcreteBlock};

    let capacity = bytes.capacity();

    let dealloc = ConcreteBlock::new(move |bytes: *mut c_void, len: usize| unsafe {
        // Recreate the Vec and let it drop
        let _ = Vec::<u8>::from_raw_parts(bytes.cast(), len, capacity);
    });
    let dealloc = dealloc.copy();
    let dealloc: &Block<(*mut c_void, usize), ()> = &dealloc;

    let mut bytes = ManuallyDrop::new(bytes);
    let bytes_ptr: *mut c_void = bytes.as_mut_ptr().cast();

    unsafe {
        let obj: *mut Object = msg_send![cls, alloc];
        msg_send![
            obj,
            initWithBytesNoCopy: bytes_ptr,
            length: bytes.len(),
            deallocator: dealloc,
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(feature = "block")]
    use alloc::vec;

    #[test]
    fn test_bytes() {
        let bytes = [3, 7, 16, 52, 112, 19];
        let data = NSData::with_bytes(&bytes);
        assert_eq!(data.len(), bytes.len());
        assert_eq!(data.bytes(), bytes);
    }

    #[test]
    fn test_no_bytes() {
        let data = NSData::new();
        assert!(Some(data.bytes()).is_some());
    }

    #[cfg(feature = "block")]
    #[test]
    fn test_from_vec() {
        let bytes = vec![3, 7, 16];
        let bytes_ptr = bytes.as_ptr();

        let data = NSData::from_vec(bytes);
        assert_eq!(data.bytes().as_ptr(), bytes_ptr);
    }
}
