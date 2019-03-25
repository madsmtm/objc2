use libc::{c_char, c_void};

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
    *mut c_char: String,
    *const c_char: String,
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

/*
External crates cannot implement Encode for pointers or Optionals, but they
*can* implement it for references. rust-lang/rust#25126

As a workaround, we provide implementations for these types that return the
same encoding as references.
*/
unsafe impl<T: 'static> Encode for *const T where for<'a> &'a T: Encode {
    const ENCODING: Encoding<'static> = <&T as Encode>::ENCODING;
}

unsafe impl<T: 'static> Encode for *mut T where for<'a> &'a mut T: Encode {
    const ENCODING: Encoding<'static> = <&mut T>::ENCODING;
}

unsafe impl<'a, T: 'a> Encode for Option<&'a T> where &'a T: Encode {
    const ENCODING: Encoding<'static> = <&T>::ENCODING;
}

unsafe impl<'a, T: 'a> Encode for Option<&'a mut T> where &'a mut T: Encode {
    const ENCODING: Encoding<'static> = <&mut T>::ENCODING;
}
