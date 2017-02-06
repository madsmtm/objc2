use std::fmt;

pub trait Encoding: fmt::Display {
    fn as_primitive(&self) -> Option<Primitive> { None }
    fn as_pointer(&self) -> Option<(&Encoding, bool)> { None }
    fn as_struct(&self) -> Option<StructDescriptor> { None }
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
    fn as_primitive(&self) -> Option<Primitive> {
        Some(*self)
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
    fn as_pointer(&self) -> Option<(&Encoding, bool)> {
        Some((&self.t, self.is_const))
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
    fn as_struct(&self) -> Option<StructDescriptor> {
        Some(StructDescriptor::new(self.name, &self.fields))
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

enum StructFields<'a> {
    Static(&'a EncodingTuple),
    Parsed(&'a str),
}

pub struct StructDescriptor<'a> {
    name: &'a str,
    fields: StructFields<'a>,
    index: u8,
    current: Option<Primitive>,
}

impl<'a> StructDescriptor<'a> {
    pub fn new<'b>(name: &'b str, fields: &'b EncodingTuple) -> StructDescriptor<'b> {
        StructDescriptor {
            name: name,
            fields: StructFields::Static(fields),
            index: 0,
            current: None,
        }
    }

    fn parse<'b>(name: &'b str, fields: &'b str) -> StructDescriptor<'b> {
        StructDescriptor {
            name: name,
            fields: StructFields::Parsed(fields),
            index: 0,
            current: None,
        }
    }

    pub fn name(&self) -> &str {
        self.name
    }

    pub fn next_field(&mut self) -> Option<&Encoding> {
        let index = self.index;
        self.index += 1;
        match self.fields {
            StructFields::Static(tup) => tup.encoding_at(index),
            StructFields::Parsed(s) => {
                let (enc, remaining) = parse(s);
                self.current = enc;
                self.fields = StructFields::Parsed(remaining);
                self.current.as_ref().map(|e| e as &Encoding)
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

        let mut s = s.as_struct().unwrap();
        assert_eq!(s.name(), "CGPoint");
        assert_eq!(s.next_field().unwrap().to_string(), "c");
        assert_eq!(s.next_field().unwrap().to_string(), "i");
        assert!(s.next_field().is_none());
    }

    #[test]
    fn test_parsed_struct() {
        let mut s = StructDescriptor::parse("CGPoint", "ci");
        assert_eq!(s.name(), "CGPoint");
        assert_eq!(s.next_field().unwrap().to_string(), "c");
        assert_eq!(s.next_field().unwrap().to_string(), "i");
        assert!(s.next_field().is_none());
    }
}
