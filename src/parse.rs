use std::fmt;
use std::mem;
use std::ops::Deref;

use {Encoding, PointerEncoding, StructEncoding, FieldsComparator};
use descriptor::Descriptor;
use encodings::{Primitive, Never};

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

/*
enum ParseResult<'a> {
    Primitive(Primitive),
    Pointer(&'a str),
    Struct(&'a str, &'a str),
    Error,
}

fn parse(s: &str) -> ParseResult {
    if s.starts_with('{') {
        if !s.ends_with('}') {
            ParseResult::Error
        } else if let Some(sep_pos) = s.find('=') {
            let name = &s[1..sep_pos];
            let fields = &s[sep_pos + 1..s.len() - 1];
            ParseResult::Struct(name, fields)
        } else {
            ParseResult::Error
        }
    } else if s.starts_with('^') {
        ParseResult::Pointer(&s[1..])
    } else {
        let (h, t) = chomp_primitive(s);
        if !t.is_empty() {
            ParseResult::Error
        } else if let Some(p) = h {
            ParseResult::Primitive(p)
        } else {
            ParseResult::Error
        }
    }
}

fn is_valid(s: &str) -> bool {
    match parse(s) {
        ParseResult::Primitive(_) => true,
        ParseResult::Pointer(s) => is_valid(s),
        ParseResult::Struct(_, mut fields) => {
            while !fields.is_empty() {
                let (h, t) = chomp(fields);
                if h.map_or(false, is_valid) {
                    return false;
                }
                fields = t;
            }
            true
        }
        ParseResult::Error => false,
    }
}
*/

pub struct StrEncoding(str);

impl StrEncoding {
    pub fn new_unchecked(s: &str) -> &StrEncoding {
        unsafe { mem::transmute(s) }
    }
}

impl Encoding for StrEncoding {
    type Pointer = StrPointerEncoding;
    type Struct = StrStructEncoding;

    fn descriptor(&self) -> Descriptor<StrPointerEncoding, StrStructEncoding> {
        if self.0.starts_with("^") {
            Descriptor::Pointer(StrPointerEncoding::new_unchecked(&self.0))
        } else if self.0.starts_with("{") {
            Descriptor::Struct(StrStructEncoding::new_unchecked(&self.0))
        } else {
            match chomp_primitive(&self.0) {
                (Some(p), t) if t.is_empty() => Descriptor::Primitive(p),
                _ => panic!(),
            }
        }
    }

    fn eq_encoding<T: ?Sized + Encoding>(&self, other: &T) -> bool {
        self.descriptor().eq_encoding(other)
    }
}

impl fmt::Display for StrEncoding {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.0, formatter)
    }
}

pub struct StrPointerEncoding(StrEncoding);

impl StrPointerEncoding {
    fn new_unchecked(s: &str) -> &StrPointerEncoding {
        unsafe { mem::transmute(s) }
    }
}

impl Encoding for StrPointerEncoding {
    type Pointer = StrPointerEncoding;
    type Struct = Never;

    fn descriptor(&self) -> Descriptor<StrPointerEncoding, Never> {
        Descriptor::Pointer(self)
    }

    fn eq_encoding<T: ?Sized + Encoding>(&self, other: &T) -> bool {
        if let Descriptor::Pointer(p) = other.descriptor() {
            self.pointee().eq_encoding(p)
        } else {
            false
        }
    }
}

impl PointerEncoding for StrPointerEncoding {
    type Pointee = StrEncoding;

    fn pointee(&self) -> &StrEncoding {
        StrEncoding::new_unchecked(&(self.0).0[1..])
    }
}

impl fmt::Display for StrPointerEncoding {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.0, formatter)
    }
}

pub struct StrStructEncoding(StrEncoding);

impl StrStructEncoding {
    fn new_unchecked(s: &str) -> &StrStructEncoding {
        unsafe { mem::transmute(s) }
    }

    fn fields(&self) -> StrFields {
        let sep_pos = (self.0).0.find('=').unwrap();
        let fields = &(self.0).0[sep_pos + 1..(self.0).0.len() - 1];
        StrFields { fields: fields }
    }
}

impl Encoding for StrStructEncoding {
    type Pointer = Never;
    type Struct = StrStructEncoding;

    fn descriptor(&self) -> Descriptor<Never, StrStructEncoding> {
        Descriptor::Struct(self)
    }

    fn eq_encoding<T: ?Sized + Encoding>(&self, other: &T) -> bool {
        if let Descriptor::Struct(s) = other.descriptor() {
            s.eq_struct(self.name(), self.fields())
        } else {
            false
        }
    }
}

impl StructEncoding for StrStructEncoding {
    fn name(&self) -> &str {
        let sep_pos = (self.0).0.find('=').unwrap();
        &(self.0).0[1..sep_pos]
    }

    fn eq_struct<F: FieldsComparator>(&self, name: &str, mut other: F) -> bool {
        if self.name() != name {
            return false;
        }

        for enc in self.fields() {
            if !other.eq_next(enc) {
                return false;
            }
        }

        other.is_finished()
    }
}

impl fmt::Display for StrStructEncoding {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.0, formatter)
    }
}

struct StrFields<'a> {
    fields: &'a str,
}

impl<'a> Iterator for StrFields<'a> {
    type Item = &'a StrEncoding;

    fn next(&mut self) -> Option<&'a StrEncoding> {
        if self.fields.is_empty() {
            None
        } else {
            let (h, t) = chomp(self.fields);
            self.fields = t;
            Some(StrEncoding::new_unchecked(h.unwrap()))
        }
    }
}

impl<'a> FieldsComparator for StrFields<'a> {
    fn eq_next<E: ?Sized + Encoding>(&mut self, other: &E) -> bool {
        self.next().map_or(false, |e| e.eq_encoding(other))
    }

    fn is_finished(&self) -> bool {
        self.fields.is_empty()
    }
}

pub struct StringEncoding<S> where S: AsRef<str> {
    buf: S,
}

impl<S> StringEncoding<S> where S: AsRef<str> {
    pub fn new_unchecked(s: S) -> StringEncoding<S> {
        StringEncoding { buf: s }
    }
}

impl<S> Deref for StringEncoding<S> where S: AsRef<str> {
    type Target = StrEncoding;

    fn deref(&self) -> &StrEncoding {
        StrEncoding::new_unchecked(self.buf.as_ref())
    }
}

impl<S> Encoding for StringEncoding<S> where S: AsRef<str> {
    type Pointer = StrPointerEncoding;
    type Struct = StrStructEncoding;

    fn descriptor(&self) -> Descriptor<StrPointerEncoding, StrStructEncoding> {
        (**self).descriptor()
    }

    fn eq_encoding<T: ?Sized + Encoding>(&self, other: &T) -> bool {
        self.descriptor().eq_encoding(other)
    }
}

impl<S> fmt::Display for StringEncoding<S> where S: AsRef<str> {
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

    #[test]
    fn test_parsed_struct() {
        let s = StrStructEncoding::new_unchecked("{CGPoint=ci}");
        assert_eq!(s.name(), "CGPoint");

        let mut fields = s.fields();
        assert_eq!(fields.next().unwrap().to_string(), "c");
        assert_eq!(fields.next().unwrap().to_string(), "i");
        assert!(fields.next().is_none());
    }
}
