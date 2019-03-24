use libc::{c_char, c_void};

use crate::Encoding;

/// Types that have an Objective-C type encoding.
///
/// Unsafe because Objective-C will make assumptions about the type (like its
/// size and alignment) from its encoding, so the implementer must verify that
/// the encoding is accurate.
pub unsafe trait Encode {
    /// Returns the Objective-C type encoding for Self.
    const CODE: Encoding<'static>;
}

macro_rules! encode_impls {
    ($($t:ty : $e:ident,)*) => ($(
        unsafe impl Encode for $t {
            const CODE: Encoding<'static> = Encoding::$e;
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
    *mut c_char: String,
    *const c_char: String,
);

unsafe impl Encode for isize {
    #[cfg(target_pointer_width = "32")]
    const CODE: Encoding<'static> = i32::CODE;

    #[cfg(target_pointer_width = "64")]
    const CODE: Encoding<'static> = i64::CODE;
}

unsafe impl Encode for usize {
    #[cfg(target_pointer_width = "32")]
    const CODE: Encoding<'static> = u32::CODE;

    #[cfg(target_pointer_width = "64")]
    const CODE: Encoding<'static> = u64::CODE;
}

unsafe impl Encode for *mut c_void {
    const CODE: Encoding<'static> = Encoding::Pointer(&Encoding::Void);
}

unsafe impl Encode for *const c_void {
    const CODE: Encoding<'static> = Encoding::Pointer(&Encoding::Void);
}

/*
External crates cannot implement Encode for pointers or Optionals, but they
*can* implement it for references. rust-lang/rust#25126

As a workaround, we provide implementations for these types that return the
same encoding as references.
*/
unsafe impl<T: 'static> Encode for *const T where for<'a> &'a T: Encode {
    const CODE: Encoding<'static> = <&T as Encode>::CODE;
}

unsafe impl<T: 'static> Encode for *mut T where for<'a> &'a mut T: Encode {
    const CODE: Encoding<'static> = <&mut T>::CODE;
}

unsafe impl<'a, T: 'a> Encode for Option<&'a T> where &'a T: Encode {
    const CODE: Encoding<'static> = <&T>::CODE;
}

unsafe impl<'a, T: 'a> Encode for Option<&'a mut T> where &'a mut T: Encode {
    const CODE: Encoding<'static> = <&mut T>::CODE;
}
