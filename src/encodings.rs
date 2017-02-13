use std::fmt;

use {Encoding, PointerEncoding, StructEncoding};
use descriptor::Descriptor;
use multi::{Encodings, EncodingsComparator, EncodingTupleComparator};

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
    type Pointer = Self;
    type Struct = Never;

    fn descriptor(&self) -> Descriptor<Self, Never> {
        Descriptor::Pointer(self)
    }

    fn eq_encoding<E: ?Sized + Encoding>(&self, other: &E) -> bool {
        if let Descriptor::Pointer(p) = other.descriptor() {
            self.0.eq_encoding(p.pointee())
        } else {
            false
        }
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


pub struct Struct<S, T> where S: AsRef<str>, T: Encodings {
    name: S,
    fields: T,
}

impl<S, T> Struct<S, T> where S: AsRef<str>, T: Encodings {
    pub fn new(name: S, fields: T) -> Struct<S, T> {
        Struct { name: name, fields: fields }
    }
}

impl<S, T> Encoding for Struct<S, T> where S: AsRef<str>, T: Encodings {
    type Pointer = Never;
    type Struct = Self;

    fn descriptor(&self) -> Descriptor<Never, Self> {
        Descriptor::Struct(self)
    }

    fn eq_encoding<E: ?Sized + Encoding>(&self, other: &E) -> bool {
        if let Descriptor::Struct(s) = other.descriptor() {
            s.eq_struct(self.name(), EncodingTupleComparator::new(&self.fields))
        } else {
            false
        }
    }
}

impl<S, T> StructEncoding for Struct<S, T> where S: AsRef<str>, T: Encodings {
    fn name(&self) -> &str {
        self.name.as_ref()
    }

    fn eq_struct<C: EncodingsComparator>(&self, name: &str, fields: C) -> bool {
        self.name() == name && self.fields.eq(fields)
    }
}

impl<S, T> fmt::Display for Struct<S, T> where S: AsRef<str>, T: Encodings {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{{{}=", self.name())?;
        self.fields.write_all(formatter)?;
        write!(formatter, "}}")
    }
}

pub enum Never { }

impl Encoding for Never {
    type Pointer = Never;
    type Struct = Never;

    fn descriptor(&self) -> Descriptor<Never, Never> {
        match self { }
    }

    fn eq_encoding<T: ?Sized + Encoding>(&self, _: &T) -> bool {
        match self { }
    }
}

impl PointerEncoding for Never {
    type Pointee = Never;

    fn pointee(&self) -> &Never {
        match self { }
    }
}

impl StructEncoding for Never {
    fn name(&self) -> &str {
        match self { }
    }

    fn eq_struct<C: EncodingsComparator>(&self, _: &str, _: C) -> bool {
        match self { }
    }
}

impl fmt::Display for Never  {
    fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
        match self { }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use parse::StrEncoding;

    #[test]
    fn test_int_display() {
        assert_eq!(Primitive::Int.to_string(), "i");
    }

    #[test]
    fn test_pointer_display() {
        let e = Pointer::new(Primitive::Int);
        assert_eq!(e.to_string(), "^i");
    }

    #[test]
    fn test_static_struct() {
        let f = (Primitive::Char, Primitive::Int);
        let s = Struct::new("CGPoint", f);
        assert_eq!(s.name(), "CGPoint");
        assert_eq!(s.to_string(), "{CGPoint=ci}");
    }

    #[test]
    fn test_eq_encoding() {
        let i = Primitive::Int;
        let c = Primitive::Char;

        assert!(i.eq_encoding(&i));
        assert!(!i.eq_encoding(&c));

        let p = Pointer::new(i);
        assert!(p.eq_encoding(&p));
        assert!(!p.eq_encoding(&i));

        let s = Struct::new("CGPoint", (c, i));
        assert!(s.eq_encoding(&s));
        assert!(!s.eq_encoding(&i));

        let s2 = StrEncoding::new_unchecked("{CGPoint=ci}");
        assert!(s2.eq_encoding(&s2));
        assert!(s.eq_encoding(&s2));
    }
}
