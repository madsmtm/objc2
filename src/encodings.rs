use std::fmt;

use {Encoding, PointerEncoding, StructEncoding, FieldsComparator};
use descriptor::{Descriptor, DescriptorKind, AnyEncoding, FieldsIterator};

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

impl Primitive {
    fn code(self) -> &'static str {
        match self {
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
        }
    }
}

impl Encoding for Primitive {
    fn descriptor(&self) -> Descriptor {
        DescriptorKind::Primitive(*self).into()
    }
}

impl fmt::Display for Primitive {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self.code(), formatter)
    }
}

pub struct Pointer<T>(T) where T: Encoding;

impl<T> Pointer<T> where T: Encoding {
    pub fn new(pointee: T) -> Pointer<T> {
        Pointer(pointee)
    }
}

impl<T> Encoding for Pointer<T> where T: Encoding {
    fn descriptor(&self) -> Descriptor {
        DescriptorKind::Pointer(AnyEncoding::Static(&self.0)).into()
    }
}

impl<T> PointerEncoding for Pointer<T> where T: Encoding {
    type Pointee = T;

    fn pointee(&self) -> &T {
        &self.0
    }
}

impl<T> fmt::Display for Pointer<T> where T: Encoding {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "^{}", self.0)
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

pub struct Struct<S, T> where S: AsRef<str>, T: EncodingTuple {
    name: S,
    fields: T,
}

impl<S, T> Struct<S, T> where S: AsRef<str>, T: EncodingTuple {
    pub fn new(name: S, fields: T) -> Struct<S, T> {
        Struct { name: name, fields: fields }
    }
}

impl<S, T> Encoding for Struct<S, T> where S: AsRef<str>, T: EncodingTuple {
    fn descriptor(&self) -> Descriptor {
        DescriptorKind::Struct(self.name(), FieldsIterator::new(&self.fields)).into()
    }
}

impl<S, T> StructEncoding for Struct<S, T> where S: AsRef<str>, T: EncodingTuple {
    fn name(&self) -> &str {
        self.name.as_ref()
    }

    fn fields_eq<F: FieldsComparator>(&self, mut other: F) -> bool {
        for i in 0.. {
            if let Some(e) = self.fields.encoding_at(i) {
                if !other.eq_next(e) {
                    return false;
                }
            } else { break; }
        }
        other.is_finished()
    }

}

impl<S, T> fmt::Display for Struct<S, T> where S: AsRef<str>, T: EncodingTuple {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{{{}=", self.name())?;
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
        let e = Pointer::new(Primitive::Int);
        assert_eq!(e.to_string(), "^i");
    }
}
