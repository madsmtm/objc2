use core::ffi::c_void;
use core::mem::{ManuallyDrop, MaybeUninit};
use core::num::{
    NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize, NonZeroU16, NonZeroU32,
    NonZeroU64, NonZeroU8, NonZeroUsize, Wrapping,
};
use core::ptr::NonNull;
use core::sync::atomic;

use crate::Encoding;

/// Types that have an Objective-C type-encoding.
///
/// Usually you will want to implement [`RefEncode`] as well.
///
/// If your type is an opaque type you should not need to implement this;
/// there you will only need [`RefEncode`].
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
/// # Examples
///
/// Implementing for a struct:
///
/// ```
/// # use objc2_encode::{Encode, Encoding, RefEncode};
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
    /// # use objc2_encode::{Encoding, RefEncode};
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
    /// # use objc2_encode::{Encode, Encoding, RefEncode};
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

// TODO: Implement for `PhantomData` and `PhantomPinned`?

/// Simple helper for implementing [`Encode`].
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

/// To allow usage as the return type of generic functions.
///
/// You should not rely on this encoding to exist for any other purpose (since
/// `()` is not FFI-safe)!
// TODO: Figure out a way to remove this - maybe with a `EncodeReturn` trait?
unsafe impl Encode for () {
    const ENCODING: Encoding = Encoding::Void;
}

// UI tests of this is too brittle.
#[cfg(doctest)]
/// ```
/// use objc2_encode::Encode;
/// <()>::ENCODING; // TODO: Make this fail as well
/// ```
/// ```should_fail
/// use core::ffi::c_void;
/// use objc2_encode::Encode;
/// <c_void>::ENCODING;
/// ```
/// ```should_fail
/// use objc2_encode::Encode;
/// <*const ()>::ENCODING;
/// ```
/// ```should_fail
/// use core::ffi::c_void;
/// use objc2_encode::Encode;
/// <&c_void>::ENCODING;
/// ```
extern "C" {}

/// Using this directly is heavily discouraged, since the type of BOOL differs
/// across platforms.
///
/// Use `objc2::runtime::Bool::ENCODING` instead.
unsafe impl Encode for bool {
    const ENCODING: Encoding = Encoding::Bool;
}

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

/// Simple helper for implementing [`RefEncode`].
macro_rules! pointer_refencode_impl {
    ($($t:ty),*) => ($(
        unsafe impl RefEncode for $t {
            const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
        }
    )*);
}

pointer_refencode_impl!(bool, i16, i32, i64, isize, u16, u32, u64, usize, f32, f64);

/// Pointers to [`i8`] use the special [`Encoding::String`] encoding.
unsafe impl RefEncode for i8 {
    const ENCODING_REF: Encoding = Encoding::String;
}

/// Pointers to [`u8`] use the special [`Encoding::String`] encoding.
unsafe impl RefEncode for u8 {
    const ENCODING_REF: Encoding = Encoding::String;
}

/// Simple helper for implementing [`Encode`] for nonzero integer types.
macro_rules! encode_impls_nonzero {
    ($($nonzero:ident => $type:ty,)*) => ($(
        unsafe impl Encode for $nonzero {
            const ENCODING: Encoding = <$type>::ENCODING;
        }

        unsafe impl Encode for Option<$nonzero> {
            const ENCODING: Encoding = <$type>::ENCODING;
        }

        unsafe impl RefEncode for $nonzero {
            const ENCODING_REF: Encoding = <$type>::ENCODING_REF;
        }

        unsafe impl RefEncode for Option<$nonzero> {
            const ENCODING_REF: Encoding = <$type>::ENCODING_REF;
        }
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

/// Simple helper for implementing for atomic types.
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
    AtomicBool => bool,
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

/// [`Encode`] is implemented manually for `*const c_void`, instead of
/// implementing [`RefEncode`], to discourage creating `&c_void`.
unsafe impl Encode for *const c_void {
    const ENCODING: Encoding = Encoding::Pointer(&Encoding::Void);
}

unsafe impl RefEncode for *const c_void {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [`Encode`] is implemented manually for `*mut c_void`, instead of
/// implementing [`RefEncode`], to discourage creating `&mut c_void`.
unsafe impl Encode for *mut c_void {
    const ENCODING: Encoding = Encoding::Pointer(&Encoding::Void);
}

unsafe impl RefEncode for *mut c_void {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

unsafe impl<T: Encode, const LENGTH: usize> Encode for [T; LENGTH] {
    const ENCODING: Encoding = Encoding::Array(LENGTH, &T::ENCODING);
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

    // The fact that this has `repr(no_niche)` has no effect on us, since we
    // don't implement `Encode` generically over `Option`.
    // (e.g. an `Option<UnsafeCell<&u8>>` impl is not available).
    // The inner field is not public, so may not be stable.
    // TODO: UnsafeCell<T>,

    // The inner field is not public, so may not be safe.
    // TODO: Pin<T>,

    // SAFETY: Guaranteed to have the same size, alignment, and ABI as `T`.
    MaybeUninit<T>,

    // SAFETY: Guaranteed to have the same layout and ABI as `T`.
    Wrapping<T>,

    // It might have requirements that would disourage this impl?
    // TODO: Cell<T>

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

        unsafe impl<'a, T: RefEncode + ?Sized> $x for Option<&'a T> {
            const $c: Encoding = $e;
        }

        unsafe impl<'a, T: RefEncode + ?Sized> $x for Option<&'a mut T> {
            const $c: Encoding = $e;
        }

        unsafe impl<T: RefEncode + ?Sized> $x for Option<NonNull<T>> {
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

/// Helper for implementing [`Encode`]/[`RefEncode`] for function pointers
/// whoose arguments implement [`Encode`].
///
/// Ideally we'd implement it for all function pointers, but due to coherence
/// issues, see <https://github.com/rust-lang/rust/issues/56105>, function
/// pointers that are higher-ranked over lifetimes don't get implemented.
///
/// We could fix it by adding those impls and allowing `coherence_leak_check`,
/// but it would have to be done for _all_ references, `Option<&T>` and such as
/// well. So trying to do it quickly requires generating a polynomial amount of
/// implementations, which IMO is overkill for such a small issue.
///
/// Using `?Sized` is probably not safe here because C functions can only take
/// and return items with a known size.
macro_rules! encode_fn_pointer_impl {
    (@ $FnTy: ty, $($Arg: ident),*) => {
        unsafe impl<Ret: Encode, $($Arg: Encode),*> Encode for $FnTy {
            const ENCODING: Encoding = Encoding::Pointer(&Encoding::Unknown);
        }
        unsafe impl<Ret: Encode, $($Arg: Encode),*> RefEncode for $FnTy {
            const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
        }

        unsafe impl<Ret: Encode, $($Arg: Encode),*> Encode for Option<$FnTy> {
            const ENCODING: Encoding = Encoding::Pointer(&Encoding::Unknown);
        }
        unsafe impl<Ret: Encode, $($Arg: Encode),*> RefEncode for Option<$FnTy> {
            const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
        }
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

mod private {
    pub trait Sealed {}
}

/// Types that represent an ordered group of function arguments, where each
/// argument has an Objective-C type-encoding.
///
/// This is implemented for tuples of up to 12 arguments, where each argument
/// implements [`Encode`]. It is primarily used to make generic code easier.
///
/// Note that tuples themselves don't implement [`Encode`] directly because
/// they're not FFI-safe!
///
/// # Safety
///
/// This is a sealed trait, and should not need to be implemented. Open an
/// issue if you know a use-case where this restrition should be lifted!
pub unsafe trait EncodeArguments: private::Sealed {
    /// The encodings for the arguments.
    const ENCODINGS: &'static [Encoding];
}

macro_rules! encode_args_impl {
    ($($Arg: ident),*) => {
        impl<$($Arg: Encode),*> private::Sealed for ($($Arg,)*) {}

        unsafe impl<$($Arg: Encode),*> EncodeArguments for ($($Arg,)*) {
            const ENCODINGS: &'static [Encoding] = &[
                $($Arg::ENCODING),*
            ];
        }
    };
}

encode_args_impl!();
encode_args_impl!(A);
encode_args_impl!(A, B);
encode_args_impl!(A, B, C);
encode_args_impl!(A, B, C, D);
encode_args_impl!(A, B, C, D, E);
encode_args_impl!(A, B, C, D, E, F);
encode_args_impl!(A, B, C, D, E, F, G);
encode_args_impl!(A, B, C, D, E, F, G, H);
encode_args_impl!(A, B, C, D, E, F, G, H, I);
encode_args_impl!(A, B, C, D, E, F, G, H, I, J);
encode_args_impl!(A, B, C, D, E, F, G, H, I, J, K);
encode_args_impl!(A, B, C, D, E, F, G, H, I, J, K, L);

#[cfg(test)]
mod tests {
    use super::atomic::*;
    use super::*;

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
        // TODO: Remove this
        assert_eq!(<()>::ENCODING, Encoding::Void);
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

        // assert_eq!(<UnsafeCell<u8>>::ENCODING, u8::ENCODING);
        // assert_eq!(<Pin<u8>>::ENCODING, u8::ENCODING);
        assert_eq!(<MaybeUninit<u8>>::ENCODING, u8::ENCODING);
        assert_eq!(<Wrapping<u8>>::ENCODING, u8::ENCODING);

        // Shouldn't compile
        // assert_eq!(<Option<UnsafeCell<&u8>>>::ENCODING, <&u8>::ENCODING);
    }

    #[test]
    fn test_extern_fn_pointer() {
        assert_eq!(
            <extern "C" fn()>::ENCODING,
            Encoding::Pointer(&Encoding::Unknown)
        );
        assert_eq!(
            <extern "C" fn(x: ()) -> ()>::ENCODING,
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
        extern "C" fn my_fn4<'a, 'b>(x: &'a u8, _y: &'b i32) -> &'a u8 {
            x
        }

        impls_encode(my_fn1 as extern "C" fn(_));
        impls_encode(my_fn2 as extern "C" fn(_, _));
        impls_encode(my_fn3 as extern "C" fn(_) -> _);
        impls_encode(my_fn4 as extern "C" fn(_, _) -> _);
    }

    #[test]
    fn test_encode_arguments() {
        assert!(<()>::ENCODINGS.is_empty());
        assert_eq!(<(i8,)>::ENCODINGS, &[i8::ENCODING]);
        assert_eq!(<(i8, u32)>::ENCODINGS, &[i8::ENCODING, u32::ENCODING]);
    }
}
