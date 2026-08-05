use core::ffi::c_ulong;
use core::fmt;
use core::marker::PhantomData;
use core::mem;
use core::mem::MaybeUninit;
use core::ops::Deref;
use core::ptr::{self, NonNull};

use crate::abi::{BlockDescriptor, BlockDescriptorPtr, BlockFlags, BlockHeader};
use crate::debug::debug_block_header;
use crate::traits::BoundedBy;
use crate::{Block, BlockSignature, SendableBlock};

// TODO: Should this be a static to help the compiler deduplicating them?
const GLOBAL_DESCRIPTOR: BlockDescriptor = BlockDescriptor {
    reserved: 0,
    size: mem::size_of::<BlockHeader>() as c_ulong,
};

/// A global Objective-C block that does not capture from the environment.
///
/// This can be used as an optimization of [`RcBlock`] if your closure doesn't
/// capture any variables.
///
/// This is a smart pointer that [`Deref`]s to [`SendableBlock`].
///
/// It can created and stored in static memory using the [`global_block!`]
/// macro.
///
/// [`RcBlock`]: crate::RcBlock
/// [`global_block!`]: crate::global_block
#[repr(C)]
pub struct GlobalBlock<Signature> {
    header: BlockHeader,
    // We don't store a function pointer, instead it is placed inside the
    // invoke function.
    f: PhantomData<Signature>,
}

// Global blocks don't store any state, so they're trivially `Send` + `Sync`.
//
// See below for reason behind `BlockSignature` bound.
unsafe impl<Signature: BlockSignature> Sync for GlobalBlock<Signature> {}
unsafe impl<Signature: BlockSignature> Send for GlobalBlock<Signature> {}

// Note: We can't put correct bounds on signature because we have a const fn,
// and that's not allowed yet in our MSRV.
//
// Fortunately, we don't need them, since they're present on `Sync`, so
// constructing the static in `global_block!` with an invalid `GlobalBlock`
// triggers an error.
impl<Signature> GlobalBlock<Signature> {
    // TODO: Use new ABI with BLOCK_HAS_SIGNATURE
    const FLAGS: BlockFlags = BlockFlags::BLOCK_IS_GLOBAL.union(BlockFlags::BLOCK_USE_STRET);

    #[doc(hidden)]
    #[allow(clippy::declare_interior_mutable_const)]
    pub const __DEFAULT_HEADER: BlockHeader = BlockHeader {
        // Populated in `global_block!`
        isa: ptr::null_mut(),
        flags: Self::FLAGS,
        reserved: MaybeUninit::new(0),
        // Populated in `global_block!`
        invoke: None,
        descriptor: BlockDescriptorPtr {
            basic: &GLOBAL_DESCRIPTOR,
        },
    };

    /// Use the [`global_block`] macro instead.
    #[doc(hidden)]
    #[inline]
    pub const unsafe fn from_header(header: BlockHeader) -> Self {
        Self {
            header,
            f: PhantomData,
        }
    }

    // TODO: Add some constructor for when `Signature: Copy`.
}

impl<Signature> Deref for GlobalBlock<Signature> {
    /// Global blocks are escaping and sendable.
    type Target = SendableBlock<'static, Signature>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        let ptr: NonNull<Self> = NonNull::from(self);
        let ptr: NonNull<SendableBlock<'static, Signature>> = ptr.cast();
        // SAFETY: This has the same layout as `SendableBlock`
        //
        // A global block does not hold any data, so it is safe to call
        // immutably and share across threads.
        unsafe { ptr.as_ref() }
    }
}

/// Get a [`Block`] reference from the [`GlobalBlock`].
impl<Signature, ThreadKind: ?Sized> AsRef<Block<'static, Signature, ThreadKind>>
    for GlobalBlock<Signature>
where
    dyn Send + Sync: BoundedBy<ThreadKind>,
{
    #[inline]
    fn as_ref(&self) -> &Block<'static, Signature, ThreadKind> {
        // `Deref` to `Block` + `AsRef` to any `Block` thread kind.
        (**self).as_ref()
    }
}

impl<Signature> fmt::Debug for GlobalBlock<Signature> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut f = f.debug_struct("GlobalBlock");
        debug_block_header(&self.header, &mut f);
        f.finish_non_exhaustive()
    }
}

/// Construct a static [`GlobalBlock`].
///
/// The syntax is similar to a static closure, except that all types have to
/// be specified. Note that the block cannot capture its environment.
///
/// The block's parameter types must be [`EncodeArgument`] and the return type
/// must be [`EncodeReturn`].
///
/// [`EncodeArgument`]: objc2::encode::EncodeArgument
/// [`EncodeReturn`]: objc2::encode::EncodeReturn
///
/// # Examples
///
/// ```
/// use block2::global_block;
/// global_block! {
///     static MY_BLOCK = || -> i32 {
///         42
///     };
/// }
/// assert_eq!(MY_BLOCK.call(), 42);
/// ```
///
/// ```
/// use block2::global_block;
/// global_block! {
///     static ADDER_BLOCK = |x: i32, y: i32| -> i32 {
///         x + y
///     };
/// }
/// assert_eq!(ADDER_BLOCK.call(5, 7), 12);
/// ```
///
/// The following does not compile because the types aren't specified.
///
/// ```compile_fail
/// use block2::global_block;
/// global_block! {
///     pub static IDENTITY_BLOCK = |x /* missing type */| /* missing type */ {
///         x
///     };
/// }
/// ```
///
/// The following does not compile because [`Box`] is not [`EncodeArgument`]:
///
/// ```compile_fail,E0277
/// use block2::global_block;
/// global_block! {
///     pub static BLOCK = |x: Box<i32>| {};
/// }
/// ```
///
/// This also doesn't work (yet), as blocks are overly restrictive about the
/// lifetimes involved.
///
/// ```compile_fail
/// use block2::global_block;
/// global_block! {
///     pub static BLOCK_WITH_LIFETIME = |x: &i32| -> i32 {
///         *x + 42
///     };
/// }
/// let x = 5;
/// let res = BLOCK_WITH_LIFETIME.call(&x);
/// assert_eq!(res, 47);
/// ```
///
/// There is also no way to get a block function that's generic over its
/// parameters. One could imagine the following syntax would work, but it
/// can't due to implementation limitations:
///
/// ```compile_fail
/// use block2::global_block;
/// global_block! {
///     pub static BLOCK<T: objc2::encode::Encode> = |x: T| {};
/// }
/// ```
///
/// [`Box`]: alloc::boxed::Box
#[macro_export]
macro_rules! global_block {
    // `||` is parsed as one token
    (
        $(#[$m:meta])*
        $vis:vis static $name:ident = || $(-> $r:ty)? $body:block $(;)?
    ) => {
        $crate::global_block!(
            $(#[$m])*
            $vis static $name = |,| $(-> $r)? $body
        );
    };
    (
        $(#[$m:meta])*
        $vis:vis static $name:ident = |$($a:ident: $t:ty),* $(,)?| $(-> $r:ty)? $body:block $(;)?
    ) => {
        $(#[$m])*
        #[allow(unused_unsafe)]
        $vis static $name: $crate::GlobalBlock<fn($($t),*) $(-> $r)?> = unsafe {
            let mut header = $crate::GlobalBlock::<fn($($t),*) $(-> $r)?>::__DEFAULT_HEADER;
            header.isa = ::core::ptr::addr_of!($crate::ffi::_NSConcreteGlobalBlock);
            header.invoke = ::core::option::Option::Some({
                unsafe extern "C-unwind" fn inner(
                    _: *mut $crate::GlobalBlock<fn($($t),*) $(-> $r)?>,
                    $($a: $t),*
                ) $(-> $r)? {
                    $body
                }

                // TODO: SAFETY
                ::core::mem::transmute::<
                    unsafe extern "C-unwind" fn(*mut $crate::GlobalBlock<fn($($t),*) $(-> $r)?>, $($a: $t),*) $(-> $r)?,
                    unsafe extern "C-unwind" fn(),
                >(inner)
            });
            $crate::GlobalBlock::from_header(header)
        };
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::format;

    global_block! {
        /// Test comments and visibility
        pub(super) static NOOP_BLOCK = || {};
    }

    global_block! {
        /// Multiple parameters + trailing comma
        #[allow(unused)]
        static BLOCK = |x: i32, y: i32, z: i32, w: i32,| -> i32 {
            x + y + z + w
        };
    }

    #[test]
    fn test_noop_block() {
        NOOP_BLOCK.call();
    }

    #[test]
    fn test_defined_in_function() {
        global_block!(static MY_BLOCK = || -> i32 {
            42
        });
        assert_eq!(MY_BLOCK.call(), 42);
    }

    #[cfg(target_vendor = "apple")]
    const DEBUG_BLOCKFLAGS: &str = r#"BlockFlags {
        value: "00110000000000000000000000000000",
        deallocating: false,
        inline_layout_string: false,
        small_descriptor: false,
        is_noescape: false,
        needs_free: false,
        has_copy_dispose: false,
        has_ctor: false,
        is_gc: false,
        is_global: true,
        use_stret: true,
        has_signature: false,
        has_extended_layout: false,
        over_referenced: false,
        reference_count: 0,
        ..
    }"#;

    #[cfg(not(target_vendor = "apple"))]
    const DEBUG_BLOCKFLAGS: &str = r#"BlockFlags {
        value: "00110000000000000000000000000000",
        has_copy_dispose: false,
        has_ctor: false,
        is_global: true,
        use_stret: true,
        has_signature: false,
        over_referenced: false,
        reference_count: 0,
        ..
    }"#;

    #[test]
    fn test_debug() {
        let invoke = NOOP_BLOCK.header.invoke.unwrap();
        let size = mem::size_of::<BlockHeader>();
        let maybeuninit = <MaybeUninit<i32>>::uninit();
        let expected = format!(
            "GlobalBlock {{
    isa: _NSConcreteGlobalBlock,
    flags: {DEBUG_BLOCKFLAGS},
    reserved: {maybeuninit:?},
    invoke: Some(
        {invoke:#?},
    ),
    descriptor: BlockDescriptor {{
        reserved: 0,
        size: {size},
    }},
    ..
}}"
        );
        assert_eq!(format!("{NOOP_BLOCK:#?}"), expected);
    }

    static_assertions::assert_impl_all!(GlobalBlock<fn()>: Send, Sync);
}
