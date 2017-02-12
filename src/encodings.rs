use std::fmt;

use {Encoding, PointerEncoding, StructEncoding, FieldsComparator};
use descriptor::Descriptor;

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

pub trait EncodingTuple {
    fn eq<F: FieldsComparator>(&self, F) -> bool;

    fn write_all<W: fmt::Write>(&self, &mut W) -> fmt::Result;

    fn encoding_at_eq<T: ?Sized + Encoding>(&self, u8, &T) -> bool;

    fn len(&self) -> u8;
}

impl<A, B> EncodingTuple for (A, B) where A: Encoding, B: Encoding {
    fn eq<F: FieldsComparator>(&self, mut fields: F) -> bool {
        fields.eq_next(&self.0) && fields.eq_next(&self.1) && fields.is_finished()
    }

    fn write_all<W: fmt::Write>(&self, formatter: &mut W) -> fmt::Result {
        write!(formatter, "{}", self.0)?;
        write!(formatter, "{}", self.1)?;
        Ok(())
    }

    fn encoding_at_eq<T: ?Sized + Encoding>(&self, index: u8, other: &T) -> bool {
        match index {
            0 => self.0.eq_encoding(other),
            1 => self.1.eq_encoding(other),
            _ => false,
        }
    }

    fn len(&self) -> u8 { 2 }
}

struct EncodingTupleComparator<'a, T> where T: 'a + EncodingTuple {
    encs: &'a T,
    index: u8,
}

impl<'a, T> EncodingTupleComparator<'a, T> where T: 'a + EncodingTuple {
    fn new(encs: &'a T) -> EncodingTupleComparator<'a, T> {
        EncodingTupleComparator { encs: encs, index: 0 }
    }
}

impl<'a, T> FieldsComparator for EncodingTupleComparator<'a, T>
        where T: 'a + EncodingTuple {
    fn eq_next<E: ?Sized + Encoding>(&mut self, other: &E) -> bool {
        let index = self.index;
        if index < self.encs.len() {
            self.index += 1;
            self.encs.encoding_at_eq(index, other)
        } else {
            false
        }
    }

    fn is_finished(&self) -> bool {
        self.index >= self.encs.len()
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
    type Pointer = Never;
    type Struct = Self;

    fn descriptor(&self) -> Descriptor<Never, Self> {
        Descriptor::Struct(self)
    }

    fn eq_encoding<E: ?Sized + Encoding>(&self, other: &E) -> bool {
        if let Descriptor::Struct(s) = other.descriptor() {
            self.name() == s.name() &&
                s.fields_eq(EncodingTupleComparator::new(&self.fields))
        } else {
            false
        }
    }
}

impl<S, T> StructEncoding for Struct<S, T> where S: AsRef<str>, T: EncodingTuple {
    fn name(&self) -> &str {
        self.name.as_ref()
    }

    fn fields_eq<F: FieldsComparator>(&self, other: F) -> bool {
        self.fields.eq(other)
    }

}

impl<S, T> fmt::Display for Struct<S, T> where S: AsRef<str>, T: EncodingTuple {
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

    fn fields_eq<T: FieldsComparator>(&self, _: T) -> bool {
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
        assert!(s2.eq_encoding(s2));
        assert!(s.eq_encoding(s2));
    }
}
