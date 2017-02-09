use std::fmt;
use std::ops::Deref;

use Encoding;
use encodings::{EncodingTuple, Primitive};
use parse::{StrEncoding, chomp};

pub enum DescriptorKind<'a> {
    Primitive(Primitive),
    Pointer(AnyEncoding<'a>),
    Struct(&'a str, FieldsIterator<'a>),
}

impl<'a> DescriptorKind<'a> {
    pub fn into(self) -> Descriptor<'a> {
        Descriptor(self)
    }
}

pub struct Descriptor<'a>(DescriptorKind<'a>);

impl<'a> Descriptor<'a> {
    pub fn eq(self, other: Descriptor) -> bool {
        use self::DescriptorKind::*;

        match (self.0, other.0) {
            (Primitive(p1), Primitive(p2)) => {
                p1 == p2
            },
            (Pointer(e1), Pointer(e2)) => {
                e1 == e2
            },
            (Struct(n1, f1), Struct(n2, f2)) => {
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
        use self::FieldsIterator::*;

        match *self {
            Static(tup, index) => {
                *self = Static(tup, index + 1);
                tup.encoding_at(index).map(|e| AnyEncoding::Static(e))
            },
            Parsed(s) => {
                let (enc, remaining) = chomp(s);
                *self = Parsed(remaining);
                enc.map(|e| AnyEncoding::Parsed(StrEncoding::new_unchecked(e)))
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use encodings::*;
    use parse::StrEncoding;

    #[test]
    fn test_static_struct() {
        let f = (Primitive::Char, Primitive::Int);
        let s = Struct::new("CGPoint", f);
        assert_eq!(s.to_string(), "{CGPoint=ci}");

        let (name, mut fields) = match s.descriptor().0 {
            DescriptorKind::Struct(name, fields) => (name, fields),
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

        let (name, mut fields) = match s.descriptor().0 {
            DescriptorKind::Struct(name, fields) => (name, fields),
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

        let p = Pointer::new(i);
        assert!(p.descriptor().eq(p.descriptor()));
        assert!(!p.descriptor().eq(i.descriptor()));

        let s = Struct::new("CGPoint", (c, i));
        assert!(s.descriptor().eq(s.descriptor()));
        assert!(!s.descriptor().eq(i.descriptor()));

        let s2 = StrEncoding::new_unchecked("{CGPoint=ci}");
        assert!(s2.descriptor().eq(s2.descriptor()));
        assert!(s.descriptor().eq(s2.descriptor()));
    }
}
