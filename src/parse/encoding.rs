use std::fmt;
use std::mem;

use encoding::{Descriptor, Encoding, Never};
use multi::Encodings;
use super::{parse, ParseResult};
use super::multi::{StrFields, StrFieldsIter};

pub struct StrEncoding<S = str>(S) where S: ?Sized + AsRef<str>;

impl StrEncoding {
    pub fn from_str_unchecked(s: &str) -> &StrEncoding {
        unsafe { mem::transmute(s) }
    }
}

impl<S> StrEncoding<S> where S: AsRef<str> {
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
    type ArrayItem = Never;
    type StructFields = StrFields;
    type UnionMembers = Never;

    fn descriptor(&self) -> Descriptor<StrEncoding, Never, StrFields, Never> {
        let s = self.as_str();
        match parse(s) {
            ParseResult::Primitive(p) => Descriptor::Primitive(p),
            ParseResult::Pointer(t) =>
                Descriptor::Pointer(StrEncoding::from_str_unchecked(t)),
            ParseResult::Struct(name, fields) =>
                Descriptor::Struct(name, StrFields::from_str_unchecked(fields)),
            ParseResult::Array(..) |
            ParseResult::Error => panic!(),
        }
    }

    fn eq_encoding<T: ?Sized + Encoding>(&self, other: &T) -> bool {
        use Descriptor::*;
        match (self.descriptor(), other.descriptor()) {
            (Primitive(p1), Primitive(p2)) => p1 == p2,
            (Pointer(t1), Pointer(t2)) => t1.eq_encoding(t2),
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
    use super::*;

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
