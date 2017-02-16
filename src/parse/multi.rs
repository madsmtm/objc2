use std::fmt;
use std::mem;

use encoding::Encoding;
use multi::{Encodings, EncodingsComparator};
use super::chomp;
use super::encoding::StrEncoding;

#[derive(Debug)]
pub struct StrFields(str);

impl StrFields {
    pub fn from_str_unchecked(s: &str) -> &StrFields {
        unsafe { mem::transmute(s) }
    }
}

impl Encodings for StrFields {
    fn eq<C: EncodingsComparator>(&self, mut comparator: C) -> bool {
        for enc in StrFieldsIter::new(self) {
            if !comparator.eq_next(enc) {
                return false;
            }
        }
        comparator.is_finished()
    }

    fn write_all<W: fmt::Write>(&self, formatter: &mut W) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

pub struct StrFieldsIter<'a> {
    fields: &'a str,
}

impl<'a> StrFieldsIter<'a> {
    pub fn new(fields: &StrFields) -> StrFieldsIter {
        StrFieldsIter { fields: &fields.0 }
    }
}

impl<'a> Iterator for StrFieldsIter<'a> {
    type Item = &'a StrEncoding;

    fn next(&mut self) -> Option<&'a StrEncoding> {
        if self.fields.is_empty() {
            None
        } else {
            let (h, t) = match chomp(self.fields) {
                (Some(h), t) => (h, t),
                (None, t) => panic!("Failed to parse an encoding from {:?}", t),
            };
            self.fields = t;
            Some(StrEncoding::from_str_unchecked(h))
        }
    }
}

impl<'a> EncodingsComparator for StrFieldsIter<'a> {
    fn eq_next<E: ?Sized + Encoding>(&mut self, other: &E) -> bool {
        self.next().map_or(false, |e| e.eq_encoding(other))
    }

    fn is_finished(&self) -> bool {
        self.fields.is_empty()
    }
}