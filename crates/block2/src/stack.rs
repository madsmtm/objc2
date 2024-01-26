use core::ffi::c_void;
use core::fmt;
use core::marker::PhantomData;
use core::mem::{self, MaybeUninit};
use core::ops::Deref;
use core::ptr::{self, NonNull};
use std::os::raw::c_ulong;

use objc2::encode::{EncodeReturn, Encoding, RefEncode};

use crate::abi::{
    BlockDescriptor, BlockDescriptorCopyDispose, BlockDescriptorPtr, BlockFlags, BlockHeader,
};
use crate::debug::debug_block_header;
use crate::rc_block::rc_new_fail;
use crate::{ffi, Block, BlockArguments, IntoBlock, RcBlock};

/// An Objective-C block constructed on the stack.
///
/// This is a smart pointer that [`Deref`]s to [`Block`].
///
///
/// # Memory layout
///
/// The memory layout of this type is _not_ guaranteed.
///
/// That said, it will always be safe to reintepret pointers to this as a
/// pointer to a `Block<A, R>`.
#[repr(C)]
pub struct StackBlock<A, R, F> {
    p: PhantomData<Block<A, R>>,
    header: BlockHeader,
    /// The block's closure.
    ///
    /// The ABI requires this field to come after the header.
    ///
    /// Note that this is not wrapped in a `ManuallyDrop`; once the
    /// `StackBlock` is dropped, the closure will also be dropped.
    pub(crate) closure: F,
}

// SAFETY: Pointers to the stack block is always safe to reintepret as an
// ordinary block pointer.
unsafe impl<A, R, F> RefEncode for StackBlock<A, R, F> {
    const ENCODING_REF: Encoding = Encoding::Block;
}

// Basic constants and helpers.
impl<A: BlockArguments, R: EncodeReturn, F> StackBlock<A, R, F> {
    /// The size of the block header and the trailing closure.
    ///
    /// This ensures that the closure that the block contains is also moved to
    /// the heap in `_Block_copy` operations.
    const SIZE: c_ulong = mem::size_of::<Self>() as _;

    /// Drop the closure that this block contains.
    unsafe extern "C" fn drop_closure(block: *mut c_void) {
        let block: *mut Self = block.cast();
        // When this function is called, the block no longer lives on the
        // stack, it has been moved to the heap as part of some `_Block_copy`
        // operation, including ownership over the block.
        //
        // It is our task to ensure that the closure's data is properly
        // disposed, which we do by calling `Drop`.

        // We use `addr_of_mut` here to not assume anything about the block's
        // contents. This is probably overly cautious, `BlockHeader` already
        // makes very few assumptions about the validity of the data it
        // contains.
        //
        // SAFETY: The block pointer is valid, and contains the closure.
        let closure = unsafe { ptr::addr_of_mut!((*block).closure) };
        // SAFETY: The ownership of the closure was moved into the block as
        // part of some `_Block_copy` operation, and as such it is valid to
        // drop here.
        unsafe { ptr::drop_in_place(closure) };
    }

    const DESCRIPTOR_BASIC: BlockDescriptor = BlockDescriptor {
        reserved: 0,
        size: Self::SIZE,
    };
}

// `StackBlock::new`
impl<A: BlockArguments, R: EncodeReturn, F: Clone> StackBlock<A, R, F> {
    /// Clone the closure from one block to another.
    unsafe extern "C" fn clone_closure(dst: *mut c_void, src: *const c_void) {
        let dst: *mut Self = dst.cast();
        let src: *const Self = src.cast();
        // When this function is called as part of some `_Block_copy`
        // operation, the `dst` block has been constructed on the heap, and
        // the `src` block has been `memmove`d to that.
        //
        // It is our task to ensure that the rest of the closure's data is
        // properly copied, which we do by calling `Clone`. This newly cloned
        // closure will be dropped in `drop_closure`.

        // We use `addr_of[_mut]` to not assume anything about the block's
        // contents, see `drop_closure` for details.
        //
        // SAFETY: The block pointers are valid, and each contain the closure.
        let dst_closure = unsafe { ptr::addr_of_mut!((*dst).closure) };
        let src_closure = unsafe { &*ptr::addr_of!((*src).closure) };
        // SAFETY: `dst` is valid for writes.
        // TODO: How do we ensure proper alignment?
        //
        // Note: This is slightly inefficient, as we're overwriting the
        // already `memmove`d data once more, which is unnecessary for closure
        // captures that implement `Copy`.
        unsafe { ptr::write(dst_closure, src_closure.clone()) };
    }

    const DESCRIPTOR_WITH_CLONE: BlockDescriptorCopyDispose = BlockDescriptorCopyDispose {
        reserved: 0,
        size: Self::SIZE,
        copy: Some(Self::clone_closure),
        dispose: Some(Self::drop_closure),
    };

    /// Construct a `StackBlock` with the given closure.
    ///
    /// Note that this requires [`Clone`], as a C block is generally assumed
    /// to be copy-able. If you want to avoid that, put the block directly on
    /// the heap using [`RcBlock::new`].
    ///
    /// When the block is called, it will return the value that results from
    /// calling the closure.
    #[inline]
    pub fn new(closure: F) -> Self
    where
        F: IntoBlock<A, Output = R>,
    {
        let header = BlockHeader {
            isa: unsafe { ptr::addr_of!(ffi::_NSConcreteStackBlock) },
            // TODO: Add signature.
            flags: BlockFlags::BLOCK_HAS_COPY_DISPOSE,
            reserved: MaybeUninit::new(0),
            invoke: Some(F::__get_invoke_stack_block()),
            // TODO: Use `Self::DESCRIPTOR_BASIC` when `F: Copy`
            // (probably only possible with specialization).
            descriptor: BlockDescriptorPtr {
                with_copy_dispose: &Self::DESCRIPTOR_WITH_CLONE,
            },
        };
        Self {
            p: PhantomData,
            header,
            closure,
        }
    }

    /// Copy the stack block onto the heap as an `RcBlock`.
    ///
    /// Most of the time you'll want to use [`RcBlock::new`] directly instead.
    //
    // TODO: Place this on `Block<A, R>`, once that carries a lifetime.
    #[inline]
    pub fn to_rc(&self) -> RcBlock<A, R>
    where
        // This bound is required because the `RcBlock` has no way of tracking
        // a lifetime.
        F: 'static,
    {
        let ptr: *const Self = self;
        let ptr: *const Block<A, R> = ptr.cast();
        let ptr: *mut Block<A, R> = ptr as *mut _;
        // SAFETY: A pointer to `StackBlock` is safe to convert to a pointer
        // to `Block`, and because of the `F: 'static` lifetime bound, the
        // block will be alive for the lifetime of the new `RcBlock`.
        unsafe { RcBlock::copy(ptr) }.unwrap_or_else(|| rc_new_fail())
    }

    /// No longer necessary, the implementation does this for you when needed.
    #[inline]
    #[deprecated = "no longer necessary"]
    pub fn copy(self) -> RcBlock<A, R>
    where
        F: 'static,
    {
        self.to_rc()
    }
}

// `RcBlock::new`
impl<A: BlockArguments, R: EncodeReturn, F> StackBlock<A, R, F> {
    unsafe extern "C" fn empty_clone_closure(_dst: *mut c_void, _src: *const c_void) {
        // We do nothing, the closure has been `memmove`'d already, and
        // ownership will be passed in `RcBlock::new`.
    }

    const DESCRIPTOR_WITH_DROP: BlockDescriptorCopyDispose = BlockDescriptorCopyDispose {
        reserved: 0,
        size: Self::SIZE,
        copy: Some(Self::empty_clone_closure),
        dispose: Some(Self::drop_closure),
    };

    /// # Safety
    ///
    /// `_Block_copy` must be called on the resulting stack block only once.
    #[inline]
    pub(crate) unsafe fn new_no_clone(closure: F) -> Self
    where
        F: IntoBlock<A, Output = R>,
    {
        // Don't need to emit copy and dispose helpers if the closure
        // doesn't need it.
        //
        // TODO: Add signature.
        let flags = if mem::needs_drop::<Self>() {
            BlockFlags::BLOCK_HAS_COPY_DISPOSE
        } else {
            BlockFlags::EMPTY
        };
        let descriptor = if mem::needs_drop::<Self>() {
            BlockDescriptorPtr {
                with_copy_dispose: &Self::DESCRIPTOR_WITH_DROP,
            }
        } else {
            BlockDescriptorPtr {
                basic: &Self::DESCRIPTOR_BASIC,
            }
        };

        let header = BlockHeader {
            isa: unsafe { ptr::addr_of!(ffi::_NSConcreteStackBlock) },
            flags,
            reserved: MaybeUninit::new(0),
            invoke: Some(F::__get_invoke_stack_block()),
            descriptor,
        };
        Self {
            p: PhantomData,
            header,
            closure,
        }
    }
}

impl<A, R, F: Clone> Clone for StackBlock<A, R, F> {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            p: PhantomData,
            header: self.header,
            closure: self.closure.clone(),
        }
    }
}

impl<A, R, F: Copy> Copy for StackBlock<A, R, F> {}

impl<A, R, F> Deref for StackBlock<A, R, F> {
    type Target = Block<A, R>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        let ptr: NonNull<Self> = NonNull::from(self);
        let ptr: NonNull<Block<A, R>> = ptr.cast();
        // SAFETY: A pointer to `StackBlock` is always safe to convert to a
        // pointer to `Block`, and the reference will be valid as long the
        // stack block exists.
        //
        // Finally, the stack block is implemented correctly, such that it is
        // safe to call `_Block_copy` on the returned value.
        unsafe { ptr.as_ref() }
    }
}

impl<A, R, F> fmt::Debug for StackBlock<A, R, F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut f = f.debug_struct("StackBlock");
        debug_block_header(&self.header, &mut f);
        f.finish_non_exhaustive()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_size() {
        assert_eq!(
            mem::size_of::<BlockHeader>(),
            <StackBlock<(), (), ()>>::SIZE as _,
        );
        assert_eq!(
            mem::size_of::<BlockHeader>() + mem::size_of::<fn()>(),
            <StackBlock<(), (), fn()>>::SIZE as _,
        );
    }
}
