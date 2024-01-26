use core::fmt;
use core::marker::PhantomData;

use objc2::encode::{EncodeReturn, Encoding, RefEncode};

use crate::abi::BlockHeader;
use crate::debug::debug_block_header;
use crate::BlockArguments;

/// An Objective-C block that takes arguments of `A` when called and
/// returns a value of `R`.
///
///
/// # Memory layout
///
/// This is intented to be an `extern type`, and as such the memory layout of
/// this type is _not_ guaranteed. That said, pointers to this type are always
/// thin, and match that of Objective-C blocks.
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

// SAFETY: Pointers to `Block` is an Objective-C block.
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
