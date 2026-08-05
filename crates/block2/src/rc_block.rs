use core::any::Any;
use core::fmt;
use core::mem::ManuallyDrop;
use core::ops::Deref;
use core::ptr::NonNull;

use crate::abi::BlockHeader;
use crate::debug::debug_block_header;
use crate::traits::{
    BlockSignature, BoundedBy, ManualBlockEncoding, ManualBlockEncodingExt, NoBlockEncoding,
    UserSpecified,
};
use crate::{ffi, Block, IntoBlock, StackBlock};

/// A reference-counted Objective-C block that is stored on the heap.
///
/// This is a smart pointer that [`Deref`]s to [`Block`], and releases it on
/// `Drop`.
///
/// The lifetime `'b`, generic `Signature` and generic `ThreadKind` are the
/// same as described in [`Block`]'s documentation.
///
///
/// # Memory-layout
///
/// This is guaranteed to have the same size and alignment as a pointer to a
/// block (i.e. same memory layout as `*const Block<'b, Signature>`).
///
/// Additionally, it participates in the null-pointer optimization, that is,
/// `Option<RcBlock<'b, Signature>>` is guaranteed to have the same size as
/// `RcBlock<'b, Signature>`.
#[repr(transparent)]
#[doc(alias = "MallocBlock")]
#[cfg_attr(
    feature = "unstable-coerce-pointee",
    derive(std::marker::CoercePointee)
)]
pub struct RcBlock<'b, Signature, ThreadKind: ?Sized = dyn Any> {
    // Covariant
    ptr: NonNull<Block<'b, Signature, ThreadKind>>,
}

// SAFETY: The block's thread kind is `Send + Sync`. Both of those are
// required, `RcBlock` acts like `Arc<T>`, and for that to be `Send`, `T` must
// be `Send + Sync`.
unsafe impl<'b, Signature: BlockSignature> Send for RcBlock<'b, Signature, dyn Send + Sync> {}
// SAFETY: The block's thread kind is `Send + Sync`. Both of those are
// required, `RcBlock` acts like `Arc<T>`, and for that to be `Sync`, `T` must
// be `Send + Sync`.
unsafe impl<'b, Signature: BlockSignature> Sync for RcBlock<'b, Signature, dyn Send + Sync> {}

impl<'b, Signature, ThreadKind: ?Sized> RcBlock<'b, Signature, ThreadKind> {
    /// Construct a `RcBlock` with the given closure.
    ///
    /// The closure will be coped to the heap on construction.
    ///
    /// When the block is called, it will return the value that results from
    /// calling the closure.
    ///
    ///
    /// ## Examples
    ///
    /// Creating a new block an passing it to a method.
    ///
    /// ```
    /// # struct ExampleObject;
    /// # impl ExampleObject {
    /// #     fn someMethod(&self, block: &block2::Block<'_, fn(i32, i32) -> i32>) {
    /// #         assert_eq!(block.call(5, 8), 13);
    /// #     }
    /// # }
    /// # let obj = ExampleObject;
    /// #
    /// use block2::RcBlock;
    ///
    /// obj.someMethod(&RcBlock::new(|a, b| a + b));
    /// ```
    ///
    /// In some cases, you might need to specify the types of the arguments:
    ///
    /// ```
    /// # struct ExampleObject;
    /// # impl ExampleObject {
    /// #     fn someMethod(&self, block: &block2::Block<'_, fn(i32, i32) -> i32>) {
    /// #         assert_eq!(block.call(5, 8), 13);
    /// #     }
    /// # }
    /// # let obj = ExampleObject;
    /// #
    /// use block2::RcBlock;
    ///
    /// let block = RcBlock::new(|a: i32, b: i32| a + b);
    /// obj.someMethod(&block);
    /// ```
    ///
    /// In other cases, the compiler might complain about the thread kind
    /// being unknown. You can force it to the default `dyn Any` by using the
    /// turbofish `::<_>`.
    ///
    /// ```
    /// use block2::RcBlock;
    ///
    /// let block = RcBlock::<_>::new(|a, b| a + b);
    /// assert_eq!(block.call(5, 8), 13);
    /// ```
    // Note: Unsure if this should be #[inline], but I think it may be able to
    // benefit from not being completely so.
    #[inline]
    pub fn new<Closure>(closure: Closure) -> Self
    where
        Signature: BlockSignature,
        Closure: IntoBlock<'b, Signature> + BoundedBy<ThreadKind>,
    {
        Self::maybe_encoded::<Closure, NoBlockEncoding<Signature>>(closure)
    }

    /// Constructs a new [`RcBlock`] with the given function and encoding
    /// information.
    ///
    /// See [`StackBlock::with_encoding`] as to why and how this could be
    /// useful. The same requirements as [`Self::new`] apply here as well.
    ///
    /// # Example
    ///
    /// ```
    /// # use core::ffi::CStr;
    /// # use block2::{Block, ManualBlockEncoding, RcBlock};
    /// # use objc2_foundation::NSError;
    /// #
    /// struct MyBlockEncoding;
    /// // SAFETY: The encoding is correct.
    /// unsafe impl ManualBlockEncoding for MyBlockEncoding {
    ///     type Signature = fn(*mut NSError) -> i32;
    ///     const ENCODING_CSTR: &'static CStr = if cfg!(target_pointer_width = "64") {
    ///         cr#"i16@?0@"NSError"8"#
    ///     } else {
    ///         cr#"i8@?0@"NSError"4"#
    ///     };
    /// }
    ///
    /// let my_block = RcBlock::<_>::with_encoding::<_, MyBlockEncoding>(|_err: *mut NSError| {
    ///     42i32
    /// });
    /// assert_eq!(my_block.call(core::ptr::null_mut()), 42);
    /// ```
    #[inline]
    pub fn with_encoding<Closure, E>(closure: Closure) -> Self
    where
        Signature: BlockSignature,
        Closure: IntoBlock<'b, Signature> + BoundedBy<ThreadKind>,
        E: ManualBlockEncoding<Signature = Signature>,
    {
        Self::maybe_encoded::<Closure, UserSpecified<E>>(closure)
    }

    fn maybe_encoded<Closure, E>(closure: Closure) -> Self
    where
        Signature: BlockSignature,
        Closure: IntoBlock<'b, Signature> + BoundedBy<ThreadKind>,
        E: ManualBlockEncodingExt<Signature = Signature>,
    {
        // SAFETY: The stack block is copied once below.
        //
        // Note: We could theoretically use `_NSConcreteMallocBlock`, and use
        // `malloc` ourselves to put the block on the heap, but that symbol is
        // not part of the public ABI, and may break in the future.
        //
        // Clang doesn't do this optimization either.
        // <https://github.com/llvm/llvm-project/blob/llvmorg-17.0.6/clang/lib/CodeGen/CGBlocks.cpp#L281-L284>
        let block = unsafe { StackBlock::new_no_clone::<E>(closure) };

        // Transfer ownership from the stack to the heap.
        let mut block = ManuallyDrop::new(block);
        let ptr: *mut StackBlock<'b, Signature, Closure, ThreadKind> = &mut *block;
        let ptr: *mut Block<'b, Signature, ThreadKind> = ptr.cast();
        // SAFETY: The block will be moved to the heap, and we forget the
        // original block because the heap block will drop in our dispose
        // helper.
        unsafe { Self::copy(ptr) }.unwrap_or_else(|| rc_new_fail())
    }

    /// A raw pointer to the underlying block.
    ///
    /// The pointer is valid for at least as long as the `RcBlock` is alive.
    ///
    /// This is an associated method, and must be called as
    /// `RcBlock::as_ptr(&block)`.
    #[inline]
    pub fn as_ptr(this: &Self) -> *mut Block<'b, Signature, ThreadKind> {
        this.ptr.as_ptr()
    }

    /// Consumes the `RcBlock`, passing ownership of the retain count to the
    /// caller.
    ///
    /// After calling this function, the caller is responsible for releasing
    /// the memory with [`ffi::_Block_release`] or similar.
    ///
    /// This is an associated method, and must be called as
    /// `RcBlock::into_raw(block)`.
    ///
    ///
    /// # Examples
    ///
    /// Converting a `RcBlock` to a pointer and back.
    ///
    /// ```
    /// use block2::RcBlock;
    ///
    /// let add2 = RcBlock::<_>::new(|x: i32| -> i32 {
    ///     x + 2
    /// });
    /// let ptr = RcBlock::into_raw(add2);
    /// // SAFETY: The pointer is valid, and ownership from above.
    /// let add2 = unsafe { RcBlock::from_raw(ptr) }.unwrap();
    /// ```
    #[inline]
    pub fn into_raw(this: Self) -> *mut Block<'b, Signature, ThreadKind> {
        let this = ManuallyDrop::new(this);
        this.ptr.as_ptr()
    }

    /// Construct an `RcBlock` from the given block pointer by taking
    /// ownership.
    ///
    /// This will return `None` if the pointer is NULL.
    ///
    ///
    /// # Safety
    ///
    /// The given pointer must point to a valid block, the parameter and
    /// return types must be correct, and the block must have a +1 reference /
    /// retain count from somewhere else.
    ///
    /// Additionally, the block must be safe to call (or, if it is not, then
    /// you must treat every call to the block as `unsafe`).
    #[inline]
    pub unsafe fn from_raw(ptr: *mut Block<'b, Signature, ThreadKind>) -> Option<Self> {
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
    /// See [`Block::copy`] for a safe alternative when you already know the
    /// block pointer is valid.
    ///
    ///
    /// # Safety
    ///
    /// The given pointer must point to a valid block, and the parameter and
    /// return types must be correct.
    ///
    /// Additionally, the block must be safe to call (or, if it is not, then
    /// you must treat every call to the block as `unsafe`).
    #[doc(alias = "Block_copy")]
    #[doc(alias = "_Block_copy")]
    #[inline]
    pub unsafe fn copy(ptr: *mut Block<'b, Signature, ThreadKind>) -> Option<Self> {
        let ptr: *mut Block<'b, Signature, ThreadKind> =
            unsafe { ffi::_Block_copy(ptr.cast()) }.cast();
        // SAFETY: We just copied the block, so the reference count is +1
        unsafe { Self::from_raw(ptr) }
    }
}

impl<'b, Signature, ThreadKind: ?Sized> Clone for RcBlock<'b, Signature, ThreadKind> {
    /// Increase the reference-count of the block.
    #[doc(alias = "Block_copy")]
    #[doc(alias = "_Block_copy")]
    #[inline]
    fn clone(&self) -> Self {
        // SAFETY: The block pointer is valid, and its safety invariant is
        // upheld, since the only way to get an `RcBlock` in the first place
        // is through unsafe functions that requires these preconditions to be
        // upheld.
        unsafe { Self::copy(self.ptr.as_ptr()) }.unwrap_or_else(|| rc_clone_fail())
    }
}

// Intentionally not `#[track_caller]`, to keep the code-size smaller (as this
// error is very unlikely).
fn rc_new_fail() -> ! {
    // This likely means the system is out of memory.
    panic!("failed creating RcBlock")
}

// Intentionally not `#[track_caller]`, to keep the code-size smaller (as this
// error is very unlikely).
fn rc_clone_fail() -> ! {
    unreachable!("cloning a RcBlock bumps the reference count, which should be infallible")
}

impl<'b, Signature, ThreadKind: ?Sized> Deref for RcBlock<'b, Signature, ThreadKind> {
    /// The lifetime and signature of the block is the same.
    type Target = Block<'b, Signature, ThreadKind>;

    #[inline]
    fn deref(&self) -> &Block<'b, Signature, ThreadKind> {
        // SAFETY: The pointer is valid, as ensured by creation methods, and
        // will be so for as long as the `RcBlock` is, since that holds +1
        // reference count.
        unsafe { self.ptr.as_ref() }
    }
}

impl<'b, Signature, ThreadKind: ?Sized> Drop for RcBlock<'b, Signature, ThreadKind> {
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

/// Convert from one [`RcBlock`] to another with a different thread kind.
impl<'b, Signature, ThreadKindFrom: ?Sized, ThreadKindTo: ?Sized>
    AsRef<RcBlock<'b, Signature, ThreadKindTo>> for RcBlock<'b, Signature, ThreadKindFrom>
where
    ThreadKindFrom: BoundedBy<ThreadKindTo>,
{
    #[inline]
    fn as_ref(&self) -> &RcBlock<'b, Signature, ThreadKindTo> {
        // SAFETY: The thread kind is just an extra bound, we can safely erase
        // that (similar to how you can go from
        // `Arc<dyn Fn() + Send + Sync + 'b>` to `Arc<dyn Fn() + 'b>`).
        unsafe { core::mem::transmute(self) }
    }
}

/// Get a [`Block`] reference from the [`RcBlock`].
impl<'b, Signature, ThreadKindFrom: ?Sized, ThreadKindTo: ?Sized>
    AsRef<Block<'b, Signature, ThreadKindTo>> for RcBlock<'b, Signature, ThreadKindFrom>
where
    ThreadKindFrom: BoundedBy<ThreadKindTo>,
{
    #[inline]
    fn as_ref(&self) -> &Block<'b, Signature, ThreadKindTo> {
        // `Deref` to `Block` + `AsRef` to any `Block` thread kind.
        (**self).as_ref()
    }
}

impl<'b, Signature, ThreadKind: ?Sized> fmt::Debug for RcBlock<'b, Signature, ThreadKind> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut f = f.debug_struct("RcBlock");
        let header = unsafe { self.ptr.cast::<BlockHeader>().as_ref() };
        debug_block_header(header, &mut f);
        f.finish_non_exhaustive()
    }
}

#[cfg(test)]
mod tests {
    use alloc::rc::Rc;
    use core::cell::OnceCell;

    use super::*;

    #[test]
    fn return_rc_block() {
        fn get_adder(x: i32) -> RcBlock<'static, fn(i32) -> i32> {
            RcBlock::new(move |y| y + x)
        }

        let add2 = get_adder(2);
        assert_eq!(add2.call(5), 7);
        assert_eq!(add2.call(-1), 1);
    }

    #[test]
    fn rc_block_with_precisely_described_lifetimes() {
        fn args<'a, 'b>(
            f: impl Fn(&'a i32, &'b i32) + 'static,
        ) -> RcBlock<'static, fn(&'a i32, &'b i32)> {
            RcBlock::new(f)
        }

        fn args_return<'a, 'b>(
            f: impl Fn(&'a i32) -> &'b i32 + 'static,
        ) -> RcBlock<'static, fn(&'a i32) -> &'b i32> {
            RcBlock::new(f)
        }

        fn args_entire<'a, 'b>(f: impl Fn(&'a i32) + 'b) -> RcBlock<'b, fn(&'a i32)> {
            RcBlock::new(f)
        }

        fn return_entire<'a, 'b>(f: impl Fn() -> &'a i32 + 'b) -> RcBlock<'b, fn() -> &'a i32> {
            RcBlock::new(f)
        }

        let _ = args(|_, _| {});
        let _ = args_return(|x| x);
        let _ = args_entire(|_| {});
        let _ = return_entire(|| &5);
    }

    #[allow(dead_code)]
    fn covariant<'b>(b: RcBlock<'static, fn()>) -> RcBlock<'b, fn()> {
        b
    }

    #[allow(dead_code)]
    fn new_allows_contravariant<'a>(
        closure: impl Fn(&'a i32) + 'static,
    ) -> RcBlock<'static, fn(&'static i32)> {
        RcBlock::new(closure)
    }

    #[test]
    fn allow_re_entrancy() {
        #[allow(clippy::type_complexity)]
        let block: Rc<OnceCell<RcBlock<'_, fn(u32) -> u32>>> = Rc::new(OnceCell::new());

        let captured_block = block.clone();
        let fibonacci = move |n| {
            let captured_fibonacci = captured_block.get().unwrap();
            match n {
                0 => 0,
                1 => 1,
                n => captured_fibonacci.call(n - 1) + captured_fibonacci.call(n - 2),
            }
        };

        let block = block.get_or_init(|| RcBlock::new(fibonacci));

        assert_eq!(block.call(0), 0);
        assert_eq!(block.call(1), 1);
        assert_eq!(block.call(6), 8);
        assert_eq!(block.call(10), 55);
        assert_eq!(block.call(19), 4181);
    }

    static_assertions::assert_not_impl_any!(RcBlock<'_, fn(), dyn Any>: Send, Sync);
    static_assertions::assert_not_impl_any!(RcBlock<'_, fn(), dyn Send>: Send, Sync);
    static_assertions::assert_not_impl_any!(RcBlock<'_, fn(), dyn Sync>: Send, Sync);
    static_assertions::assert_impl_all!(RcBlock<'_, fn(), dyn Send + Sync>: Send, Sync);
}
