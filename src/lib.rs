use std::fmt;
use std::ops::Deref;

pub trait Encoding: fmt::Display {
    fn descriptor(&self) -> Descriptor;
}

pub enum Descriptor<'a> {
    Primitive(Primitive),
    Pointer(EncodeFoo<'a>, bool),
    Struct(&'a str, FieldsIterator<'a>),
}

pub enum EncodeFoo<'a> {
    Static(&'a Encoding),
    Parsed(Primitive),
}

impl<'a> Deref for EncodeFoo<'a> {
    type Target = (Encoding + 'a);

    fn deref(&self) -> &(Encoding + 'a) {
        match *self {
            EncodeFoo::Static(e) => e,
            EncodeFoo::Parsed(ref e) => e,
        }
    }
}

impl<'a> fmt::Display for EncodeFoo<'a> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            EncodeFoo::Static(e) => fmt::Display::fmt(e, formatter),
            EncodeFoo::Parsed(ref e) => fmt::Display::fmt(e, formatter),
        }
    }
}

#[derive(Clone, Copy)]
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
        Descriptor::Pointer(EncodeFoo::Static(&self.t), self.is_const)
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

pub enum FieldsIterator<'a> {
    Static(&'a EncodingTuple, u8),
    Parsed(&'a str),
}

impl<'a> FieldsIterator<'a> {
    pub fn new(fields: &EncodingTuple) -> FieldsIterator {
        FieldsIterator::Static(fields, 0)
    }

    fn parse(fields: &str) -> FieldsIterator {
        FieldsIterator::Parsed(fields)
    }
}

impl<'a> Iterator for FieldsIterator<'a> {
    type Item = EncodeFoo<'a>;

    fn next(&mut self) -> Option<EncodeFoo<'a>> {
        use FieldsIterator::*;

        match *self {
            Static(tup, index) => {
                *self = Static(tup, index + 1);
                tup.encoding_at(index).map(|e| EncodeFoo::Static(e))
            },
            Parsed(s) => {
                let (enc, remaining) = parse(s);
                *self = Parsed(remaining);
                enc.map(|e| EncodeFoo::Parsed(e))
            },
        }
    }
}

fn parse(s: &str) -> (Option<Primitive>, &str) {
    if s.len() == 0 {
        return (None, s);
    }
    match s.as_bytes()[0] {
        b'c' => (Some(Primitive::Char), &s[1..]),
        b'i' => (Some(Primitive::Int), &s[1..]),
        _ => (None, s),
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
        let mut fields = FieldsIterator::parse("ci");
        assert_eq!(fields.next().unwrap().to_string(), "c");
        assert_eq!(fields.next().unwrap().to_string(), "i");
        assert!(fields.next().is_none());
    }
}
