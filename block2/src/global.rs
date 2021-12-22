use core::marker::PhantomData;
use core::mem;
use core::ops::Deref;
use core::ptr;
use std::os::raw::c_ulong;

use objc2_encode::{Encode, EncodeArguments};

use super::{ffi, Block};
use crate::BlockArguments;

// TODO: Should this be a static to help the compiler deduplicating them?
const GLOBAL_DESCRIPTOR: ffi::Block_descriptor_header = ffi::Block_descriptor_header {
    reserved: 0,
    size: mem::size_of::<ffi::Block_layout>() as c_ulong,
};

/// An Objective-C block that does not capture it's environment.
///
/// This is effectively just a glorified function pointer, and can created and
/// stored in static memory using the [`global_block`][`global_block!`] macro.
///
/// If [`ConcreteBlock`] is the [`Fn`]-block equivalent, this is likewise the
/// [`fn`]-block equivalent.
#[repr(C)]
pub struct GlobalBlock<A, R = ()> {
    layout: ffi::Block_layout,
    p: PhantomData<(A, R)>,
}

unsafe impl<A, R> Sync for GlobalBlock<A, R>
where
    A: BlockArguments + EncodeArguments,
    R: Encode,
{
}
unsafe impl<A, R> Send for GlobalBlock<A, R>
where
    A: BlockArguments + EncodeArguments,
    R: Encode,
{
}

// Note: We can't put correct bounds on A and R because we have a const fn!
//
// Fortunately, we don't need them, since they're present on `Sync`, so
// constructing the static in `global_block!` with an invalid `GlobalBlock`
// triggers an error.
impl<A, R> GlobalBlock<A, R> {
    // TODO: Use new ABI with BLOCK_HAS_SIGNATURE
    const FLAGS: ffi::block_flags = ffi::BLOCK_IS_GLOBAL | ffi::BLOCK_USE_STRET;

    #[doc(hidden)]
    pub const __DEFAULT_LAYOUT: ffi::Block_layout = ffi::Block_layout {
        // Populated in `global_block!`
        isa: ptr::null_mut(),
        flags: Self::FLAGS,
        reserved: 0,
        // Populated in `global_block!`
        invoke: None,
        descriptor: &GLOBAL_DESCRIPTOR as *const _ as *mut _,
    };

    /// Use the [`global_block`] macro instead.
    #[doc(hidden)]
    pub const unsafe fn from_layout(layout: ffi::Block_layout) -> Self {
        Self {
            layout,
            p: PhantomData,
        }
    }
}

impl<A, R> Deref for GlobalBlock<A, R>
where
    A: BlockArguments + EncodeArguments,
    R: Encode,
{
    type Target = Block<A, R>;

    fn deref(&self) -> &Block<A, R> {
        // TODO: SAFETY
        unsafe { &*(self as *const Self as *const Block<A, R>) }
    }
}

/// Construct a static [`GlobalBlock`].
///
/// The syntax is similar to a static closure. Note that the block cannot
/// capture it's environment, and it's argument types and return type must be
/// [`Encode`].
///
/// # Examples
///
/// ```
/// use block2::global_block;
/// global_block! {
///     static MY_BLOCK = || -> i32 {
///         42
///     }
/// };
/// assert_eq!(unsafe { MY_BLOCK.call(()) }, 42);
/// ```
///
/// ```
/// use block2::global_block;
/// global_block! {
///     static ADDER_BLOCK = |x: i32, y: i32,| -> i32 {
///         x + y
///     }
/// };
/// assert_eq!(unsafe { ADDER_BLOCK.call((5, 7)) }, 12);
/// ```
///
/// ```
/// use block2::global_block;
/// global_block! {
///     pub static MUTATING_BLOCK = |x: &mut i32| {
///         *x = *x + 42;
///     }
/// };
/// let mut x = 5;
/// unsafe { MUTATING_BLOCK.call((&mut x,)) };
/// assert_eq!(x, 47);
/// ```
///
/// The following does not compile because [`Box`] is not [`Encode`]:
///
/// ```compile_fail
/// use block2::global_block;
/// global_block! {
///     pub static INVALID_BLOCK = |b: Box<i32>| {}
/// };
/// ```
#[macro_export]
macro_rules! global_block {
    // `||` is parsed as one token
    (
        $(#[$m:meta])*
        $vis:vis static $name:ident = || $(-> $r:ty)? $body:block
    ) => {
        $crate::global_block!($(#[$m])* $vis static $name = |,| $(-> $r)? $body);
    };
    (
        $(#[$m:meta])*
        $vis:vis static $name:ident = |$($a:ident: $t:ty),* $(,)?| $(-> $r:ty)? $body:block
    ) => {
        $(#[$m])*
        #[allow(unused_unsafe)]
        $vis static $name: $crate::GlobalBlock<($($t,)*) $(, $r)?> = unsafe {
            let mut layout = $crate::GlobalBlock::<($($t,)*) $(, $r)?>::__DEFAULT_LAYOUT;
            layout.isa = &$crate::ffi::_NSConcreteGlobalBlock as *const _ as *mut _;
            layout.invoke = Some({
                unsafe extern "C" fn inner(_: *mut $crate::ffi::Block_layout, $($a: $t),*) $(-> $r)? {
                    $body
                }
                let inner: unsafe extern "C" fn(*mut $crate::ffi::Block_layout, $($a: $t),*) $(-> $r)? = inner;

                // TODO: SAFETY
                ::core::mem::transmute(inner)
            });
            $crate::GlobalBlock::from_layout(layout)
        };
    };
}

#[cfg(test)]
mod tests {
    global_block! {
        /// Test comments and visibility
        pub(super) static NOOP_BLOCK = || {}
    }

    global_block! {
        /// Multiple arguments + trailing comma
        #[allow(unused)]
        static BLOCK = |x: i32, y: i32, z: i32, w: i32,| -> i32 {
            x + y + z + w
        }
    }

    #[test]
    fn test_noop_block() {
        unsafe { NOOP_BLOCK.call(()) };
    }

    #[test]
    fn test_defined_in_function() {
        global_block!(
            static MY_BLOCK = || -> i32 {
                42
            }
        );
        assert_eq!(unsafe { MY_BLOCK.call(()) }, 42);
    }
}
