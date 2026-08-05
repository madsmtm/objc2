use core::any::Any;
use core::fmt;
use core::marker::PhantomData;
use core::ptr::NonNull;

use objc2::encode::{EncodeArgument, EncodeReturn, Encoding, RefEncode};

use crate::abi::BlockHeader;
use crate::debug::debug_block_header;
use crate::traits::BoundedBy;
use crate::{BlockSignature, RcBlock};

/// An opaque type that holds an Objective-C block.
///
/// Objective-C blocks are similar to Rust closures, see [the top-level
/// documentation][crate] for more information.
///
/// `'b` describes the **lifetime of the data** held by the block. If `'b` is
/// `'static`, the block is considered [escaping].
///
/// The generic type `Signature` describes the block's **signature**, that is,
/// the parameter and return types of the block. This is described using a
/// [function pointer type][prim@fn] that implements the [`BlockSignature`]
/// trait. The parameter types must be [`EncodeArgument`] and the return type
/// must be [`EncodeReturn`].
///
/// As an example, you may have the type `Block<'_, fn(u8, u8) -> i32>`, and
/// that would be a non-escaping block that takes two `u8`s, and returns an
/// `i32`.
///
/// Finally, the generic type `ThreadKind` describes the thread-safety of the
/// block's data. These are used to bound the closures that the block can be
/// created from in [`RcBlock::new`] and similar. Supported values include:
/// - `dyn Any`, indicating that the block is not thread-safe (default).
/// - `dyn Send`, indicating that the block can be dropped on another thread.
///   Very niche.
/// - `dyn Sync`, indicating that the block can be used from on another thread
///   (but must be dropped on this thread). Very niche.
/// - `dyn Send + Sync`, indicating that the block is thread-safe.
///
/// For `dyn Send + Sync`, a convenience alias [`SendableBlock`] is provided
/// to avoid having to spell that out all the time.
///
/// ["escaping"]: https://docs.swift.org/swift-book/documentation/the-swift-programming-language/closures/#Escaping-Closures
/// [`EncodeArgument`]: objc2::encode::EncodeArgument
/// [`EncodeReturn`]: objc2::encode::EncodeReturn
///
///
/// # Creating blocks
///
/// Blocks are usually created using [`RcBlock::new`], and dereferencing the
/// resulting [`RcBlock`]. [`StackBlock::new`] and [`global_block!`] can be
/// used in some cases for optimization purposes.
///
///
/// # Memory layout
///
/// This is intended to be an `extern type`, and as such the memory layout of
/// this type is _not_ guaranteed. That said, **pointers** to this type are
/// always thin, and match that of Objective-C blocks. So the layout of e.g.
/// `&Block<'_, fn(...) -> ...>` is defined, and guaranteed to be
/// pointer-sized and ABI-compatible with a block pointer.
///
///
/// # Safety invariant
///
/// Calling this potentially invokes foreign code, so you must verify, when
/// creating a reference to this, or returning it from an external API, that
/// it doesn't violate any of Rust's safety rules.
///
/// In particular, blocks are sharable with multiple references (see e.g.
/// [`Block::copy`]), so the caller must ensure that calling it can never
/// cause a data race. This usually means you'll have to use some form of
/// interior mutability, if you need to mutate something from inside a block.
//
// TODO: Potentially restrict to `Signature: BlockSignature`, for better error messages?
#[repr(C)]
pub struct Block<'b, Signature, ThreadKind: ?Sized = dyn Any> {
    _inner: [u8; 0],
    /// We store `BlockHeader` + the closure captures, but `Block` has to
    /// remain an empty type because we don't know the size of the closure,
    /// and otherwise the compiler would think we only have provenance over
    /// `BlockHeader`.
    ///
    /// This is possible to improve once we have extern types.
    _header: PhantomData<BlockHeader>,
    // Covariant over both lifetime and signature.
    _p: PhantomData<(&'b (), Signature, ThreadKind)>,
}

/// A [`Block`] that is `Send` and `Sync`.
///
/// This is a convenience type alias, and can be created from [`RcBlock`],
/// [`StackBlock`] or [`GlobalBlock`] in the same manner as [`Block`] (with
/// the required addition that the closure is `Send` and `Sync` of course).
pub type SendableBlock<'b, Signature> = Block<'b, Signature, dyn Send + Sync>;

// SAFETY: The block's thread kind is `Send`.
unsafe impl<'b, Signature: BlockSignature> Send for Block<'b, Signature, dyn Send + Sync> {}
// SAFETY: The block's thread kind is `Send`.
unsafe impl<'b, Signature: BlockSignature> Send for Block<'b, Signature, dyn Send> {}
// SAFETY: The block's thread kind is `Sync`.
unsafe impl<'b, Signature: BlockSignature> Sync for Block<'b, Signature, dyn Send + Sync> {}
// SAFETY: The block's thread kind is `Sync`.
unsafe impl<'b, Signature: BlockSignature> Sync for Block<'b, Signature, dyn Sync> {}

// SAFETY: Pointers to `Block` is an Objective-C block.
// This is only valid when `Signature: BlockSignature`, as that bounds the parameters and
// return type to be encodable too.
unsafe impl<'b, Signature: BlockSignature, ThreadKind: ?Sized> RefEncode
    for Block<'b, Signature, ThreadKind>
{
    const ENCODING_REF: Encoding = Encoding::Block;
}

impl<'b, Signature, ThreadKind: ?Sized> Block<'b, Signature, ThreadKind> {
    fn header(&self) -> &BlockHeader {
        let ptr: NonNull<Self> = NonNull::from(self);
        let ptr: NonNull<BlockHeader> = ptr.cast();
        // SAFETY: `Block` is `BlockHeader` + closure
        unsafe { ptr.as_ref() }
    }

    /// Copy the block onto the heap as an [`RcBlock`].
    ///
    /// The behaviour of this function depends on whether the block is from a
    /// [`RcBlock`] or a [`StackBlock`]. In the former case, it will bump the
    /// reference-count (just as-if you'd `Clone`'d the `RcBlock`), in the
    /// latter case it will construct a new `RcBlock` from the `StackBlock`.
    ///
    /// This distinction should not matter, except for micro-optimizations.
    ///
    /// [`StackBlock`]: crate::StackBlock
    #[doc(alias = "Block_copy")]
    #[doc(alias = "_Block_copy")]
    #[doc(alias = "retain")]
    #[inline]
    pub fn copy(&self) -> RcBlock<'b, Signature, ThreadKind> {
        let ptr: *const Self = self;
        let ptr: *mut Block<'b, Signature, ThreadKind> = ptr as *mut _;
        // SAFETY: The lifetime of the block is extended from `&self` to that
        // of the `RcBlock`, which is fine, because the lifetime `'b` of the
        // contained closure is still carried along to the `RcBlock`.
        unsafe { RcBlock::copy(ptr) }.unwrap_or_else(|| block_copy_fail())
    }

    #[inline]
    fn invoke_ptr(&self) -> unsafe extern "C-unwind" fn() {
        // SAFETY: `invoke` is never NULL - Clang also assumes this in its
        // codegen, and will null ptr deref if it is.
        unsafe { self.header().invoke.unwrap_unchecked() }
    }
}

// Add `call` method to blocks for each argument combination.
//
// In the future, we should use some form of variadic generics here, or maybe
// implement the `Fn` traits for blocks (once that is stable). For now, this
// is the only way to avoid redundant parentheses when calling blocks.
macro_rules! call_block {
    ($num_args:literal; $($a:ident: $t:ident),*) => {
        impl<'b, $($t: EncodeArgument,)* R: EncodeReturn, ThreadKind: ?Sized> Block<'b, fn($($t),*) -> R, ThreadKind> {
            #[doc = concat!("Call the block with ", $num_args, " arguments.")]
            ///
            /// The return value is the output of the block.
            #[doc(alias = "invoke")]
            #[inline]
            #[allow(clippy::too_many_arguments)]
            pub fn call(&self, $($a: $t),*) -> R {
                // Very similar to `MessageArguments::__invoke`

                // SAFETY: We're transmuting to the signature of the `Block`'s
                // `invoke` function, which should be the correct one based on
                // the type parameters given that we hold a reference to this
                // `Block` with this signature in the first place.
                let invoke: unsafe extern "C-unwind" fn(&Self $(, $t)*) -> R = unsafe {
                    core::mem::transmute(self.invoke_ptr())
                };

                // SAFETY: The block's "closure" is an `Fn`, and as such is
                // safe to call from an immutable reference.
                unsafe { invoke(self $(, $a)*) }
            }
        }
    };
}

call_block!(0;);
call_block!(1; t1: T1);
call_block!(2; t1: T1, t2: T2);
call_block!(3; t1: T1, t2: T2, t3: T3);
call_block!(4; t1: T1, t2: T2, t3: T3, t4: T4);
call_block!(5; t1: T1, t2: T2, t3: T3, t4: T4, t5: T5);
call_block!(6; t1: T1, t2: T2, t3: T3, t4: T4, t5: T5, t6: T6);
call_block!(7; t1: T1, t2: T2, t3: T3, t4: T4, t5: T5, t6: T6, t7: T7);
call_block!(8; t1: T1, t2: T2, t3: T3, t4: T4, t5: T5, t6: T6, t7: T7, t8: T8);
call_block!(9; t1: T1, t2: T2, t3: T3, t4: T4, t5: T5, t6: T6, t7: T7, t8: T8, t9: T9);
call_block!(10; t1: T1, t2: T2, t3: T3, t4: T4, t5: T5, t6: T6, t7: T7, t8: T8, t9: T9, t10: T10);
call_block!(11; t1: T1, t2: T2, t3: T3, t4: T4, t5: T5, t6: T6, t7: T7, t8: T8, t9: T9, t10: T10, t11: T11);
call_block!(12; t1: T1, t2: T2, t3: T3, t4: T4, t5: T5, t6: T6, t7: T7, t8: T8, t9: T9, t10: T10, t11: T11, t12: T12);

/// Convert from one [`Block`] to another with a different thread kind.
impl<'b, Signature, ThreadKindFrom: ?Sized, ThreadKindTo: ?Sized>
    AsRef<Block<'b, Signature, ThreadKindTo>> for Block<'b, Signature, ThreadKindFrom>
where
    ThreadKindFrom: BoundedBy<ThreadKindTo>,
{
    #[inline]
    fn as_ref(&self) -> &Block<'b, Signature, ThreadKindTo> {
        // SAFETY: The thread kind is just an extra bound, we can safely erase
        // that (similar to how you can go from `dyn Fn() + Send + Sync + 'b`
        // to `dyn Fn() + 'b`).
        unsafe { core::mem::transmute(self) }
    }
}

impl<'b, Signature, ThreadKind: ?Sized> fmt::Debug for Block<'b, Signature, ThreadKind> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut f = f.debug_struct("Block");
        debug_block_header(self.header(), &mut f);
        f.finish_non_exhaustive()
    }
}

// Intentionally not `#[track_caller]`, to keep the code-size smaller (as this
// error is very unlikely).
fn block_copy_fail() -> ! {
    // This likely means the system is out of memory.
    panic!("failed copying Block")
}

#[cfg(test)]
mod tests {
    use core::cell::Cell;
    use core::sync::atomic::{AtomicUsize, Ordering};

    use crate::StackBlock;

    use super::*;

    /// Test that the way you specify lifetimes are as documented in the
    /// reference.
    /// <https://doc.rust-lang.org/nightly/reference/lifetime-elision.html#default-trait-object-lifetimes>
    #[test]
    fn test_rust_dyn_lifetime_semantics() {
        fn takes_static(block: &Block<'static, fn()>) {
            block.call();
        }

        fn takes_elided(block: &Block<'_, fn()>) {
            block.call();
        }

        #[allow(elided_lifetimes_in_paths)]
        fn takes_unspecified(block: &Block<fn()>) {
            block.call();
        }

        // Static lifetime
        static MY_STATIC: AtomicUsize = AtomicUsize::new(0);
        MY_STATIC.store(0, Ordering::Relaxed);
        let static_lifetime: RcBlock<'static, fn()> = RcBlock::new(|| {
            MY_STATIC.fetch_add(1, Ordering::Relaxed);
        });
        takes_static(&static_lifetime);
        takes_elided(&static_lifetime);
        takes_unspecified(&static_lifetime);
        assert_eq!(MY_STATIC.load(Ordering::Relaxed), 3);

        // Lifetime declared with `'_`
        let captured = Cell::new(0);
        let elided_lifetime: RcBlock<'_, fn()> = RcBlock::new(|| {
            captured.set(captured.get() + 1);
        });
        // takes_static(&elided_lifetime); // Compile error
        takes_elided(&elided_lifetime);
        takes_unspecified(&elided_lifetime);
        assert_eq!(captured.get(), 2);

        // Lifetime kept unspecified
        let captured = Cell::new(0);
        #[allow(elided_lifetimes_in_paths)]
        let unspecified_lifetime: RcBlock<fn()> = RcBlock::new(|| {
            captured.set(captured.get() + 1);
        });
        // takes_static(&unspecified_lifetime); // Compile error
        takes_elided(&unspecified_lifetime);
        takes_unspecified(&unspecified_lifetime);
        assert_eq!(captured.get(), 2);
    }

    fn takes_lifetime<'l>(block: &Block<'_, fn(&'l i32)>) {
        // This block is not callable with anything other than `'static` data,
        // the lifetime is chosen by the caller.
        block.call(&42);
    }

    #[allow(dead_code)]
    fn takes_higher_ranked_lifetime(block: &Block<'_, fn(&i32)>) {
        // This one can be called with locals.
        // (_Why_ is actually a bit of a mystery, the `Block::call` impl
        // shouldn't allow for this?)
        let x = 42;
        block.call(&x);
    }

    #[test]
    fn lifetime() {
        // Parameter type cannot be named, as it is not higher-ranked.
        let cell = std::cell::Cell::new(&0);
        takes_lifetime(&RcBlock::new(|x| {
            cell.set(x);
        }));
        assert_eq!(*cell.get(), 42);

        // Conversely, this usage _requires_ naming the parameter, due to Rust
        // closures being weird here.
        //
        // Doesn't work yet, see https://github.com/madsmtm/objc2/issues/837.
        //
        // takes_higher_ranked_lifetime(&RcBlock::new(|x: &i32| {
        //     assert_eq!(*x, 42);
        // }));
    }

    #[allow(dead_code)]
    fn covariant<'a, 'b>(b: &'a Block<'static, fn()>) -> &'a Block<'b, fn()> {
        b
    }

    #[allow(dead_code)]
    fn inner_covariant<'a, 'r, 'b, 'p>(
        b: &'p Block<'static, fn(&'a i32) -> &'static i32>,
    ) -> &'p Block<'b, fn(&'static i32) -> &'r i32> {
        b
    }

    static_assertions::assert_not_impl_any!(Block<'_, fn(), dyn Any>: Send, Sync);
    static_assertions::assert_not_impl_any!(Block<'_, fn(), dyn Send>: Sync);
    static_assertions::assert_not_impl_any!(Block<'_, fn(), dyn Sync>: Send);
    static_assertions::assert_impl_all!(Block<'_, fn(), dyn Send>: Send);
    static_assertions::assert_impl_all!(Block<'_, fn(), dyn Sync>: Sync);
    static_assertions::assert_impl_all!(Block<'_, fn(), dyn Send + Sync>: Send, Sync);
    static_assertions::assert_impl_all!(SendableBlock<'_, fn()>: Send, Sync); // Just for clarity

    // Consequence: using `dyn Send` is basically the same as `dyn Any`.
    static_assertions::assert_impl_all!(&Block<'_, fn(), dyn Sync>: Send, Sync);
    static_assertions::assert_not_impl_any!(&Block<'_, fn(), dyn Send>: Send, Sync);

    #[test]
    fn as_ref() {
        let block: RcBlock<'_, _, dyn Send + Sync> = RcBlock::new(|| {});

        let _: &RcBlock<'_, _, dyn Any> = block.as_ref();
        let _: &RcBlock<'_, _, dyn Send> = block.as_ref();
        let _: &RcBlock<'_, _, dyn Sync> = block.as_ref();
        let _: &RcBlock<'_, _, dyn Send + Sync> = block.as_ref();
        let _: &Block<'_, _, dyn Any> = block.as_ref();
        let _: &Block<'_, _, dyn Send> = block.as_ref();
        let _: &Block<'_, _, dyn Sync> = block.as_ref();
        let _: &Block<'_, _, dyn Send + Sync> = block.as_ref();

        let block: StackBlock<'_, _, _, dyn Send + Sync> = StackBlock::new(|| {});

        let _: &StackBlock<'_, _, _, dyn Any> = block.as_ref();
        let _: &StackBlock<'_, _, _, dyn Send> = block.as_ref();
        let _: &StackBlock<'_, _, _, dyn Sync> = block.as_ref();
        let _: &StackBlock<'_, _, _, dyn Send + Sync> = block.as_ref();
        let _: &Block<'_, _, dyn Any> = block.as_ref();
        let _: &Block<'_, _, dyn Send> = block.as_ref();
        let _: &Block<'_, _, dyn Sync> = block.as_ref();
        let _: &Block<'_, _, dyn Send + Sync> = block.as_ref();
    }
}
