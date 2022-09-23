use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;
use core::fmt;

use crate::helper::{compare_encodings, Helper, NestingLevel};
use crate::Encoding;

/// The boxed version of [`Encoding`].
///
/// This has exactly the same items as `Encoding`, the only difference is in
/// where the contents of the more complex encodings like [`Struct`] are
/// stored.
///
/// In `Encoding`, the data is stored in static memory, while in `EncodingBox`
/// it is stored on the heap. The former allows storing in constants (which is
/// required by the [`Encode`] and [`RefEncode`] traits), while the latter
/// allows dynamically, such as in the case of parsing encodings.
///
/// **This should be considered a _temporary_ restriction**. `Encoding` and
/// `EncodingBox` will become equivalent once heap allocation in constants
/// is possible.
///
/// [`Struct`]: Self::Struct
/// [`Encode`]: crate::Encode
/// [`RefEncode`]: crate::RefEncode
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive] // Maybe we're missing some encodings?
pub enum EncodingBox {
    /// Same as [`Encoding::Char`].
    Char,
    /// Same as [`Encoding::Short`].
    Short,
    /// Same as [`Encoding::Int`].
    Int,
    /// Same as [`Encoding::Long`].
    Long,
    /// Same as [`Encoding::LongLong`].
    LongLong,
    /// Same as [`Encoding::UChar`].
    UChar,
    /// Same as [`Encoding::UShort`].
    UShort,
    /// Same as [`Encoding::UInt`].
    UInt,
    /// Same as [`Encoding::ULong`].
    ULong,
    /// Same as [`Encoding::ULongLong`].
    ULongLong,
    /// Same as [`Encoding::Float`].
    Float,
    /// Same as [`Encoding::Double`].
    Double,
    /// Same as [`Encoding::LongDouble`].
    LongDouble,
    /// Same as [`Encoding::FloatComplex`].
    FloatComplex,
    /// Same as [`Encoding::DoubleComplex`].
    DoubleComplex,
    /// Same as [`Encoding::LongDoubleComplex`].
    LongDoubleComplex,
    /// Same as [`Encoding::Bool`].
    Bool,
    /// Same as [`Encoding::Void`].
    Void,
    /// Same as [`Encoding::String`].
    String,
    /// Same as [`Encoding::Object`].
    Object,
    /// Same as [`Encoding::Block`].
    Block,
    /// Same as [`Encoding::Class`].
    Class,
    /// Same as [`Encoding::Sel`].
    Sel,
    /// Same as [`Encoding::Unknown`].
    Unknown,
    /// Same as [`Encoding::BitField`].
    BitField(u8, Box<Self>),
    /// Same as [`Encoding::Pointer`].
    Pointer(Box<Self>),
    /// Same as [`Encoding::Atomic`].
    Atomic(Box<Self>),
    /// Same as [`Encoding::Array`].
    Array(usize, Box<Self>),
    /// Same as [`Encoding::Struct`].
    Struct(String, Vec<Self>),
    /// Same as [`Encoding::Union`].
    Union(String, Vec<Self>),
}

impl EncodingBox {
    /// Same as [`Encoding::C_LONG`].
    pub const C_LONG: Self = match Encoding::C_LONG {
        Encoding::Long => Self::Long,
        Encoding::LongLong => Self::LongLong,
        _ => unreachable!(),
    };

    /// Same as [`Encoding::C_ULONG`].
    pub const C_ULONG: Self = match Encoding::C_ULONG {
        Encoding::ULong => Self::ULong,
        Encoding::ULongLong => Self::ULongLong,
        _ => unreachable!(),
    };
}

/// Same formatting as [`Encoding`]'s `Display` implementation.
impl fmt::Display for EncodingBox {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Helper::from_box(self).display_fmt(f, NestingLevel::new())
    }
}

impl PartialEq<Encoding> for EncodingBox {
    fn eq(&self, other: &Encoding) -> bool {
        compare_encodings(self, other, NestingLevel::new(), true)
    }
}

impl PartialEq<EncodingBox> for Encoding {
    fn eq(&self, other: &EncodingBox) -> bool {
        other.eq(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::string::ToString;
    use alloc::vec;

    #[test]
    fn eq_encodings() {
        let enc1 = Encoding::Char;
        let enc2 = EncodingBox::Char;
        let enc3 = EncodingBox::String;
        assert_eq!(enc1, enc2);
        assert_ne!(enc1, enc3);
    }

    #[test]
    fn eq_complex_encodings() {
        let enc1 = Encoding::Atomic(&Encoding::Struct(
            "test",
            &[Encoding::Array(2, &Encoding::Int)],
        ));
        let enc2 = EncodingBox::Atomic(Box::new(EncodingBox::Struct(
            "test".to_string(),
            vec![EncodingBox::Array(2, Box::new(EncodingBox::Int))],
        )));
        let enc3 = EncodingBox::Atomic(Box::new(EncodingBox::Struct(
            "test".to_string(),
            vec![EncodingBox::Array(2, Box::new(EncodingBox::Char))],
        )));
        assert_eq!(enc1, enc2);
        assert_ne!(enc1, enc3);
    }
}
