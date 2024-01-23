use core::fmt;
use core::marker::PhantomData;
use core::mem;
use core::mem::MaybeUninit;
use core::ops::Deref;
use core::ptr::{self, NonNull};
use std::os::raw::c_ulong;

use objc2::encode::EncodeReturn;

use crate::abi::{BlockDescriptor, BlockDescriptorPtr, BlockFlags, BlockHeader};
use crate::debug::debug_block_header;
use crate::{Block, BlockArguments};

// TODO: Should this be a static to help the compiler deduplicating them?
const GLOBAL_DESCRIPTOR: BlockDescriptor = BlockDescriptor {
    reserved: 0,
    size: mem::size_of::<BlockHeader>() as c_ulong,
};

/// An Objective-C block that does not capture its environment.
///
/// This is effectively a glorified function pointer, and can created and
/// stored in static memory using the [`global_block!`] macro.
///
/// If [`RcBlock`] is the [`Fn`]-block equivalent, this is likewise the
/// [`fn`]-block equivalent.
///
/// [`RcBlock`]: crate::RcBlock
/// [`global_block!`]: crate::global_block
#[repr(C)]
pub struct GlobalBlock<A, R = ()> {
    header: BlockHeader,
    p: PhantomData<(A, R)>,
}

unsafe impl<A, R> Sync for GlobalBlock<A, R>
where
    A: BlockArguments,
    R: EncodeReturn,
{
}
unsafe impl<A, R> Send for GlobalBlock<A, R>
where
    A: BlockArguments,
    R: EncodeReturn,
{
}

// Note: We can't put correct bounds on A and R because we have a const fn!
//
// Fortunately, we don't need them, since they're present on `Sync`, so
// constructing the static in `global_block!` with an invalid `GlobalBlock`
// triggers an error.
impl<A, R> GlobalBlock<A, R> {
    // TODO: Use new ABI with BLOCK_HAS_SIGNATURE
    const FLAGS: BlockFlags =
        BlockFlags(BlockFlags::BLOCK_IS_GLOBAL.0 | BlockFlags::BLOCK_USE_STRET.0);

    #[doc(hidden)]
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
            p: PhantomData,
        }
    }
}

impl<A, R> Deref for GlobalBlock<A, R>
where
    A: BlockArguments,
    R: EncodeReturn,
{
    type Target = Block<A, R>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        let ptr: NonNull<Self> = NonNull::from(self);
        let ptr: NonNull<Block<A, R>> = ptr.cast();
        // TODO: SAFETY
        unsafe { ptr.as_ref() }
    }
}

impl<A, R> fmt::Debug for GlobalBlock<A, R> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut f = f.debug_struct("GlobalBlock");
        debug_block_header(&self.header, &mut f);
        f.finish_non_exhaustive()
    }
}

/// Construct a static [`GlobalBlock`].
///
/// The syntax is similar to a static closure (except that all types have to
/// be specified). Note that the block cannot capture its environment, and
/// its argument types and return type must be [`EncodeReturn`].
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
/// assert_eq!(unsafe { MY_BLOCK.call(()) }, 42);
/// ```
///
/// ```
/// use block2::global_block;
/// global_block! {
///     static ADDER_BLOCK = |x: i32, y: i32| -> i32 {
///         x + y
///     };
/// }
/// assert_eq!(unsafe { ADDER_BLOCK.call((5, 7)) }, 12);
/// ```
///
/// ```
/// use block2::global_block;
/// global_block! {
///     pub static MUTATING_BLOCK = |x: &mut i32| {
///         *x = *x + 42;
///     };
/// }
/// let mut x = 5;
/// unsafe { MUTATING_BLOCK.call((&mut x,)) };
/// assert_eq!(x, 47);
/// ```
///
/// The following does not compile because [`Box`] is not [`EncodeReturn`]:
///
/// ```compile_fail
/// use block2::global_block;
/// global_block! {
///     pub static BLOCK = |b: Box<i32>| {};
/// }
/// ```
///
/// There is also no way to get a block function that's generic over its
/// arguments. One could imagine the following syntax would work, but it can't
/// due to implementation limitations:
///
/// ```compile_fail
/// use block2::global_block;
/// global_block! {
///     pub static BLOCK<T: Encode> = |b: T| {};
/// }
/// ```
///
/// [`Box`]: std::boxed::Box
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
        $vis static $name: $crate::GlobalBlock<($($t,)*) $(, $r)?> = unsafe {
            let mut header = $crate::GlobalBlock::<($($t,)*) $(, $r)?>::__DEFAULT_HEADER;
            header.isa = ::core::ptr::addr_of!($crate::ffi::_NSConcreteGlobalBlock);
            header.invoke = ::core::option::Option::Some({
                unsafe extern "C" fn inner(_: *mut $crate::GlobalBlock<($($t,)*) $(, $r)?>, $($a: $t),*) $(-> $r)? {
                    $body
                }

                // TODO: SAFETY
                ::core::mem::transmute::<
                    unsafe extern "C" fn(*mut $crate::GlobalBlock<($($t,)*) $(, $r)?>, $($a: $t),*) $(-> $r)?,
                    unsafe extern "C" fn(),
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
        /// Multiple arguments + trailing comma
        #[allow(unused)]
        static BLOCK = |x: i32, y: i32, z: i32, w: i32,| -> i32 {
            x + y + z + w
        };
    }

    #[test]
    fn test_noop_block() {
        unsafe { NOOP_BLOCK.call(()) };
    }

    #[test]
    fn test_defined_in_function() {
        global_block!(static MY_BLOCK = || -> i32 {
            42
        });
        assert_eq!(unsafe { MY_BLOCK.call(()) }, 42);
    }

    #[cfg(feature = "apple")]
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

    #[cfg(not(feature = "apple"))]
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
        let expected = format!(
            "GlobalBlock {{
    isa: _NSConcreteGlobalBlock,
    flags: {DEBUG_BLOCKFLAGS},
    reserved: core::mem::maybe_uninit::MaybeUninit<i32>,
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
}
