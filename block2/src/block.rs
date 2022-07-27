use core::marker::PhantomData;
use core::mem;

use objc2_encode::{Encode, EncodeArguments, Encoding, RefEncode};

use crate::ffi;

/// Types that may be used as the arguments to an Objective-C block.
pub trait BlockArguments: Sized {
    /// Calls the given `Block` with self as the arguments.
    ///
    /// # Safety
    ///
    /// The given block must point to a valid `Block`.
    ///
    /// This invokes foreign code whose safety the user must guarantee.
    unsafe fn call_block<R>(self, block: *mut Block<Self, R>) -> R;
}

macro_rules! block_args_impl {
    ($($a:ident : $t:ident),*) => (
        impl<$($t),*> BlockArguments for ($($t,)*) {
            unsafe fn call_block<R>(self, block: *mut Block<Self, R>) -> R {
                let layout = unsafe { block.cast::<ffi::Block_layout>().as_ref().unwrap_unchecked() };
                // TODO: Can `invoke` actually be null?
                let invoke: unsafe extern "C" fn() = layout.invoke.unwrap();
                let invoke: unsafe extern "C" fn(*mut Block<Self, R>, $($t),*) -> R =
                    unsafe { mem::transmute(invoke) }
                ;
                let ($($a,)*) = self;
                unsafe { invoke(block, $($a),*) }
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
    // We effectively store `Block_layout` + a bit more, but `Block` has to
    // remain an empty type otherwise the compiler thinks we only have
    // provenance over `Block_layout`.
    _layout: PhantomData<ffi::Block_layout>,
    // To get correct variance on args and return types
    _p: PhantomData<fn(A) -> R>,
}

unsafe impl<A: BlockArguments + EncodeArguments, R: Encode> RefEncode for Block<A, R> {
    const ENCODING_REF: Encoding<'static> = Encoding::Block;
}

impl<A: BlockArguments + EncodeArguments, R: Encode> Block<A, R> {
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
        unsafe { args.call_block(self as *const Self as *mut Self) }
    }
}
