use std::fmt;

use Encoding;
use descriptor::{Descriptor, DescriptorKind, AnyEncoding, FieldsIterator};
use encodings::Primitive;

pub fn chomp(s: &str) -> (Option<&str>, &str) {
    let head_len = chomp_ptr(s)
        .or_else(|| chomp_struct(s))
        .or_else(|| {
            if let (Some(_), t) = chomp_primitive(s) {
                Some(s.len() - t.len())
            } else {
                None
            }
        });

    if let Some(head_len) = head_len {
        let (h, t) = s.split_at(head_len);
        (Some(h), t)
    } else {
        (None, s)
    }
}

fn chomp_ptr(s: &str) -> Option<usize> {
    if s.starts_with("^") {
        let (h, _) = chomp(&s[1..]);
        h.map(|h| h.len() + 1)
    } else {
        None
    }
}

fn chomp_struct(s: &str) -> Option<usize> {
    if !s.starts_with("{") {
        return None;
    }

    let mut depth = 1;
    for (i, b) in s.bytes().enumerate().skip(1) {
        if b == b'{' {
            depth += 1;
        } else if b == b'}' {
            depth -= 1;
        }

        if depth == 0 {
            return Some(i + 1);
        }
    }

    None
}

fn chomp_primitive(s: &str) -> (Option<Primitive>, &str) {
    if s.is_empty() {
        return (None, s);
    }

    let (h, t) = s.split_at(1);
    match h {
        "c" => (Some(Primitive::Char), t),
        "i" => (Some(Primitive::Int), t),
        _ => (None, s),
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
                DescriptorKind::Pointer(AnyEncoding::Parsed(e))
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chomp() {
        let (h, t) = chomp("{A={B=ci^{C=c}}ci}c^i{C=c}");
        assert_eq!(h, Some("{A={B=ci^{C=c}}ci}"));

        let (h, t) = chomp(t);
        assert_eq!(h, Some("c"));

        let (h, t) = chomp(t);
        assert_eq!(h, Some("^i"));

        let (h, t) = chomp(t);
        assert_eq!(h, Some("{C=c}"));

        let (h, _) = chomp(t);
        assert_eq!(h, None);
    }
}
