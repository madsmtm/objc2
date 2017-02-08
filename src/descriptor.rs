use std::fmt;
use std::ops::Deref;
use super::{Encoding, EncodingTuple, Primitive, StrEncoding, parse};

pub enum Descriptor<'a> {
    Primitive(Primitive),
    Pointer(AnyEncoding<'a>, bool),
    Struct(&'a str, FieldsIterator<'a>),
}

impl<'a> Descriptor<'a> {
    pub fn eq(self, other: Descriptor) -> bool {
        match (self, other) {
            (Descriptor::Primitive(p1), Descriptor::Primitive(p2)) => {
                p1 == p2
            },
            (Descriptor::Pointer(e1, c1), Descriptor::Pointer(e2, c2)) => {
                c1 == c2 && e1 == e2
            },
            (Descriptor::Struct(n1, f1), Descriptor::Struct(n2, f2)) => {
                n1 == n2 && f1.eq(f2)
            }
            _ => false,
        }
    }
}

pub enum AnyEncoding<'a> {
    Static(&'a Encoding),
    Parsed(StrEncoding<&'a str>),
}

impl<'a> Deref for AnyEncoding<'a> {
    type Target = (Encoding + 'a);

    fn deref(&self) -> &(Encoding + 'a) {
        match *self {
            AnyEncoding::Static(e) => e,
            AnyEncoding::Parsed(ref e) => e,
        }
    }
}

impl<'a> fmt::Display for AnyEncoding<'a> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AnyEncoding::Static(e) => fmt::Display::fmt(e, formatter),
            AnyEncoding::Parsed(ref e) => fmt::Display::fmt(e, formatter),
        }
    }
}

impl<'a> PartialEq for AnyEncoding<'a> {
    fn eq(&self, other: &AnyEncoding) -> bool {
        self.descriptor().eq(other.descriptor())
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

    pub fn parse(fields: &str) -> FieldsIterator {
        FieldsIterator::Parsed(fields)
    }
}

impl<'a> Iterator for FieldsIterator<'a> {
    type Item = AnyEncoding<'a>;

    fn next(&mut self) -> Option<AnyEncoding<'a>> {
        use FieldsIterator::*;

        match *self {
            Static(tup, index) => {
                *self = Static(tup, index + 1);
                tup.encoding_at(index).map(|e| AnyEncoding::Static(e))
            },
            Parsed(s) => {
                let (enc, remaining) = parse(s);
                *self = Parsed(remaining);
                enc.map(|e| AnyEncoding::Parsed(e))
            },
        }
    }
}