use core::ffi::c_void;

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
/// The type must be FFI-safe, see the [nomicon on other `repr`s][reprs].
///
/// Objective-C will make assumptions about the type (like its size and
/// alignment) from its encoding, so the implementer must verify that the
/// encoding is accurate.
///
/// Concretely, [`Self::ENCODING`] must match the result of running `@encode`
/// in Objective-C with the type in question.
///
/// # Examples
///
/// Implementing for a struct:
///
/// ```
/// # use objc_encode::{Encode, Encoding, RefEncode};
/// # use core::ffi::c_void;
/// #
/// #[repr(C)]
/// struct MyType {
///     a: i32,
///     b: bool,
///     c: *const c_void,
/// }
///
/// unsafe impl Encode for MyType {
///     const ENCODING: Encoding<'static> = Encoding::Struct(
///         // The name of the type that Objective-C sees.
///         "MyType",
///         &[
///             // Delegate to field's implementations.
///             // The order is the same as in the definition.
///             i32::ENCODING,
///             bool::ENCODING,
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
    const ENCODING: Encoding<'static>;
}

/// Types whoose references has an Objective-C type-encoding.
///
/// Implementing this for `T` provides [`Encode`] implementations for:
/// - `*const T`
/// - `*mut T`
/// - `&T`
/// - `&mut T`
/// - `Option<&T>`
/// - `Option<&mut T>`
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
/// Objective-C will make assumptions about the type (like its size and
/// alignment) from its encoding, so the implementer must verify that the
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
    /// # use objc_encode::{Encoding, RefEncode};
    /// # #[repr(C)]
    /// # struct MyObject {
    /// #     _priv: [u8; 0],
    /// # }
    /// # unsafe impl RefEncode for MyObject {
    /// const ENCODING_REF: Encoding<'static> = Encoding::Object;
    /// # }
    /// ```
    ///
    /// Or as a pointer to the type, delegating the rest to the [`Encode`]
    /// implementation:
    /// ```
    /// # use objc_encode::{Encode, Encoding, RefEncode};
    /// # #[repr(transparent)]
    /// # struct MyType(i32);
    /// # unsafe impl Encode for MyType {
    /// #     const ENCODING: Encoding<'static> = i32::ENCODING;
    /// # }
    /// # unsafe impl RefEncode for MyType {
    /// const ENCODING_REF: Encoding<'static> = Encoding::Pointer(&Self::ENCODING);
    /// # }
    /// ```
    const ENCODING_REF: Encoding<'static>;
}

/// Simple helper for implementing [`Encode`].
macro_rules! encode_impls {
    ($($t:ty => $e:ident,)*) => ($(
        unsafe impl Encode for $t {
            const ENCODING: Encoding<'static> = Encoding::$e;
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
    bool => Bool,
    () => Void,
    *mut i8 => String,
    *const i8 => String,
    *mut u8 => String,
    *const u8 => String,
);

macro_rules! encode_impls_size {
    ($($t:ty => ($t16:ty, $t32:ty, $t64:ty),)*) => ($(
        #[doc = concat!("The encoding of [`", stringify!($t), "`] varies based on the target pointer width.")]
        unsafe impl Encode for $t {
            #[cfg(target_pointer_width = "16")]
            const ENCODING: Encoding<'static> = <$t16>::ENCODING;
            #[cfg(target_pointer_width = "32")]
            const ENCODING: Encoding<'static> = <$t32>::ENCODING;
            #[cfg(target_pointer_width = "64")]
            const ENCODING: Encoding<'static> = <$t64>::ENCODING;
        }
    )*);
}

encode_impls_size!(
    isize => (i16, i32, i64),
    usize => (u16, u32, u64),
);

/// Simple helper for implementing [`Encode`] for integer types.
macro_rules! encode_impls_nonzero {
    ($($nonzero:ident => $type:ty,)*) => ($(
        unsafe impl Encode for core::num::$nonzero {
            const ENCODING: Encoding<'static> = <$type>::ENCODING;
        }

        unsafe impl Encode for Option<core::num::$nonzero> {
            const ENCODING: Encoding<'static> = <$type>::ENCODING;
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

// Note: I'm not sure atomic integers would be safe, since they might need the
// Objective-C runtime to insert proper memory fences and ordering stuff?

/// [`Encode`] is implemented manually for `*const c_void`, instead of
/// implementing [`RefEncode`], to discourage creating `&c_void`.
unsafe impl Encode for *const c_void {
    const ENCODING: Encoding<'static> = Encoding::Pointer(&Encoding::Void);
}

/// [`Encode`] is implemented manually for `*mut c_void`, instead of
/// implementing [`RefEncode`], to discourage creating `&mut c_void`.
unsafe impl Encode for *mut c_void {
    const ENCODING: Encoding<'static> = Encoding::Pointer(&Encoding::Void);
}

unsafe impl<T: Encode, const LENGTH: usize> Encode for [T; LENGTH] {
    const ENCODING: Encoding<'static> = Encoding::Array(LENGTH, &T::ENCODING);
}

/// Helper for implementing `Encode`/`RefEncode` for pointers to types that
/// implement `RefEncode`.
///
/// Using `?Sized` is safe here because we delegate to other implementations
/// (which will verify that the implementation is safe for the unsized type).
macro_rules! encode_pointer_impls {
    (unsafe impl<T: RefEncode> $x:ident for &$t:ident {
        const $c:ident = $e:expr;
    }) => (
        unsafe impl<$t: RefEncode + ?Sized> $x for *const $t {
            const $c: Encoding<'static> = $e;
        }

        unsafe impl<$t: RefEncode + ?Sized> $x for *mut $t {
            const $c: Encoding<'static> = $e;
        }

        unsafe impl<'a, $t: RefEncode + ?Sized> $x for &'a $t {
            const $c: Encoding<'static> = $e;
        }

        unsafe impl<'a, $t: RefEncode + ?Sized> $x for &'a mut $t {
            const $c: Encoding<'static> = $e;
        }

        unsafe impl<'a, $t: RefEncode + ?Sized> $x for Option<&'a $t> {
            const $c: Encoding<'static> = $e;
        }

        unsafe impl<'a, $t: RefEncode + ?Sized> $x for Option<&'a mut $t> {
            const $c: Encoding<'static> = $e;
        }
    );
}

// Implement `Encode` for types that are `RefEncode`.
//
// This allows users to implement `Encode` for custom types that have a
// specific encoding as a pointer, instead of having to implement it for each
// pointer-like type in turn.
encode_pointer_impls!(
    unsafe impl<T: RefEncode> Encode for &T {
        const ENCODING = T::ENCODING_REF;
    }
);

// Implement `RefEncode` for pointers to types that are `RefEncode`.
//
// This implements `Encode` for pointers to pointers (to pointers, and so on),
// which would otherwise be very cumbersome to do manually.
encode_pointer_impls!(
    unsafe impl<T: RefEncode> RefEncode for &T {
        const ENCODING_REF = Encoding::Pointer(&T::ENCODING_REF);
    }
);
