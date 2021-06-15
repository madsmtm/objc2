use core::ffi::c_void;

use crate::Encoding;

/// Types that have an Objective-C type encoding.
///
/// Unsafe because Objective-C will make assumptions about the type (like its
/// size and alignment) from its encoding, so the implementer must verify that
/// the encoding is accurate.
pub unsafe trait Encode {
    /// Returns the Objective-C type encoding for Self.
    const ENCODING: Encoding<'static>;
}

macro_rules! encode_impls {
    ($($t:ty : $e:ident,)*) => ($(
        unsafe impl Encode for $t {
            const ENCODING: Encoding<'static> = Encoding::$e;
        }
    )*);
}

encode_impls!(
    i8: Char,
    i16: Short,
    i32: Int,
    i64: LongLong,
    u8: UChar,
    u16: UShort,
    u32: UInt,
    u64: ULongLong,
    f32: Float,
    f64: Double,
    bool: Bool,
    (): Void,
    *mut i8: String,
    *const i8: String,
    *mut u8: String,
    *const u8: String,
);

unsafe impl Encode for isize {
    #[cfg(target_pointer_width = "32")]
    const ENCODING: Encoding<'static> = i32::ENCODING;

    #[cfg(target_pointer_width = "64")]
    const ENCODING: Encoding<'static> = i64::ENCODING;
}

unsafe impl Encode for usize {
    #[cfg(target_pointer_width = "32")]
    const ENCODING: Encoding<'static> = u32::ENCODING;

    #[cfg(target_pointer_width = "64")]
    const ENCODING: Encoding<'static> = u64::ENCODING;
}

unsafe impl Encode for *mut c_void {
    const ENCODING: Encoding<'static> = Encoding::Pointer(&Encoding::Void);
}

unsafe impl Encode for *const c_void {
    const ENCODING: Encoding<'static> = Encoding::Pointer(&Encoding::Void);
}

// TODO: Replace this with a const generics implementation when they stabilise (#44580)
macro_rules! slice_encode_impl {
    ($($n:literal),* $(,)?) => {
        $(
            unsafe impl<T: Encode> Encode for [T; $n] {
                const ENCODING: Encoding<'static> = Encoding::Array($n, &<T as Encode>::ENCODING);
            }
        )*
    };
}

slice_encode_impl!(
    1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26,
    27, 28, 29, 30, 31, 32
);


// External crates cannot implement Encode for pointers or [`Option`], but
// they *can* implement it for references. See
// [rust-lang/rust#25126](https://github.com/rust-lang/rust/issues/25126)
//
// So, as a workaround, we provide implementations for these types that return
// the same encoding as references.
//
// Using `?Sized` is safe here because we delegate to other implementations
// (which will verify that the implementation is safe for the unsized type).

unsafe impl<T: ?Sized> Encode for *const T where for<'b> &'b T: Encode {
    const ENCODING: Encoding<'static> = <&T>::ENCODING;
}

unsafe impl<T: ?Sized> Encode for *mut T where for<'b> &'b mut T: Encode {
    const ENCODING: Encoding<'static> = <&mut T>::ENCODING;
}

unsafe impl<'a, T: ?Sized> Encode for Option<&'a T> where for<'b> &'b T: Encode {
    const ENCODING: Encoding<'static> = <&T>::ENCODING;
}

unsafe impl<'a, T: ?Sized> Encode for Option<&'a mut T> where for<'b> &'b mut T: Encode {
    const ENCODING: Encoding<'static> = <&mut T>::ENCODING;
}
