use core::fmt;
use core::mem;

use crate::Encoding;
use super::{is_valid, parse, ParseResult};
use super::multi::StrEncodingsIter;

/// An error returned when constructing a `StrEncoding` if the string could not
/// be parsed as a valid encoding.
#[derive(Clone, Copy, Debug)]
pub struct ParseEncodingError<S>(S) where S: AsRef<str>;

impl<S> fmt::Display for ParseEncodingError<S> where S: AsRef<str> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "Invalid encoding: {:?}", self.0.as_ref())
    }
}

/**
An Objective-C type encoding parsed from its string representation.

Encodings are parsed lazily; for example, when comparing for equality we may
then find that the encoding is of a struct, but no further evaluation is done
of the fields of the struct until they are requested.
*/
#[derive(Clone, Copy, Debug)]
pub struct StrEncoding<S: ?Sized = str>(S) where S: AsRef<str>;

impl StrEncoding {
    /**
    Constructs a `StrEncoding` parsed from the given string, or returns an
    error if the string was not a valid encoding.

    `from_str` will return a reference to a dynamically-sized `StrEncoding`
    which is valid for the lifetime of the given string. To construct a sized
    `StrEncoding` that owns a buffer, use the `new` method.
    */
    pub fn from_str(s: &str) -> Result<&StrEncoding, ParseEncodingError<&str>> {
        if is_valid(s) {
            Ok(StrEncoding::from_str_unchecked(s))
        } else {
            Err(ParseEncodingError(s))
        }
    }

    /**
    Constructs a `StrEncoding` without checking to see if the given string is
    a valid encoding.

    If the given string is not a valid encoding, the returned `StrEncoding`
    **will panic later** when being evaluated.
    */
    pub fn from_str_unchecked(s: &str) -> &StrEncoding {
        unsafe { mem::transmute(s) }
    }
}

impl<S: ?Sized> StrEncoding<S> where S: AsRef<str> {
    /// Returns the string representation of self.
    pub fn as_str(&self) -> &str {
        self.0.as_ref()
    }

    fn eq_encoding(&self, encoding: &Encoding) -> bool {
        let s = self.as_str();
        match (parse(s), *encoding) {
            (ParseResult::Error, _) =>
                panic!("Failed to parse an encoding from {:?}", s),
            (ParseResult::Primitive(p1), p2) => p1 == p2,
            (ParseResult::Pointer(t1), Encoding::Pointer(t2)) =>
                StrEncoding::from_str_unchecked(t1) == t2,
            (ParseResult::Array(l1, i1), Encoding::Array(l2, i2)) =>
                l1 == l2 && StrEncoding::from_str_unchecked(i1) == i2,
            (ParseResult::Struct(n1, f1), Encoding::Struct(n2, f2)) =>
                n1 == n2 && StrEncodingsIter::new(f1).eq(f2),
            (ParseResult::Union(n1, m1), Encoding::Union(n2, m2)) =>
                n1 == n2 && StrEncodingsIter::new(m1).eq(m2),
            _ => false,
        }
    }
}

impl<S: ?Sized> fmt::Display for StrEncoding<S> where S: AsRef<str> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self.as_str(), formatter)
    }
}

impl<'a, S: ?Sized> PartialEq<Encoding<'a>> for StrEncoding<S>
        where S: AsRef<str> {
    fn eq(&self, other: &Encoding<'a>) -> bool {
        self.eq_encoding(other)
    }
}

#[cfg(test)]
mod tests {
    use crate::Encoding;
    use super::*;

    #[test]
    fn test_parsed_array() {
        let a = StrEncoding::from_str_unchecked("[12i]");
        assert!(a == &Encoding::Array(12, &Encoding::Int));
    }

    #[test]
    fn test_parsed_struct() {
        let parsed = StrEncoding::from_str_unchecked("{CGPoint=ci}");
        let expected = Encoding::Struct("CGPoint", &[Encoding::Char, Encoding::Int]);
        assert!(parsed == &expected);
    }
}
