use std::fmt;

use Encoding;
use descriptor::{Descriptor, AnyEncoding, FieldsIterator};
use encodings::Primitive;

pub fn parse(s: &str) -> (Option<StrEncoding<&str>>, &str) {
    if s.len() >= 1 {
        (Some(StrEncoding { buf: &s[..1] }), &s[1..])
    } else {
        (None, s)
    }
}

pub struct StrEncoding<S> where S: AsRef<str> {
    buf: S,
}

impl<S> StrEncoding<S> where S: AsRef<str> {
    pub fn new_unchecked(s: S) -> StrEncoding<S> {
        StrEncoding { buf: s }
    }
}

impl<S> Encoding for StrEncoding<S> where S: AsRef<str> {
    fn descriptor(&self) -> Descriptor {
        match self.buf.as_ref() {
            s if s.starts_with('{') => {
                let sep_pos = s.find('=').unwrap();
                let f = FieldsIterator::parse(&s[sep_pos + 1..s.len() - 1]);
                Descriptor::Struct(&s[1..sep_pos], f)
            },
            s if s.starts_with('^') => {
                let e = StrEncoding::new_unchecked(&s[1..]);
                Descriptor::Pointer(AnyEncoding::Parsed(e), false)
            },
            s if s.starts_with("r^") => {
                let e = StrEncoding::new_unchecked(&s[2..]);
                Descriptor::Pointer(AnyEncoding::Parsed(e), true)
            },
            "c" => Descriptor::Primitive(Primitive::Char),
            "i" => Descriptor::Primitive(Primitive::Int),
            _ => panic!(),
        }
    }
}

impl<S> fmt::Display for StrEncoding<S> where S: AsRef<str> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self.buf.as_ref(), formatter)
    }
}
