use core::fmt;
use core::mem;

use {Descriptor, Encoding};
use multi::Encodings;
use super::{is_valid, parse, ParseResult};
use super::multi::StrFields;

#[derive(Clone, Copy, Debug)]
pub struct ParseEncodingError<S>(S) where S: AsRef<str>;

impl<S> fmt::Display for ParseEncodingError<S> where S: AsRef<str> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "Invalid encoding: {:?}", self.0.as_ref())
    }
}

#[derive(Clone, Copy, Debug)]
pub struct StrEncoding<S = str>(S) where S: ?Sized + AsRef<str>;

impl StrEncoding {
    pub fn from_str(s: &str) -> Result<&StrEncoding, ParseEncodingError<&str>> {
        if is_valid(s) {
            Ok(StrEncoding::from_str_unchecked(s))
        } else {
            Err(ParseEncodingError(s))
        }
    }

    pub fn from_str_unchecked(s: &str) -> &StrEncoding {
        unsafe { mem::transmute(s) }
    }
}

impl<S> StrEncoding<S> where S: AsRef<str> {
    pub fn new(s: S) -> Result<StrEncoding<S>, ParseEncodingError<S>> {
        if is_valid(s.as_ref()) {
            Ok(StrEncoding::new_unchecked(s))
        } else {
            Err(ParseEncodingError(s))
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
    type UnionMembers = StrFields;

    fn descriptor(&self) -> Descriptor<StrEncoding, StrEncoding, StrFields, StrFields> {
        let s = self.as_str();
        match parse(s) {
            ParseResult::Primitive(p) => Descriptor::Primitive(p),
            ParseResult::Pointer(t) =>
                Descriptor::Pointer(StrEncoding::from_str_unchecked(t)),
            ParseResult::Array(len, item) =>
                Descriptor::Array(len, StrEncoding::from_str_unchecked(item)),
            ParseResult::Struct(name, fields) =>
                Descriptor::Struct(name, StrFields::from_str_unchecked(fields)),
            ParseResult::Union(name, members) =>
                Descriptor::Union(name, StrFields::from_str_unchecked(members)),
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
                n1 == n2 && f1.eq_encodings(f2),
            (Union(n1, m1), Union(n2, m2)) =>
                n1 == n2 && m1.eq_encodings(m2),
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
    use encoding::{Array, Primitive, Struct};
    use super::*;

    #[test]
    fn test_parsed_array() {
        let a = StrEncoding::from_str_unchecked("[12^i]");
        assert!(a.eq_encoding(&Array::new(12, Primitive::Int)));
    }

    #[test]
    fn test_parsed_struct() {
        let parsed = StrEncoding::from_str_unchecked("{CGPoint=ci}");
        let expected = Struct::new("CGPoint", (Primitive::Char, Primitive::Int));
        assert!(parsed.eq_encoding(&expected));
    }
}
