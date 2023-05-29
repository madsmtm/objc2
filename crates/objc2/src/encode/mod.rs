//! # Support for type-encodings.
//!
//! This module contains traits for annotating types that has an Objective-C
//! type-encoding: Specifically [`Encode`] for structs/numeric types and
//! [`RefEncode`] for references.
//!
//! Additionally, this exports the [`Encoding`] and [`EncodingBox`] types from
//! [`objc2-encode`][objc2_encode], see that crate for a few more details on
//! what Objective-C type-encodings are.
//!
//!
//! ## Examples
//!
//! Implementing [`Encode`] and [`RefEncode`] for a custom type:
//!
//! ```
//! use objc2::encode::{Encode, Encoding, RefEncode};
//!
//! #[repr(C)]
//! struct MyStruct {
//!     a: f32, // float
//!     b: i16, // int16_t
//! }
//!
//! unsafe impl Encode for MyStruct {
//!     const ENCODING: Encoding = Encoding::Struct(
//!         "MyStruct", // Must use the same name as defined in C header files
//!         &[
//!             f32::ENCODING, // Same as Encoding::Float
//!             i16::ENCODING, // Same as Encoding::Short
//!         ],
//!     );
//! }
//!
//! // @encode(MyStruct) -> "{MyStruct=fs}"
//! assert!(MyStruct::ENCODING.equivalent_to_str("{MyStruct=fs}"));
//!
//! unsafe impl RefEncode for MyStruct {
//!     const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
//! }
//!
//! // @encode(MyStruct*) -> "^{MyStruct=fs}"
//! assert!(MyStruct::ENCODING_REF.equivalent_to_str("^{MyStruct=fs}"));
//! ```
//!
//! Implementing [`Encode`] for a few core-graphics types.
//!
//! Note that these are available in `icrate`, so the implementation here is
//! mostly for demonstration.
//!
//! ```
#![doc = include_str!("../../examples/encode_core_graphics.rs")]
//! ```
//!
//! Implementing [`Encode`] and [`RefEncode`] for a transparent newtype.
//!
//! ```
#![doc = include_str!("../../examples/encode_nsuinteger.rs")]
//! ```
//!
//! Implementing [`RefEncode`] for an object, in this case `NSString`.
//!
//! ```
#![doc = include_str!("../../examples/encode_nsstring.rs")]
//! ```
//!
//! Implementing [`RefEncode`] for a type where you don't necessarily know
//! about the exact internals / the internals are not representable in Rust.
//!
//! ```
#![doc = include_str!("../../examples/encode_opaque_type.rs")]
//! ```

use core::cell::{Cell, UnsafeCell};
use core::ffi::c_void;
use core::mem::{self, ManuallyDrop, MaybeUninit};
use core::num::{
    NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize, NonZeroU16, NonZeroU32,
    NonZeroU64, NonZeroU8, NonZeroUsize, Wrapping,
};
use core::ptr::NonNull;
use core::sync::atomic;

// Intentionally not `#[doc(hidden)]`
pub mod __unstable;

#[doc(inline)]
pub use objc2_encode::{Encoding, EncodingBox, ParseError};

/// Types that have an Objective-C type-encoding.
///
/// Usually you will want to implement [`RefEncode`] as well.
///
/// If your type is an opaque type you should not need to implement this;
/// there you will only need [`RefEncode`].
///
///
/// # Safety
///
/// The type must be FFI-safe, meaning a C-compatible `repr` (`repr(C)`,
/// `repr(u8)`, `repr(transparent)` where the inner types are C-compatible,
/// and so on). See the [nomicon on other `repr`s][reprs].
///
/// Objective-C will make assumptions about the type (like its size, alignment
/// and ABI) from its encoding, so the implementer must verify that the
/// encoding is accurate.
///
/// Concretely, [`Self::ENCODING`] must match the result of running `@encode`
/// in Objective-C with the type in question.
///
/// You should also beware of having [`Drop`] types implement this, since when
/// passed to Objective-C via. `objc2::msg_send!` their destructor will not be
/// called!
///
///
/// # Examples
///
/// Implementing for a struct:
///
/// ```
/// # use objc2::encode::{Encode, Encoding, RefEncode};
/// # use core::ffi::c_void;
/// #
/// #[repr(C)]
/// struct MyType {
///     a: i32,
///     b: f64,
///     c: *const c_void,
/// }
///
/// unsafe impl Encode for MyType {
///     const ENCODING: Encoding = Encoding::Struct(
///         // The name of the type that Objective-C sees.
///         "MyType",
///         &[
///             // Delegate to field's implementations.
///             // The order is the same as in the definition.
///             i32::ENCODING,
///             f64::ENCODING,
///             <*const c_void>::ENCODING,
///         ],
///     );
/// }
///
/// // Note: You would also implement `RefEncode` for this type.
/// ```
///
/// [reprs]: https://doc.rust-lang.org/nomicon/other-reprs.html
pub unsafe trait Encode {
    /// The Objective-C type-encoding for this type.
    const ENCODING: Encoding;
}

/// Types whoose references has an Objective-C type-encoding.
///
/// Implementing this for `T` provides [`Encode`] implementations for:
/// - `*const T`
/// - `*mut T`
/// - `&T`
/// - `&mut T`
/// - `NonNull<T>`
/// - `Option<&T>`
/// - `Option<&mut T>`
/// - `Option<NonNull<T>>`
///
///
/// # Reasoning behind this trait's existence
///
/// External crates cannot implement [`Encode`] for pointers or [`Option`]s
/// containing references, so instead, they can implement this trait.
/// Additionally it would be very cumbersome if every type had to implement
/// [`Encode`] for all possible pointer types.
///
/// Finally, having this trait allows for much cleaner generic code that need
/// to represent types that can be encoded as pointers.
///
///
/// # Safety
///
/// References to the object must be FFI-safe.
///
/// See the nomicon entry on [representing opaque structs][opaque] for
/// information on how to represent objects that you don't know the layout of
/// (or use `extern type` ([RFC-1861]) if you're using nightly).
///
/// Objective-C will make assumptions about the type (like its size, alignment
/// and ABI) from its encoding, so the implementer must verify that the
/// encoding is accurate.
///
/// Concretely, [`Self::ENCODING_REF`] must match the result of running
/// `@encode` in Objective-C with a pointer to the type in question.
///
/// [opaque]: https://doc.rust-lang.org/nomicon/ffi.html#representing-opaque-structs
/// [RFC-1861]: https://rust-lang.github.io/rfcs/1861-extern-types.html
pub unsafe trait RefEncode {
    /// The Objective-C type-encoding for a reference of this type.
    ///
    /// Should be one of [`Encoding::Object`], [`Encoding::Block`],
    /// [`Encoding::Class`], [`Encoding::Pointer`], [`Encoding::Sel`] or
    /// [`Encoding::Unknown`].
    ///
    /// # Examples
    ///
    /// This is usually implemented either as an object pointer:
    /// ```
    /// # use objc2::encode::{Encoding, RefEncode};
    /// # #[repr(C)]
    /// # struct MyObject {
    /// #     _priv: [u8; 0],
    /// # }
    /// # unsafe impl RefEncode for MyObject {
    /// const ENCODING_REF: Encoding = Encoding::Object;
    /// # }
    /// ```
    ///
    /// Or as a pointer to the type, delegating the rest to the [`Encode`]
    /// implementation:
    /// ```
    /// # use objc2::encode::{Encode, Encoding, RefEncode};
    /// # #[repr(transparent)]
    /// # struct MyType(i32);
    /// # unsafe impl Encode for MyType {
    /// #     const ENCODING: Encoding = i32::ENCODING;
    /// # }
    /// # unsafe impl RefEncode for MyType {
    /// const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
    /// # }
    /// ```
    const ENCODING_REF: Encoding;
}

/// A helper trait for types that are encodable inside an [`Option`].
///
/// See [the nomicon][nomicon-nullable] for details on which types uphold this
/// promise.
///
/// This is used to work around the orphan rule, which would normally prevent
/// you from implementing [`Encode`]/[`RefEncode`] for `Option<CustomType>`.
///
/// [nomicon-nullable]: https://doc.rust-lang.org/nightly/nomicon/ffi.html#the-nullable-pointer-optimization
///
///
/// # Safety
///
/// You must ensure that the implemented type `T` has the same layout as
/// `Option<T>`.
///
///
/// # Examples
///
/// ```
/// use objc2::encode::{Encode, Encoding, OptionEncode};
/// use core::ptr::NonNull;
/// use core::ffi::c_void;
///
/// #[repr(transparent)]
/// struct MyBlockType(NonNull<c_void>);
///
/// // SAFETY: `MyBlockType` is meant to represent a pointer to a block
/// unsafe impl Encode for MyBlockType {
///     const ENCODING: Encoding = Encoding::Block;
/// }
///
/// // SAFETY: `MyBlockType` is `repr(transparent)` over `NonNull`, which
/// // means that `Option<MyBlockType>` has the same layout.
/// unsafe impl OptionEncode for MyBlockType {}
///
/// assert_eq!(<Option<MyBlockType>>::ENCODING, MyBlockType::ENCODING);
/// ```
pub unsafe trait OptionEncode {}

// SAFETY: Implementor of `OptionEncode` guarantees this impl is sound
unsafe impl<T: Encode + OptionEncode> Encode for Option<T> {
    const ENCODING: Encoding = {
        if mem::size_of::<T>() != mem::size_of::<Option<T>>() {
            panic!("invalid OptionEncode + Encode implementation");
        }
        T::ENCODING
    };
}

// SAFETY: Implementor of `OptionEncode` guarantees this impl is sound
unsafe impl<T: RefEncode + OptionEncode> RefEncode for Option<T> {
    const ENCODING_REF: Encoding = {
        if mem::size_of::<T>() != mem::size_of::<Option<T>>() {
            panic!("invalid OptionEncode + RefEncode implementation");
        }
        T::ENCODING_REF
    };
}

// TODO: Implement for `PhantomData` and `PhantomPinned`?

/// Helper for implementing [`Encode`].
macro_rules! encode_impls {
    ($($t:ty => $e:ident,)*) => ($(
        unsafe impl Encode for $t {
            const ENCODING: Encoding = Encoding::$e;
        }
    )*);
}

encode_impls!(
    i8 => Char,
    i16 => Short,
    i32 => Int,
    i64 => LongLong,
    u8 => UChar,
    u16 => UShort,
    u32 => UInt,
    u64 => ULongLong,
    f32 => Float,
    f64 => Double,

    // TODO: i128 & u128
    // https://github.com/rust-lang/rust/issues/54341
);

// TODO: Structs in core::arch?

macro_rules! encode_impls_size {
    ($($t:ty => ($t16:ty, $t32:ty, $t64:ty),)*) => ($(
        #[doc = concat!("The encoding of [`", stringify!($t), "`] varies based on the target pointer width.")]
        unsafe impl Encode for $t {
            #[cfg(target_pointer_width = "16")]
            const ENCODING: Encoding = <$t16>::ENCODING;
            #[cfg(target_pointer_width = "32")]
            const ENCODING: Encoding = <$t32>::ENCODING;
            #[cfg(target_pointer_width = "64")]
            const ENCODING: Encoding = <$t64>::ENCODING;
        }
    )*);
}

encode_impls_size!(
    isize => (i16, i32, i64),
    usize => (u16, u32, u64),
);

/// Helper for implementing [`RefEncode`].
macro_rules! pointer_refencode_impl {
    ($($t:ty),*) => ($(
        unsafe impl RefEncode for $t {
            const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
        }
    )*);
}

pointer_refencode_impl!(i16, i32, i64, isize, u16, u32, u64, usize, f32, f64);

/// Pointers to [`i8`] use the special [`Encoding::String`] encoding.
unsafe impl RefEncode for i8 {
    const ENCODING_REF: Encoding = Encoding::String;
}

/// Pointers to [`u8`] use the special [`Encoding::String`] encoding.
unsafe impl RefEncode for u8 {
    const ENCODING_REF: Encoding = Encoding::String;
}

/// Helper for implementing [`Encode`] for nonzero integer types.
macro_rules! encode_impls_nonzero {
    ($($nonzero:ident => $type:ty,)*) => ($(
        unsafe impl Encode for $nonzero {
            const ENCODING: Encoding = <$type>::ENCODING;
        }

        unsafe impl RefEncode for $nonzero {
            const ENCODING_REF: Encoding = <$type>::ENCODING_REF;
        }

        // SAFETY: nonzero types have a NUL niche that is exploited by Option
        unsafe impl OptionEncode for $nonzero {}
    )*);
}

encode_impls_nonzero!(
    NonZeroI8 => i8,
    NonZeroI16 => i16,
    NonZeroI32 => i32,
    NonZeroI64 => i64,
    NonZeroIsize => isize,
    NonZeroU8 => u8,
    NonZeroU16 => u16,
    NonZeroU32 => u32,
    NonZeroU64 => u64,
    NonZeroUsize => usize,
);

/// Helper for implementing for atomic types.
macro_rules! encode_atomic_impls {
    ($(
        $(#[$m:meta])*
        $atomic:ident => $type:ty,
    )*) => ($(
        // SAFETY: C11 `_Atomic` types use compatible synchronization
        // primitives, and the atomic type is guaranteed to have the same
        // in-memory representation as the underlying type.
        $(#[$m])*
        unsafe impl Encode for atomic::$atomic {
            const ENCODING: Encoding = Encoding::Atomic(&<$type>::ENCODING);
        }

        $(#[$m])*
        unsafe impl RefEncode for atomic::$atomic {
            const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
        }
    )*);
}

encode_atomic_impls!(
    #[cfg(target_has_atomic = "8")]
    AtomicI8 => i8,
    #[cfg(target_has_atomic = "8")]
    AtomicU8 => u8,

    #[cfg(target_has_atomic = "16")]
    AtomicI16 => i16,
    #[cfg(target_has_atomic = "16")]
    AtomicU16 => u16,

    #[cfg(target_has_atomic = "32")]
    AtomicI32 => i32,
    #[cfg(target_has_atomic = "32")]
    AtomicU32 => u32,

    #[cfg(target_has_atomic = "64")]
    AtomicI64 => i64,
    #[cfg(target_has_atomic = "64")]
    AtomicU64 => u64,

    // TODO
    // #[cfg(target_has_atomic = "128")]
    // AtomicI128 => i128,
    // #[cfg(target_has_atomic = "128")]
    // AtomicU128 => u128,

    #[cfg(target_has_atomic = "ptr")]
    AtomicIsize => isize,
    #[cfg(target_has_atomic = "ptr")]
    AtomicUsize => usize,
);

// SAFETY: Guaranteed to have the same in-memory representation as `*mut T`.
#[cfg(target_has_atomic = "ptr")]
unsafe impl<T: RefEncode> Encode for atomic::AtomicPtr<T> {
    const ENCODING: Encoding = Encoding::Atomic(&T::ENCODING_REF);
}

#[cfg(target_has_atomic = "ptr")]
unsafe impl<T: RefEncode> RefEncode for atomic::AtomicPtr<T> {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [`Encode`] is implemented manually for `*const c_void`, `*mut c_void` and
/// `NonNull<c_void>`, instead of implementing [`RefEncode`], to discourage
/// creating `&c_void`/`&mut c_void`.
unsafe impl Encode for *const c_void {
    const ENCODING: Encoding = Encoding::Pointer(&Encoding::Void);
}
unsafe impl RefEncode for *const c_void {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
unsafe impl Encode for *mut c_void {
    const ENCODING: Encoding = Encoding::Pointer(&Encoding::Void);
}
unsafe impl RefEncode for *mut c_void {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
unsafe impl Encode for NonNull<c_void> {
    const ENCODING: Encoding = Encoding::Pointer(&Encoding::Void);
}
unsafe impl RefEncode for NonNull<c_void> {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
unsafe impl OptionEncode for NonNull<c_void> {}

unsafe impl<T: Encode, const LENGTH: usize> Encode for [T; LENGTH] {
    const ENCODING: Encoding = Encoding::Array(LENGTH as u64, &T::ENCODING);
}

unsafe impl<T: Encode, const LENGTH: usize> RefEncode for [T; LENGTH] {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

macro_rules! encode_impls_transparent {
    ($($t:ident<T $(: ?$b:ident)?>,)*) => ($(
        unsafe impl<T: Encode $(+ ?$b)?> Encode for $t<T> {
            const ENCODING: Encoding = T::ENCODING;
        }

        unsafe impl<T: RefEncode $(+ ?$b)?> RefEncode for $t<T> {
            const ENCODING_REF: Encoding = T::ENCODING_REF;
        }
    )*);
}

encode_impls_transparent! {
    // SAFETY: Guaranteed to have the same layout as `T`, and is subject to
    // the same layout optimizations as `T`.
    // TODO: With specialization: `impl Encode for ManuallyDrop<Box<T>>`
    ManuallyDrop<T: ?Sized>,

    // SAFETY: Guaranteed to have the same in-memory representation `T`.
    //
    // The fact that this has `repr(no_niche)` has no effect on us, since we
    // don't unconditionally implement `Encode` generically over `Option`.
    // (e.g. an `Option<UnsafeCell<&u8>>` impl is not available).
    UnsafeCell<T: ?Sized>,

    // SAFETY: Guaranteed to have the same layout as `UnsafeCell<T>`.
    Cell<T: ?Sized>,

    // The inner field is not public, so may not be safe.
    // TODO: Pin<T>,

    // SAFETY: Guaranteed to have the same size, alignment, and ABI as `T`.
    MaybeUninit<T>,

    // SAFETY: Guaranteed to have the same layout and ABI as `T`.
    Wrapping<T>,

    // TODO: Types that need to be made repr(transparent) first:
    // - core::cell::Ref?
    // - core::cell::RefCell?
    // - core::cell::RefMut?
    // - core::panic::AssertUnwindSafe<T>
    // TODO: core::num::Saturating when that is stabilized
    // TODO: core::cmp::Reverse?
}

/// Helper for implementing `Encode`/`RefEncode` for pointers to types that
/// implement `RefEncode`.
///
/// Using `?Sized` is safe here because we delegate to other implementations
/// (which will verify that the implementation is safe for the unsized type).
macro_rules! encode_pointer_impls {
    (unsafe impl<T: RefEncode> $x:ident for Pointer<T> {
        const $c:ident = $e:expr;
    }) => (
        unsafe impl<T: RefEncode + ?Sized> $x for *const T {
            const $c: Encoding = $e;
        }

        unsafe impl<T: RefEncode + ?Sized> $x for *mut T {
            const $c: Encoding = $e;
        }

        unsafe impl<'a, T: RefEncode + ?Sized> $x for &'a T {
            const $c: Encoding = $e;
        }

        unsafe impl<'a, T: RefEncode + ?Sized> $x for &'a mut T {
            const $c: Encoding = $e;
        }

        unsafe impl<T: RefEncode + ?Sized> $x for NonNull<T> {
            const $c: Encoding = $e;
        }
    );
}

// Implement `Encode` for types that are `RefEncode`.
//
// This allows users to implement `Encode` for custom types that have a
// specific encoding as a pointer, instead of having to implement it for each
// pointer-like type in turn.
encode_pointer_impls!(
    unsafe impl<T: RefEncode> Encode for Pointer<T> {
        const ENCODING = T::ENCODING_REF;
    }
);

// Implement `RefEncode` for pointers to types that are `RefEncode`.
//
// This implements `Encode` for pointers to pointers (to pointers, and so on),
// which would otherwise be very cumbersome to do manually.
encode_pointer_impls!(
    unsafe impl<T: RefEncode> RefEncode for Pointer<T> {
        const ENCODING_REF = Encoding::Pointer(&T::ENCODING_REF);
    }
);

// SAFETY: References and `NonNull` have a NULL niche
unsafe impl<'a, T: RefEncode + ?Sized> OptionEncode for &'a T {}
unsafe impl<'a, T: RefEncode + ?Sized> OptionEncode for &'a mut T {}
unsafe impl<T: RefEncode + ?Sized> OptionEncode for NonNull<T> {}

/// Helper for implementing [`Encode`]/[`RefEncode`] for function pointers
/// whoose arguments implement [`Encode`].
///
/// Ideally we'd implement it for all function pointers, but due to coherence
/// issues, see <https://github.com/rust-lang/rust/issues/56105>, function
/// pointers that are higher-ranked over lifetimes don't get implemented.
///
/// We could fix it by adding those impls and allowing `coherence_leak_check`,
/// but it would have to be done for _all_ references, `Option<&T>` and such
/// as well. So trying to do it quickly requires generating a polynomial
/// amount of implementations, which IMO is overkill for such a small issue.
///
/// Using `?Sized` is probably not safe here because C functions can only take
/// and return items with a known size.
macro_rules! encode_fn_pointer_impl {
    (@ $FnTy: ty, $($Arg: ident),*) => {
        unsafe impl<Ret: __unstable::EncodeReturn, $($Arg: Encode),*> Encode for $FnTy {
            const ENCODING: Encoding = Encoding::Pointer(&Encoding::Unknown);
        }
        unsafe impl<Ret: __unstable::EncodeReturn, $($Arg: Encode),*> RefEncode for $FnTy {
            const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
        }
        // SAFETY: Function pointers have a NULL niche
        unsafe impl<Ret: __unstable::EncodeReturn, $($Arg: Encode),*> OptionEncode for $FnTy {}
    };
    (# $abi:literal; $($Arg: ident),+) => {
        // Normal functions
        encode_fn_pointer_impl!(@ extern $abi fn($($Arg),+) -> Ret, $($Arg),+ );
        encode_fn_pointer_impl!(@ unsafe extern $abi fn($($Arg),+) -> Ret, $($Arg),+ );
        // Variadic functions
        encode_fn_pointer_impl!(@ extern $abi fn($($Arg),+ , ...) -> Ret, $($Arg),+ );
        encode_fn_pointer_impl!(@ unsafe extern $abi fn($($Arg),+ , ...) -> Ret, $($Arg),+ );
    };
    (# $abi:literal; ) => {
        // No variadic functions with 0 parameters
        encode_fn_pointer_impl!(@ extern $abi fn() -> Ret, );
        encode_fn_pointer_impl!(@ unsafe extern $abi fn() -> Ret, );
    };
    ($($Arg: ident),*) => {
        encode_fn_pointer_impl!(# "C"; $($Arg),*);
        #[cfg(feature = "unstable-c-unwind")]
        encode_fn_pointer_impl!(# "C-unwind"; $($Arg),*);
    };
}

encode_fn_pointer_impl!();
encode_fn_pointer_impl!(A);
encode_fn_pointer_impl!(A, B);
encode_fn_pointer_impl!(A, B, C);
encode_fn_pointer_impl!(A, B, C, D);
encode_fn_pointer_impl!(A, B, C, D, E);
encode_fn_pointer_impl!(A, B, C, D, E, F);
encode_fn_pointer_impl!(A, B, C, D, E, F, G);
encode_fn_pointer_impl!(A, B, C, D, E, F, G, H);
encode_fn_pointer_impl!(A, B, C, D, E, F, G, H, I);
encode_fn_pointer_impl!(A, B, C, D, E, F, G, H, I, J);
encode_fn_pointer_impl!(A, B, C, D, E, F, G, H, I, J, K);
encode_fn_pointer_impl!(A, B, C, D, E, F, G, H, I, J, K, L);

#[cfg(test)]
mod tests {
    use super::*;

    use core::sync::atomic::*;

    #[test]
    fn test_c_string() {
        assert_eq!(i8::ENCODING, Encoding::Char);
        assert_eq!(u8::ENCODING, Encoding::UChar);

        assert_eq!(<*const i8>::ENCODING, Encoding::String);
        assert_eq!(<&u8>::ENCODING, Encoding::String);
        assert_eq!(i8::ENCODING_REF, Encoding::String);
        assert_eq!(i8::ENCODING_REF, Encoding::String);

        assert_eq!(
            <*const *const i8>::ENCODING,
            Encoding::Pointer(&Encoding::String)
        );
        assert_eq!(<&&u8>::ENCODING, Encoding::Pointer(&Encoding::String));
    }

    #[test]
    fn test_i32() {
        assert_eq!(i32::ENCODING, Encoding::Int);
        assert_eq!(<&i32>::ENCODING, Encoding::Pointer(&Encoding::Int));
        assert_eq!(
            <&&i32>::ENCODING,
            Encoding::Pointer(&Encoding::Pointer(&Encoding::Int))
        );
    }

    #[test]
    fn test_atomic() {
        assert_eq!(AtomicI32::ENCODING, Encoding::Atomic(&Encoding::Int));
        assert_eq!(
            AtomicI32::ENCODING_REF,
            Encoding::Pointer(&Encoding::Atomic(&Encoding::Int))
        );
        assert_eq!(
            AtomicPtr::<i32>::ENCODING,
            Encoding::Atomic(&Encoding::Pointer(&Encoding::Int))
        );

        assert_eq!(AtomicI8::ENCODING, Encoding::Atomic(&Encoding::Char));
        assert_eq!(
            AtomicI8::ENCODING_REF,
            Encoding::Pointer(&Encoding::Atomic(&Encoding::Char))
        );
        assert_eq!(
            AtomicPtr::<i8>::ENCODING,
            Encoding::Atomic(&Encoding::String)
        );
    }

    #[test]
    fn test_void() {
        assert_eq!(
            <*const c_void>::ENCODING,
            Encoding::Pointer(&Encoding::Void)
        );
        assert_eq!(
            <&*const c_void>::ENCODING,
            Encoding::Pointer(&Encoding::Pointer(&Encoding::Void))
        );
    }

    #[test]
    fn test_transparent() {
        assert_eq!(<ManuallyDrop<u8>>::ENCODING, u8::ENCODING);
        assert_eq!(<ManuallyDrop<&u8>>::ENCODING, u8::ENCODING_REF);
        assert_eq!(<ManuallyDrop<Option<&u8>>>::ENCODING, u8::ENCODING_REF);
        assert_eq!(<&ManuallyDrop<Option<&u8>>>::ENCODING, <&&u8>::ENCODING);

        assert_eq!(<UnsafeCell<u8>>::ENCODING, u8::ENCODING);
        assert_eq!(<UnsafeCell<&u8>>::ENCODING, <&u8>::ENCODING);
        assert_eq!(<Cell<u8>>::ENCODING, u8::ENCODING);
        assert_eq!(<Cell<&u8>>::ENCODING, <&u8>::ENCODING);
        // assert_eq!(<Pin<u8>>::ENCODING, u8::ENCODING);
        assert_eq!(<MaybeUninit<u8>>::ENCODING, u8::ENCODING);
        assert_eq!(<Wrapping<u8>>::ENCODING, u8::ENCODING);
    }

    #[test]
    fn test_extern_fn_pointer() {
        assert_eq!(
            <extern "C" fn()>::ENCODING,
            Encoding::Pointer(&Encoding::Unknown)
        );
        assert_eq!(
            <extern "C" fn(x: i32) -> u32>::ENCODING,
            Encoding::Pointer(&Encoding::Unknown)
        );
        assert_eq!(
            <Option<unsafe extern "C" fn()>>::ENCODING,
            Encoding::Pointer(&Encoding::Unknown)
        );
        #[cfg(feature = "unstable-c-unwind")]
        assert_eq!(
            <extern "C-unwind" fn()>::ENCODING,
            Encoding::Pointer(&Encoding::Unknown)
        );
    }

    #[test]
    fn test_extern_fn_pointer_elided_lifetime() {
        fn impls_encode<T: Encode>(_x: T) {}

        extern "C" fn my_fn1(_x: &i32) {}
        extern "C" fn my_fn2(_x: &i32, _y: &u8) {}
        extern "C" fn my_fn3(x: &u8) -> &u8 {
            x
        }
        extern "C" fn my_fn4<'a>(x: &'a u8, _y: &i32) -> &'a u8 {
            x
        }

        impls_encode(my_fn1 as extern "C" fn(_));
        impls_encode(my_fn2 as extern "C" fn(_, _));
        impls_encode(my_fn3 as extern "C" fn(_) -> _);
        impls_encode(my_fn4 as extern "C" fn(_, _) -> _);
    }
}
