//! Types for statically defined encodings.

use core::fmt;

#[derive(Clone, Copy, Debug, PartialEq)]
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

impl<'a> fmt::Display for Encoding<'a> {
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

#[cfg(test)]
mod tests {
    use std::string::ToString;
    use super::Encoding;

    #[test]
    fn test_array_display() {
        let e = Encoding::Array(12, &Encoding::Int);
        assert_eq!(e.to_string(), "[12i]");
    }

    #[test]
    fn test_pointer_display() {
        let e = Encoding::Pointer(&Encoding::Int);
        assert_eq!(e.to_string(), "^i");
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
    }

    #[test]
    fn test_union_display() {
        let s = Encoding::Union("Onion", &[Encoding::Char, Encoding::Int]);
        assert_eq!(s.to_string(), "(Onion=ci)");
    }
}
