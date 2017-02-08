use std::fmt;
use descriptor::{Descriptor, AnyEncoding, FieldsIterator};

pub mod descriptor;

pub trait Encoding: fmt::Display {
    fn descriptor(&self) -> Descriptor;
}

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

fn parse(s: &str) -> (Option<StrEncoding<&str>>, &str) {
    if s.len() >= 1 {
        (Some(StrEncoding { buf: &s[..1] }), &s[1..])
    } else {
        (None, s)
    }
}

pub struct StrEncoding<S> where S: AsRef<str> {
    buf: S,
}

impl<S> StrEncoding<S> where S: AsRef<str> {
    fn new_unchecked(s: S) -> StrEncoding<S> {
        StrEncoding { buf: s }
    }
}

impl<S> Encoding for StrEncoding<S> where S: AsRef<str> {
    fn descriptor(&self) -> Descriptor {
        match self.buf.as_ref() {
            s if s.starts_with('{') => {
                let sep_pos = s.find('=').unwrap();
                let f = FieldsIterator::parse(&s[sep_pos + 1..s.len() - 1]);
                Descriptor::Struct(&s[1..sep_pos], f)
            },
            s if s.starts_with('^') => {
                let e = StrEncoding::new_unchecked(&s[1..]);
                Descriptor::Pointer(AnyEncoding::Parsed(e), false)
            },
            s if s.starts_with("r^") => {
                let e = StrEncoding::new_unchecked(&s[2..]);
                Descriptor::Pointer(AnyEncoding::Parsed(e), true)
            },
            "c" => Descriptor::Primitive(Primitive::Char),
            "i" => Descriptor::Primitive(Primitive::Int),
            _ => panic!(),
        }
    }
}

impl<S> fmt::Display for StrEncoding<S> where S: AsRef<str> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self.buf.as_ref(), formatter)
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
        let e = Pointer { t: Primitive::Int, is_const: false };
        assert_eq!(e.to_string(), "^i");
    }

    #[test]
    fn test_static_struct() {
        let f = (Primitive::Char, Primitive::Int);
        let s = Struct { name: "CGPoint", fields: f };
        assert_eq!(s.to_string(), "{CGPoint=ci}");

        let (name, mut fields) = match s.descriptor() {
            Descriptor::Struct(name, fields) => (name, fields),
            _ => panic!("Descriptor was not a struct"),
        };
        assert_eq!(name, "CGPoint");
        assert_eq!(fields.next().unwrap().to_string(), "c");
        assert_eq!(fields.next().unwrap().to_string(), "i");
        assert!(fields.next().is_none());
    }

    #[test]
    fn test_parsed_struct() {
        let s = StrEncoding::new_unchecked("{CGPoint=ci}");

        let (name, mut fields) = match s.descriptor() {
            Descriptor::Struct(name, fields) => (name, fields),
            _ => panic!("Descriptor was not a struct"),
        };
        assert_eq!(name, "CGPoint");
        assert_eq!(fields.next().unwrap().to_string(), "c");
        assert_eq!(fields.next().unwrap().to_string(), "i");
        assert!(fields.next().is_none());
    }

    #[test]
    fn test_descriptor_eq() {
        let i = Primitive::Int;
        let c = Primitive::Char;

        assert!(i.descriptor().eq(i.descriptor()));
        assert!(!i.descriptor().eq(c.descriptor()));

        let p = Pointer { t: i, is_const: false };
        assert!(p.descriptor().eq(p.descriptor()));
        assert!(!p.descriptor().eq(i.descriptor()));

        let s = Struct { name: "CGPoint", fields: (c, i) };
        assert!(s.descriptor().eq(s.descriptor()));
        assert!(!s.descriptor().eq(i.descriptor()));

        let s2 = StrEncoding::new_unchecked("{CGPoint=ci}");
        assert!(s2.descriptor().eq(s2.descriptor()));
        assert!(s.descriptor().eq(s2.descriptor()));
    }
}
