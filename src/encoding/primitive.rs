use std::fmt;

use super::{Descriptor, Encoding, Never};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Primitive {
    Char,
    Short,
    Int,
    LongLong,
    UChar,
    UShort,
    UInt,
    ULongLong,
    Float,
    Double,
    Bool,
    Void,
    Object,
    Class,
    Sel,
    Unknown,
    BitField(u32),
}

impl Encoding for Primitive {
    type Pointer = Never;
    type Struct = Never;

    fn descriptor(&self) -> Descriptor<Never, Never> {
        Descriptor::Primitive(*self)
    }

    fn eq_encoding<T: ?Sized + Encoding>(&self, other: &T) -> bool {
        if let Descriptor::Primitive(p) = other.descriptor() {
            *self == p
        } else {
            false
        }
    }
}

impl fmt::Display for Primitive {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let code = match *self {
            Primitive::Char      => "c",
            Primitive::Short     => "s",
            Primitive::Int       => "i",
            Primitive::LongLong  => "q",
            Primitive::UChar     => "C",
            Primitive::UShort    => "S",
            Primitive::UInt      => "I",
            Primitive::ULongLong => "Q",
            Primitive::Float     => "f",
            Primitive::Double    => "d",
            Primitive::Bool      => "B",
            Primitive::Void      => "v",
            Primitive::Object    => "@",
            Primitive::Class     => "#",
            Primitive::Sel       => ":",
            Primitive::Unknown   => "?",
            Primitive::BitField(b) => {
                return write!(formatter, "b{}", b);
            }
        };
        fmt::Display::fmt(code, formatter)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_int_display() {
        assert_eq!(Primitive::Int.to_string(), "i");
    }

    #[test]
    fn test_eq_encoding() {
        let i = Primitive::Int;
        let c = Primitive::Char;

        assert!(i.eq_encoding(&i));
        assert!(!i.eq_encoding(&c));
    }
}
