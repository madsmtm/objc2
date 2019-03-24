use core::fmt;
use core::mem;

use crate::Encoding;
use crate::multi::{Encodings, EncodingsIterateCallback};
use super::chomp;
use super::encoding::StrEncoding;

#[derive(Debug)]
pub struct StrEncodings(str);

impl StrEncodings {
    pub fn from_str_unchecked(s: &str) -> &StrEncodings {
        unsafe { mem::transmute(s) }
    }
}

impl Encodings for StrEncodings {
    fn each<F: EncodingsIterateCallback>(&self, callback: &mut F) {
        for enc in StrEncodingsIter::new(self) {
            if callback.call(enc) { break; }
        }
    }

    fn eq_encodings<T: ?Sized + Encodings>(&self, encs: &T) -> bool {
        let mut comparator = StrEncodingsComparator::new(self);
        encs.each(&mut comparator);
        comparator.was_equal()
    }

    fn write_all<W: fmt::Write>(&self, formatter: &mut W) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

struct StrEncodingsIter<'a> {
    fields: &'a str,
}

impl<'a> StrEncodingsIter<'a> {
    fn new(fields: &StrEncodings) -> StrEncodingsIter {
        StrEncodingsIter { fields: &fields.0 }
    }
}

impl<'a> Iterator for StrEncodingsIter<'a> {
    type Item = &'a StrEncoding;

    fn next(&mut self) -> Option<&'a StrEncoding> {
        if self.fields.is_empty() {
            None
        } else {
            let (h, t) = match chomp(self.fields) {
                Some((h, t)) => (h, t),
                None => panic!("Failed to parse an encoding from {:?}", self.fields),
            };
            self.fields = t;
            Some(StrEncoding::from_str_unchecked(h))
        }
    }
}

struct StrEncodingsComparator<'a> {
    iter: StrEncodingsIter<'a>,
    all_equal: bool,
}

impl<'a> StrEncodingsComparator<'a> {
    fn new(fields: &StrEncodings) -> StrEncodingsComparator {
        StrEncodingsComparator {
            iter: StrEncodingsIter::new(fields),
            all_equal: true,
        }
    }

    fn was_equal(mut self) -> bool {
        self.all_equal && self.iter.next().is_none()
    }
}

impl<'a> EncodingsIterateCallback for StrEncodingsComparator<'a> {
    fn call<T: ?Sized + Encoding>(&mut self, encoding: &T) -> bool {
        if !self.iter.next().map_or(false, |e| e == encoding) {
            self.all_equal = false;
            // stop iteration
            true
        } else {
            // don't stop iteration
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iter_fields() {
        let fields = StrEncodings::from_str_unchecked("ci");
        let mut fields = StrEncodingsIter::new(fields);
        assert_eq!(fields.next().unwrap().as_str(), "c");
        assert_eq!(fields.next().unwrap().as_str(), "i");
        assert!(fields.next().is_none());
    }
}
