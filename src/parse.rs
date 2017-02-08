use std::fmt;

use Encoding;
use descriptor::{Descriptor, DescriptorKind, AnyEncoding, FieldsIterator};
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
        let kind = match self.buf.as_ref() {
            s if s.starts_with('{') => {
                let sep_pos = s.find('=').unwrap();
                let f = FieldsIterator::parse(&s[sep_pos + 1..s.len() - 1]);
                DescriptorKind::Struct(&s[1..sep_pos], f)
            },
            s if s.starts_with('^') => {
                let e = StrEncoding::new_unchecked(&s[1..]);
                DescriptorKind::Pointer(AnyEncoding::Parsed(e), false)
            },
            s if s.starts_with("r^") => {
                let e = StrEncoding::new_unchecked(&s[2..]);
                DescriptorKind::Pointer(AnyEncoding::Parsed(e), true)
            },
            "c" => DescriptorKind::Primitive(Primitive::Char),
            "i" => DescriptorKind::Primitive(Primitive::Int),
            _ => panic!(),
        };
        kind.into()
    }
}

impl<S> fmt::Display for StrEncoding<S> where S: AsRef<str> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self.buf.as_ref(), formatter)
    }
}
