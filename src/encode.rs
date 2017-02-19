use libc::{c_char, c_void};

use Encoding;
use encoding::{Pointer, Primitive};

/// Types that have an Objective-C type encoding.
///
/// Unsafe because Objective-C will make assumptions about the type (like its
/// size and alignment) from its encoding, so the implementer must verify that
/// the encoding is accurate.
pub unsafe trait Encode {
    type Encoding: Encoding;

    /// Returns the Objective-C type encoding for Self.
    fn encode() -> Self::Encoding;
}

macro_rules! encode_impls {
    ($($t:ty : $e:ident,)*) => ($(
        unsafe impl Encode for $t {
            type Encoding = Primitive;

            fn encode() -> Primitive { Primitive::$e }
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
    type Encoding = Primitive;

    #[cfg(target_pointer_width = "32")]
    fn encode() -> Primitive { i32::encode() }

    #[cfg(target_pointer_width = "64")]
    fn encode() -> Primitive { i64::encode() }
}

unsafe impl Encode for usize {
    type Encoding = Primitive;

    #[cfg(target_pointer_width = "32")]
    fn encode() -> Primitive { u32::encode() }

    #[cfg(target_pointer_width = "64")]
    fn encode() -> Primitive { u64::encode() }
}

unsafe impl Encode for *mut c_void {
    type Encoding = Pointer<Primitive>;

    fn encode() -> Self::Encoding { Pointer::new(Primitive::Void) }
}

unsafe impl Encode for *const c_void {
    type Encoding = Pointer<Primitive>;

    fn encode() -> Self::Encoding { Pointer::new(Primitive::Void) }
}
