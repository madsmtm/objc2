use std::fmt;
use std::mem;

use encoding::{Descriptor, Encoding};
use super::{parse, ParseResult};
use super::ptr_encoding::StrPointerEncoding;
use super::struct_encoding::StrStructEncoding;

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
    type Pointer = StrPointerEncoding;
    type Struct = StrStructEncoding;

    fn descriptor(&self) -> Descriptor<StrPointerEncoding, StrStructEncoding> {
        let s = self.as_str();
        match parse(s) {
            ParseResult::Primitive(p) => Descriptor::Primitive(p),
            ParseResult::Pointer =>
                Descriptor::Pointer(StrPointerEncoding::from_str_unchecked(s)),
            ParseResult::Struct =>
                Descriptor::Struct(StrStructEncoding::from_str_unchecked(s)),
            ParseResult::Array |
            ParseResult::Union |
            ParseResult::Error => panic!(),
        }
    }

    fn eq_encoding<T: ?Sized + Encoding>(&self, other: &T) -> bool {
        self.descriptor().eq_encoding(other)
    }
}

impl<S> fmt::Display for StrEncoding<S> where S: ?Sized + AsRef<str> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self.as_str(), formatter)
    }
}
