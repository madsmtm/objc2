use core::fmt;
use core::marker::PhantomData;
use core::mem;

use objc2::encode::{EncodeArgument, EncodeReturn, Encoding, RefEncode};

use crate::abi::BlockHeader;
use crate::debug::debug_block_header;

/// Types that may be used as the arguments of an Objective-C block.
///
/// This is implemented for tuples of up to 12 arguments, where each argument
/// implements [`EncodeArgument`].
///
///
/// # Safety
///
/// This is a sealed trait, and should not need to be implemented. Open an
/// issue if you know a use-case where this restrition should be lifted!
pub unsafe trait BlockArguments: Sized {
    /// Calls the given method the block and arguments.
    #[doc(hidden)]
    unsafe fn __call_block<R: EncodeReturn>(
        invoke: unsafe extern "C" fn(),
        block: *mut Block<Self, R>,
        args: Self,
    ) -> R;
}

macro_rules! block_args_impl {
    ($($a:ident: $t:ident),*) => (
        unsafe impl<$($t: EncodeArgument),*> BlockArguments for ($($t,)*) {
            #[inline]
            unsafe fn __call_block<R: EncodeReturn>(
                invoke: unsafe extern "C" fn(),
                block: *mut Block<Self, R>,
                ($($a,)*): Self,
            ) -> R {
                // Very similar to `MessageArguments::__invoke`
                let invoke: unsafe extern "C" fn(*mut Block<Self, R> $(, $t)*) -> R = unsafe {
                    mem::transmute(invoke)
                };

                unsafe { invoke(block $(, $a)*) }
            }
        }
    );
}

block_args_impl!();
block_args_impl!(a: A);
block_args_impl!(a: A, b: B);
block_args_impl!(a: A, b: B, c: C);
block_args_impl!(a: A, b: B, c: C, d: D);
block_args_impl!(a: A, b: B, c: C, d: D, e: E);
block_args_impl!(a: A, b: B, c: C, d: D, e: E, f: F);
block_args_impl!(a: A, b: B, c: C, d: D, e: E, f: F, g: G);
block_args_impl!(a: A, b: B, c: C, d: D, e: E, f: F, g: G, h: H);
block_args_impl!(a: A, b: B, c: C, d: D, e: E, f: F, g: G, h: H, i: I);
block_args_impl!(a: A, b: B, c: C, d: D, e: E, f: F, g: G, h: H, i: I, j: J);
block_args_impl!(
    a: A,
    b: B,
    c: C,
    d: D,
    e: E,
    f: F,
    g: G,
    h: H,
    i: I,
    j: J,
    k: K
);
block_args_impl!(
    a: A,
    b: B,
    c: C,
    d: D,
    e: E,
    f: F,
    g: G,
    h: H,
    i: I,
    j: J,
    k: K,
    l: L
);

/// An Objective-C block that takes arguments of `A` when called and
/// returns a value of `R`.
#[repr(C)]
pub struct Block<A, R> {
    _inner: [u8; 0],
    // We store `BlockHeader` + the closure captures, but `Block` has to
    // remain an empty type otherwise the compiler thinks we only have
    // provenance over `BlockHeader`.
    _header: PhantomData<BlockHeader>,
    // To get correct variance on args and return types
    _p: PhantomData<fn(A) -> R>,
}

unsafe impl<A: BlockArguments, R: EncodeReturn> RefEncode for Block<A, R> {
    const ENCODING_REF: Encoding = Encoding::Block;
}

impl<A: BlockArguments, R: EncodeReturn> Block<A, R> {
    /// Call self with the given arguments.
    ///
    /// # Safety
    ///
    /// This invokes foreign code that the caller must verify doesn't violate
    /// any of Rust's safety rules.
    ///
    /// For example, if this block is shared with multiple references, the
    /// caller must ensure that calling it will not cause a data race.
    pub unsafe fn call(&self, args: A) -> R {
        let ptr: *const Self = self;
        let header = unsafe { ptr.cast::<BlockHeader>().as_ref().unwrap_unchecked() };
        // TODO: Is `invoke` actually ever null?
        let invoke = header.invoke.unwrap_or_else(|| unreachable!());

        unsafe { A::__call_block(invoke, ptr as *mut Self, args) }
    }
}

impl<A, R> fmt::Debug for Block<A, R> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut f = f.debug_struct("Block");
        let ptr: *const Self = self;
        let header = unsafe { ptr.cast::<BlockHeader>().as_ref().unwrap() };
        debug_block_header(header, &mut f);
        f.finish_non_exhaustive()
    }
}
