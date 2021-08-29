use core::fmt;

use crate::parse;

/// An Objective-C type encoding.
///
/// For more information, see Apple's documentation:
/// <https://developer.apple.com/library/mac/documentation/Cocoa/Conceptual/ObjCRuntimeGuide/Articles/ocrtTypeEncodings.html>
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Encoding<'a> {
    Char,
    Short,
    Int,
    Long,
    LongLong,
    UChar,
    UShort,
    UInt,
    ULong,
    ULongLong,
    Float,
    Double,
    Bool,
    Void,
    String,
    Object,
    Block,
    Class,
    Sel,
    Unknown,
    BitField(u32),
    Pointer(&'a Encoding<'a>),
    Array(u32, &'a Encoding<'a>),
    Struct(&'a str, &'a [Encoding<'a>]),
    Union(&'a str, &'a [Encoding<'a>]),
}

impl fmt::Display for Encoding<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        use Encoding::*;
        let code = match *self {
            Char      => "c",
            Short     => "s",
            Int       => "i",
            Long      => "l",
            LongLong  => "q",
            UChar     => "C",
            UShort    => "S",
            UInt      => "I",
            ULong     => "L",
            ULongLong => "Q",
            Float     => "f",
            Double    => "d",
            Bool      => "B",
            Void      => "v",
            String    => "*",
            Object    => "@",
            Block     => "@?",
            Class     => "#",
            Sel       => ":",
            Unknown   => "?",
            BitField(b) => {
                return write!(formatter, "b{}", b);
            }
            Pointer(t) => {
                return write!(formatter, "^{}", t);
            }
            Array(len, item) => {
                return write!(formatter, "[{}{}]", len, item);
            }
            Struct(name, fields) => {
                write!(formatter, "{{{}=", name)?;
                for field in fields {
                    fmt::Display::fmt(field, formatter)?;
                }
                return formatter.write_str("}");
            }
            Union(name, members) => {
                write!(formatter, "({}=", name)?;
                for member in members {
                    fmt::Display::fmt(member, formatter)?;
                }
                return formatter.write_str(")");
            }
        };
        formatter.write_str(code)
    }
}

impl PartialEq<str> for Encoding<'_> {
    fn eq(&self, other: &str) -> bool {
        parse::eq_enc(other, self)
    }
}

impl PartialEq<Encoding<'_>> for str {
    fn eq(&self, other: &Encoding) -> bool {
        parse::eq_enc(self, other)
    }
}

#[cfg(test)]
mod tests {
    use std::string::ToString;
    use super::Encoding;

    #[test]
    fn test_array_display() {
        let e = Encoding::Array(12, &Encoding::Int);
        assert_eq!(e.to_string(), "[12i]");
        assert_eq!(&e, "[12i]");
    }

    #[test]
    fn test_pointer_display() {
        let e = Encoding::Pointer(&Encoding::Int);
        assert_eq!(e.to_string(), "^i");
        assert_eq!(&e, "^i");
    }

    #[test]
    fn test_pointer_eq() {
        let i = Encoding::Int;
        let p = Encoding::Pointer(&Encoding::Int);

        assert!(p == p);
        assert!(p != i);
    }

    #[test]
    fn test_int_display() {
        assert_eq!(Encoding::Int.to_string(), "i");
        assert_eq!(&Encoding::Int, "i");
    }

    #[test]
    fn test_eq() {
        let i = Encoding::Int;
        let c = Encoding::Char;

        assert!(i == i);
        assert!(i != c);
    }

    #[test]
    fn test_struct_display() {
        let s = Encoding::Struct("CGPoint", &[Encoding::Char, Encoding::Int]);
        assert_eq!(s.to_string(), "{CGPoint=ci}");
        assert_eq!(&s, "{CGPoint=ci}");
    }

    #[test]
    fn test_struct_eq() {
        let s = Encoding::Struct("CGPoint", &[Encoding::Char, Encoding::Int]);
        assert!(s == s);
        assert!(s != Encoding::Int);
    }

    #[test]
    fn test_union_display() {
        let u = Encoding::Union("Onion", &[Encoding::Char, Encoding::Int]);
        assert_eq!(u.to_string(), "(Onion=ci)");
        assert_eq!(&u, "(Onion=ci)");
    }

    #[test]
    fn test_union_eq() {
        let u = Encoding::Union("Onion", &[Encoding::Char, Encoding::Int]);
        assert!(u == u);
        assert!(u != Encoding::Int);
    }
}
