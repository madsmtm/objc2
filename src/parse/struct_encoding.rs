use std::fmt;
use std::mem;

use {Encoding, StructEncoding, EncodingsComparator};
use descriptor::Descriptor;
use encodings::Never;
use super::{chomp, parse_struct};
use super::encoding::StrEncoding;

pub struct StrStructEncoding(str);

impl StrStructEncoding {
    pub fn from_str_unchecked(s: &str) -> &StrStructEncoding {
        unsafe { mem::transmute(s) }
    }

    fn contents(&self) -> (&str, StrFields) {
        let (name, fields) = parse_struct(&self.0).unwrap();
        (name, StrFields { fields: fields })
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
            let (name, fields) = self.contents();
            s.eq_struct(name, fields)
        } else {
            false
        }
    }
}

impl StructEncoding for StrStructEncoding {
    fn name(&self) -> &str {
        self.contents().0
    }

    fn eq_struct<C: EncodingsComparator>(&self, other_name: &str, other_fields: C) -> bool {
        let (name, fields) = self.contents();
        name == other_name && fields.eq(other_fields)
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

impl<'a> StrFields<'a> {
    fn eq<C: EncodingsComparator>(self, mut other: C) -> bool {
        for enc in self {
            if !other.eq_next(enc) {
                return false;
            }
        }
        other.is_finished()
    }
}

impl<'a> Iterator for StrFields<'a> {
    type Item = &'a StrEncoding;

    fn next(&mut self) -> Option<&'a StrEncoding> {
        if self.fields.is_empty() {
            None
        } else {
            let (h, t) = chomp(self.fields);
            self.fields = t;
            Some(StrEncoding::from_str_unchecked(h.unwrap()))
        }
    }
}

impl<'a> EncodingsComparator for StrFields<'a> {
    fn eq_next<E: ?Sized + Encoding>(&mut self, other: &E) -> bool {
        self.next().map_or(false, |e| e.eq_encoding(other))
    }

    fn is_finished(&self) -> bool {
        self.fields.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsed_struct() {
        let s = StrStructEncoding::from_str_unchecked("{CGPoint=ci}");

        let (name, mut fields) = s.contents();
        assert_eq!(name, "CGPoint");
        assert_eq!(fields.next().unwrap().to_string(), "c");
        assert_eq!(fields.next().unwrap().to_string(), "i");
        assert!(fields.next().is_none());
    }
}
