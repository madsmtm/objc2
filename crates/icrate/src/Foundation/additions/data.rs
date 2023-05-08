#![cfg(feature = "Foundation_NSData")]
#[cfg(feature = "block")]
use alloc::vec::Vec;
use core::fmt;
use core::ops::Index;
use core::panic::{RefUnwindSafe, UnwindSafe};
use core::slice::{self, SliceIndex};

use objc2::rc::DefaultId;

use crate::common::*;
use crate::Foundation::{self, NSData};

// SAFETY: `NSData` is immutable and `NSMutableData` can only be mutated from
// `&mut` methods.
unsafe impl Sync for NSData {}
unsafe impl Send for NSData {}

impl UnwindSafe for NSData {}
impl RefUnwindSafe for NSData {}

/// Creation methods.
impl NSData {
    pub fn with_bytes(bytes: &[u8]) -> Id<Self> {
        let bytes_ptr = bytes.as_ptr() as *mut c_void;
        unsafe { Self::initWithBytes_length(Self::alloc(), bytes_ptr, bytes.len()) }
    }

    #[cfg(feature = "block")]
    pub fn from_vec(bytes: Vec<u8>) -> Id<Self> {
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

        unsafe { Id::cast(with_vec(cls, bytes)) }
    }
}

/// Accessor methods.
impl NSData {
    pub fn len(&self) -> usize {
        self.length()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn bytes(&self) -> &[u8] {
        let ptr = self.bytes_raw();
        let ptr: *const u8 = ptr.cast();
        // The bytes pointer may be null for length zero
        if ptr.is_null() {
            &[]
        } else {
            unsafe { slice::from_raw_parts(ptr, self.len()) }
        }
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
    #[inline]
    fn default_id() -> Id<Self> {
        Self::new()
    }
}

impl fmt::Debug for NSData {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // -[NSData description] is quite unreadable
        fmt::Debug::fmt(self.bytes(), f)
    }
}

impl<'a> IntoIterator for &'a NSData {
    type Item = &'a u8;
    type IntoIter = core::slice::Iter<'a, u8>;

    fn into_iter(self) -> Self::IntoIter {
        self.bytes().iter()
    }
}

#[cfg(feature = "block")]
pub(crate) unsafe fn with_vec(cls: &Class, bytes: Vec<u8>) -> Id<Object> {
    use core::mem::ManuallyDrop;

    use block2::{Block, ConcreteBlock};
    use objc2::msg_send_id;

    let capacity = bytes.capacity();

    let dealloc = ConcreteBlock::new(move |bytes: *mut c_void, len: usize| unsafe {
        // Recreate the Vec and let it drop
        let _ = <Vec<u8>>::from_raw_parts(bytes.cast(), len, capacity);
    });
    let dealloc = dealloc.copy();
    let dealloc: &Block<(*mut c_void, usize), ()> = &dealloc;

    let mut bytes = ManuallyDrop::new(bytes);
    let bytes_ptr: *mut c_void = bytes.as_mut_ptr().cast();

    unsafe {
        msg_send_id![
            msg_send_id![cls, alloc],
            initWithBytesNoCopy: bytes_ptr,
            length: bytes.len(),
            deallocator: dealloc,
        ]
    }
}
