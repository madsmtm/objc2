use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;
use core::fmt;
use core::str::FromStr;

use crate::helper::{compare_encodings, Helper, NestingLevel};
use crate::parse::{ParseError, Parser};
use crate::Encoding;

/// The boxed version of [`Encoding`].
///
/// This has exactly the same items as `Encoding`, the only difference is in
/// where the contents of the more complex encodings like [`Struct`] are
/// stored.
///
/// In `Encoding`, the data is stored in static memory, while in `EncodingBox`
/// it is stored on the heap. The former allows storing in constants (which is
/// required by the `objc2::encode::Encode` and `objc2::encode::RefEncode`
/// traits), while the latter allows dynamic creation, such as in the case of
/// parsing encodings.
///
/// **This should be considered a _temporary_ restriction**. `Encoding` and
/// `EncodingBox` will become equivalent once heap allocation in constants
/// is possible.
///
/// [`Struct`]: Self::Struct
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
    BitField(u8, Option<Box<(u64, Self)>>),
    /// Same as [`Encoding::Pointer`].
    Pointer(Box<Self>),
    /// Same as [`Encoding::Atomic`].
    Atomic(Box<Self>),
    /// Same as [`Encoding::Array`].
    Array(u64, Box<Self>),
    /// Same as [`Encoding::Struct`].
    Struct(String, Option<Vec<Self>>),
    /// Same as [`Encoding::Union`].
    Union(String, Option<Vec<Self>>),
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

    /// Parse and comsume an encoding from the start of a string.
    ///
    /// This is can be used to parse concatenated encodings, such as those
    /// returned by `method_getTypeEncoding`.
    ///
    /// [`from_str`][Self::from_str] is simpler, use that instead if you can.
    pub fn from_start_of_str(s: &mut &str) -> Result<Self, ParseError> {
        let mut parser = Parser::new(s);
        parser.strip_leading_qualifiers();

        match parser.parse_encoding() {
            Err(err) => Err(ParseError::new(parser, err)),
            Ok(encoding) => {
                let remaining = parser.remaining();
                *s = remaining;

                Ok(encoding)
            }
        }
    }
}

/// Same formatting as [`Encoding`]'s `Display` implementation.
impl fmt::Display for EncodingBox {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Helper::from_box(self, NestingLevel::new()))
    }
}

impl PartialEq<Encoding> for EncodingBox {
    fn eq(&self, other: &Encoding) -> bool {
        compare_encodings(self, NestingLevel::new(), other, NestingLevel::new(), true)
    }
}

impl PartialEq<EncodingBox> for Encoding {
    fn eq(&self, other: &EncodingBox) -> bool {
        other.eq(self)
    }
}

impl FromStr for EncodingBox {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parser = Parser::new(s);
        parser.strip_leading_qualifiers();

        parser
            .parse_encoding()
            .and_then(|enc| parser.expect_empty().map(|()| enc))
            .map_err(|err| ParseError::new(parser, err))
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
            Some(vec![EncodingBox::Array(2, Box::new(EncodingBox::Int))]),
        )));
        let enc3 = EncodingBox::Atomic(Box::new(EncodingBox::Struct(
            "test".to_string(),
            Some(vec![EncodingBox::Array(2, Box::new(EncodingBox::Char))]),
        )));
        assert_eq!(enc1, enc2);
        assert_ne!(enc1, enc3);
    }

    #[test]
    fn ne_struct_with_without_fields() {
        let enc1 = EncodingBox::Struct("test".to_string(), Some(vec![EncodingBox::Char]));
        let enc2 = EncodingBox::Struct("test".to_string(), None);
        const ENC3A: Encoding = Encoding::Struct("test", &[Encoding::Char]);
        assert_ne!(enc1, enc2);
        assert!(ENC3A.equivalent_to_box(&enc1));
        assert!(!ENC3A.equivalent_to_box(&enc2));

        let enc1 = EncodingBox::Pointer(Box::new(enc1));
        let enc2 = EncodingBox::Pointer(Box::new(enc2));
        const ENC3B: Encoding = Encoding::Pointer(&ENC3A);
        assert_ne!(enc1, enc2);
        assert!(ENC3B.equivalent_to_box(&enc1));
        assert!(!ENC3B.equivalent_to_box(&enc2));

        let enc1 = EncodingBox::Pointer(Box::new(enc1));
        let enc2 = EncodingBox::Pointer(Box::new(enc2));
        const ENC3C: Encoding = Encoding::Pointer(&ENC3B);
        assert_ne!(enc1, enc2);
        assert!(ENC3C.equivalent_to_box(&enc1));
        assert!(ENC3C.equivalent_to_box(&enc2), "now they're equivalent");
    }

    #[test]
    fn parse_atomic_struct() {
        let expected = EncodingBox::Atomic(Box::new(EncodingBox::Atomic(Box::new(
            EncodingBox::Struct("a".into(), Some(Vec::new())),
        ))));
        let actual = EncodingBox::from_str("AA{a=}").unwrap();
        assert_eq!(expected, actual);
        assert_eq!(expected.to_string(), "AA{a}");

        let expected = EncodingBox::Atomic(Box::new(EncodingBox::Atomic(Box::new(
            EncodingBox::Struct("a".into(), None),
        ))));
        let actual = EncodingBox::from_str("AA{a}").unwrap();
        assert_eq!(expected, actual);
        assert_eq!(expected.to_string(), "AA{a}");
    }

    #[test]
    fn parse_part_of_string() {
        let mut s = "{a}cb0i16";

        let expected = EncodingBox::Struct("a".into(), None);
        let actual = EncodingBox::from_start_of_str(&mut s).unwrap();
        assert_eq!(expected, actual);

        let expected = EncodingBox::Char;
        let actual = EncodingBox::from_start_of_str(&mut s).unwrap();
        assert_eq!(expected, actual);

        let expected = EncodingBox::BitField(16, Some(Box::new((0, EncodingBox::Int))));
        let actual = EncodingBox::from_start_of_str(&mut s).unwrap();
        assert_eq!(expected, actual);

        assert_eq!(s, "");
    }
}
