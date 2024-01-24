use core::fmt;
use core::mem::ManuallyDrop;
use core::ops::Deref;
use core::ptr::NonNull;

use objc2::encode::EncodeReturn;

use crate::abi::BlockHeader;
use crate::debug::debug_block_header;
use crate::{ffi, Block, BlockArguments, IntoBlock, StackBlock};

/// A reference-counted Objective-C block that is stored on the heap.
///
/// This is a smart pointer that [`Deref`]s to [`Block`].
///
///
/// # Memory-layout
///
/// This is guaranteed to have the same size and alignment as a pointer to a
/// block, `*const Block<A, R>`.
///
/// Additionally, it participates in the null-pointer optimization, that is,
/// `Option<RcBlock<A, R>>` is guaranteed to have the same size as
/// `RcBlock<A, R>`.
#[doc(alias = "MallocBlock")]
pub struct RcBlock<A, R> {
    ptr: NonNull<Block<A, R>>,
}

impl<A, R> RcBlock<A, R> {
    /// Construct an `RcBlock` from the given block pointer by taking
    /// ownership.
    ///
    /// This will return `None` if the pointer is NULL.
    ///
    /// # Safety
    ///
    /// The given pointer must point to a valid block, the arguments and
    /// return type must be correct, and the block must have a +1 reference /
    /// retain count from somewhere else.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut Block<A, R>) -> Option<Self> {
        NonNull::new(ptr).map(|ptr| Self { ptr })
    }

    /// Construct an `RcBlock` from the given block pointer.
    ///
    /// The block will be copied, and have its reference-count increased by
    /// one.
    ///
    /// This will return `None` if the pointer is NULL, or if an allocation
    /// failure occurred.
    ///
    /// # Safety
    ///
    /// The given pointer must point to a valid block, and the arguments and
    /// return type must be correct.
    #[doc(alias = "Block_copy")]
    #[doc(alias = "_Block_copy")]
    #[inline]
    pub unsafe fn copy(ptr: *mut Block<A, R>) -> Option<Self> {
        let ptr: *mut Block<A, R> = unsafe { ffi::_Block_copy(ptr.cast()) }.cast();
        // SAFETY: We just copied the block, so the reference count is +1
        unsafe { Self::from_raw(ptr) }
    }
}

impl<A: BlockArguments, R: EncodeReturn> RcBlock<A, R> {
    /// Construct a `RcBlock` with the given closure.
    ///
    /// The closure will be coped to the heap on construction.
    ///
    /// When the block is called, it will return the value that results from
    /// calling the closure.
    //
    // Note: Unsure if this should be #[inline], but I think it may be able to
    // benefit from not being so.
    pub fn new<F>(closure: F) -> Self
    where
        // The `F: 'static` bound is required because the `RcBlock` has no way
        // of tracking a lifetime.
        F: IntoBlock<A, Output = R> + 'static,
    {
        // SAFETY: The stack block is copied once below.
        //
        // Note: We could theoretically use `_NSConcreteMallocBlock`, and use
        // `malloc` ourselves to put the block on the heap, but that symbol is
        // not part of the public ABI, and may break in the future.
        //
        // Clang doesn't do this optimization either.
        // <https://github.com/llvm/llvm-project/blob/llvmorg-17.0.6/clang/lib/CodeGen/CGBlocks.cpp#L281-L284>
        let block = unsafe { StackBlock::new_no_clone(closure) };

        // Transfer ownership from the stack to the heap.
        let mut block = ManuallyDrop::new(block);
        let ptr: *mut StackBlock<A, R, F> = &mut *block;
        let ptr: *mut Block<A, R> = ptr.cast();
        // SAFETY: The block will be moved to the heap, and we forget the
        // original block because the heap block will drop in our dispose
        // helper.
        unsafe { Self::copy(ptr) }.unwrap_or_else(|| rc_new_fail())
    }
}

impl<A, R> Clone for RcBlock<A, R> {
    #[inline]
    fn clone(&self) -> Self {
        // SAFETY: The pointer is valid, since the only way to get an RcBlock
        // in the first place is through unsafe functions.
        unsafe { Self::copy(self.ptr.as_ptr()) }.unwrap_or_else(|| rc_clone_fail())
    }
}

// Intentionally not `#[track_caller]`, to keep the code-size smaller (as this
// error is very unlikely).
pub(crate) fn rc_new_fail() -> ! {
    // This likely means the system is out of memory.
    panic!("failed creating RcBlock")
}

// Intentionally not `#[track_caller]`, see above.
fn rc_clone_fail() -> ! {
    unreachable!("cloning a RcBlock bumps the reference count, which should be infallible")
}

impl<A, R> Deref for RcBlock<A, R> {
    type Target = Block<A, R>;

    #[inline]
    fn deref(&self) -> &Block<A, R> {
        // SAFETY: The pointer is valid, as ensured by creation methods, and
        // will be so for as long as the `RcBlock` is, since that holds +1
        // reference count.
        unsafe { self.ptr.as_ref() }
    }
}

impl<A, R> Drop for RcBlock<A, R> {
    /// Release the block, decreasing the reference-count by 1.
    ///
    /// The `Drop` method of the underlying closure will be called once the
    /// reference-count reaches zero.
    #[doc(alias = "Block_release")]
    #[doc(alias = "_Block_release")]
    #[inline]
    fn drop(&mut self) {
        // SAFETY: The pointer has +1 reference count, as ensured by creation
        // methods.
        unsafe { ffi::_Block_release(self.ptr.as_ptr().cast()) };
    }
}

impl<A, R> fmt::Debug for RcBlock<A, R> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut f = f.debug_struct("RcBlock");
        let header = unsafe { self.ptr.cast::<BlockHeader>().as_ref() };
        debug_block_header(header, &mut f);
        f.finish_non_exhaustive()
    }
}
