use std::fmt;
use std::mem;

use encoding::{Descriptor, Encoding, Never};
use multi::Encodings;
use super::{is_valid, parse, ParseResult};
use super::multi::{StrFields, StrFieldsIter};

pub struct StrEncoding<S = str>(S) where S: ?Sized + AsRef<str>;

impl StrEncoding {
    pub fn from_str(s: &str) -> Option<&StrEncoding> {
        if is_valid(s) {
            Some(StrEncoding::from_str_unchecked(s))
        } else {
            None
        }
    }

    pub fn from_str_unchecked(s: &str) -> &StrEncoding {
        unsafe { mem::transmute(s) }
    }
}

impl<S> StrEncoding<S> where S: AsRef<str> {
    pub fn new(s: S) -> Option<StrEncoding<S>> {
        if is_valid(s.as_ref()) {
            Some(StrEncoding::new_unchecked(s))
        } else {
            None
        }
    }

    pub fn new_unchecked(s: S) -> StrEncoding<S> {
        StrEncoding(s)
    }
}

impl<S> StrEncoding<S> where S: ?Sized + AsRef<str> {
    pub fn as_str(&self) -> &str {
        self.0.as_ref()
    }
}

impl<S> Encoding for StrEncoding<S> where S: ?Sized + AsRef<str> {
    type PointerTarget = StrEncoding;
    type ArrayItem = StrEncoding;
    type StructFields = StrFields;
    type UnionMembers = Never;

    fn descriptor(&self) -> Descriptor<StrEncoding, StrEncoding, StrFields, Never> {
        let s = self.as_str();
        match parse(s) {
            ParseResult::Primitive(p) => Descriptor::Primitive(p),
            ParseResult::Pointer(t) =>
                Descriptor::Pointer(StrEncoding::from_str_unchecked(t)),
            ParseResult::Array(len, item) =>
                Descriptor::Array(len, StrEncoding::from_str_unchecked(item)),
            ParseResult::Struct(name, fields) =>
                Descriptor::Struct(name, StrFields::from_str_unchecked(fields)),
            ParseResult::Error => panic!("Failed to parse an encoding from {:?}", s),
        }
    }

    fn eq_encoding<T: ?Sized + Encoding>(&self, other: &T) -> bool {
        use Descriptor::*;
        match (self.descriptor(), other.descriptor()) {
            (Primitive(p1), Primitive(p2)) => p1 == p2,
            (Pointer(t1), Pointer(t2)) => t1.eq_encoding(t2),
            (Array(l1, i1), Array(l2, i2)) =>
                l1 == l2 && i1.eq_encoding(i2),
            (Struct(n1, f1), Struct(n2, f2)) =>
                n1 == n2 && f2.eq(StrFieldsIter::new(f1)),
            _ => false,
        }
    }
}

impl<S> fmt::Display for StrEncoding<S> where S: ?Sized + AsRef<str> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self.as_str(), formatter)
    }
}

#[cfg(test)]
mod tests {
    use encoding::{Array, Primitive};
    use super::*;

    #[test]
    fn test_parsed_array() {
        let a = StrEncoding::from_str_unchecked("[12^i]");
        assert!(a.eq_encoding(&Array::new(12, Primitive::Int)));
    }

    #[test]
    fn test_parsed_struct() {
        let s = StrEncoding::from_str_unchecked("{CGPoint=ci}");

        let (name, fields) = match s.descriptor() {
            Descriptor::Struct(name, fields) => (name, fields),
            _ => panic!("Descriptor was not a struct"),
        };
        assert_eq!(name, "CGPoint");

        let mut fields = StrFieldsIter::new(fields);
        assert_eq!(fields.next().unwrap().to_string(), "c");
        assert_eq!(fields.next().unwrap().to_string(), "i");
        assert!(fields.next().is_none());
    }
}
