use core::fmt;
use core::mem;

use crate::{Descriptor, Encoding};
use super::{is_valid, parse, ParseResult};
use super::multi::StrEncodings;

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

impl<S> StrEncoding<S> where S: AsRef<str> {
    /**
    Constructs a `StrEncoding` parsed from the given string, or returns an
    error if the string was not a valid encoding.

    `new` constructs a sized `StrEncoding` that owns the given buffer, meaning
    it can be moved without dealing with lifetimes. If you only have a borrowed
    `&str` slice or don't want to allocate a buffer, use the `from_str` method.
    */
    pub fn new(s: S) -> Result<StrEncoding<S>, ParseEncodingError<S>> {
        if is_valid(s.as_ref()) {
            Ok(StrEncoding::new_unchecked(s))
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
    pub fn new_unchecked(s: S) -> StrEncoding<S> {
        StrEncoding(s)
    }
}

impl<S: ?Sized> StrEncoding<S> where S: AsRef<str> {
    /// Returns the string representation of self.
    pub fn as_str(&self) -> &str {
        self.0.as_ref()
    }
}

impl<S: ?Sized> Encoding for StrEncoding<S> where S: AsRef<str> {
    type PointerTarget = StrEncoding;
    type ArrayItem = StrEncoding;
    type StructFields = StrEncodings;
    type UnionMembers = StrEncodings;

    fn descriptor(&self) -> Descriptor<StrEncoding, StrEncoding, StrEncodings, StrEncodings> {
        let s = self.as_str();
        match parse(s) {
            ParseResult::Primitive(p) => Descriptor::Primitive(p),
            ParseResult::Pointer(t) =>
                Descriptor::Pointer(StrEncoding::from_str_unchecked(t)),
            ParseResult::Array(len, item) =>
                Descriptor::Array(len, StrEncoding::from_str_unchecked(item)),
            ParseResult::Struct(name, fields) =>
                Descriptor::Struct(name, StrEncodings::from_str_unchecked(fields)),
            ParseResult::Union(name, members) =>
                Descriptor::Union(name, StrEncodings::from_str_unchecked(members)),
            ParseResult::Error => panic!("Failed to parse an encoding from {:?}", s),
        }
    }

    fn write<W: fmt::Write>(&self, writer: &mut W) -> fmt::Result {
        writer.write_str(self.as_str())
    }
}

impl<S: ?Sized> fmt::Display for StrEncoding<S> where S: AsRef<str> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        self.write(formatter)
    }
}

impl<S: ?Sized, E: ?Sized> PartialEq<E> for StrEncoding<S>
        where S: AsRef<str>, E: Encoding {
    fn eq(&self, other: &E) -> bool {
        self.eq_encoding(other)
    }
}

#[cfg(test)]
mod tests {
    use crate::encoding::{Array, Primitive, Struct};
    use super::*;

    #[test]
    fn test_parsed_array() {
        let a = StrEncoding::from_str_unchecked("[12i]");
        assert!(a == &Array::new(12, Primitive::Int));
    }

    #[test]
    fn test_parsed_struct() {
        let parsed = StrEncoding::from_str_unchecked("{CGPoint=ci}");
        let expected = Struct::new("CGPoint", (Primitive::Char, Primitive::Int));
        assert!(parsed == &expected);
    }
}
