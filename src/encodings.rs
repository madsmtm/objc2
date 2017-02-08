use std::fmt;

use Encoding;
use descriptor::{Descriptor, AnyEncoding, FieldsIterator};

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
}

impl Encoding for Primitive {
    fn descriptor(&self) -> Descriptor {
        Descriptor::Primitive(*self)
    }
}

impl fmt::Display for Primitive {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let s = match *self {
            Primitive::Char => "c",
            Primitive::Int => "i",
            _ => panic!(),
        };
        fmt::Display::fmt(s, formatter)
    }
}

pub struct Pointer<T> where T: Encoding {
    t: T,
    is_const: bool,
}

impl<T> Pointer<T> where T: Encoding {
    pub fn new(pointee: T, is_const: bool) -> Pointer<T> {
        Pointer { t: pointee, is_const: is_const }
    }
}

impl<T> Encoding for Pointer<T> where T: Encoding {
    fn descriptor(&self) -> Descriptor {
        Descriptor::Pointer(AnyEncoding::Static(&self.t), self.is_const)
    }
}

impl<T> fmt::Display for Pointer<T> where T: Encoding {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{}^{}", if self.is_const {"r"} else {""}, self.t)
    }
}

pub trait EncodingTuple {
    fn encoding_at(&self, i: u8) -> Option<&Encoding>;
}

impl<A, B> EncodingTuple for (A, B) where A: Encoding, B: Encoding {
    fn encoding_at(&self, i: u8) -> Option<&Encoding> {
        match i {
            0 => Some(&self.0),
            1 => Some(&self.1),
            _ => None,
        }
    }
}

pub struct Struct<'a, T> where T: EncodingTuple {
    name: &'a str,
    fields: T,
}

impl<'a, T> Struct<'a, T> where T: EncodingTuple {
    pub fn new(name: &'a str, fields: T) -> Struct<'a, T> {
        Struct { name: name, fields: fields }
    }
}

impl<'a, T> Encoding for Struct<'a, T> where T: EncodingTuple {
    fn descriptor(&self) -> Descriptor {
        Descriptor::Struct(self.name, FieldsIterator::new(&self.fields))
    }
}

impl<'a, T> fmt::Display for Struct<'a, T> where T: EncodingTuple {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{{{}=", self.name)?;
        for i in 0.. {
            if let Some(e) = self.fields.encoding_at(i) {
                write!(formatter, "{}", e)?;
            } else { break; }
        }
        write!(formatter, "}}")
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
    fn test_pointer_display() {
        let e = Pointer::new(Primitive::Int, false);
        assert_eq!(e.to_string(), "^i");
    }
}
