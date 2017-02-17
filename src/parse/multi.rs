use core::fmt;
use core::mem;

use Encoding;
use multi::{Encodings, EncodingsIterateCallback};
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
    fn each<F: EncodingsIterateCallback>(&self, callback: &mut F) {
        for enc in StrFieldsIter::new(self) {
            if callback.call(enc) { break; }
        }
    }

    fn eq_encodings<T: ?Sized + Encodings>(&self, encs: &T) -> bool {
        let mut comparator = StrFieldsComparator::new(self);
        encs.each(&mut comparator);
        comparator.was_equal()
    }

    fn write_all<W: fmt::Write>(&self, formatter: &mut W) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

struct StrFieldsIter<'a> {
    fields: &'a str,
}

impl<'a> StrFieldsIter<'a> {
    fn new(fields: &StrFields) -> StrFieldsIter {
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

struct StrFieldsComparator<'a> {
    iter: StrFieldsIter<'a>,
    all_equal: bool,
}

impl<'a> StrFieldsComparator<'a> {
    fn new(fields: &StrFields) -> StrFieldsComparator {
        StrFieldsComparator {
            iter: StrFieldsIter::new(fields),
            all_equal: true,
        }
    }

    fn was_equal(mut self) -> bool {
        self.all_equal && self.iter.next().is_none()
    }
}

impl<'a> EncodingsIterateCallback for StrFieldsComparator<'a> {
    fn call<T: ?Sized + Encoding>(&mut self, encoding: &T) -> bool {
        if !self.iter.next().map_or(false, |e| e.eq_encoding(encoding)) {
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
        let fields = StrFields::from_str_unchecked("ci");
        let mut fields = StrFieldsIter::new(fields);
        assert_eq!(fields.next().unwrap().as_str(), "c");
        assert_eq!(fields.next().unwrap().as_str(), "i");
        assert!(fields.next().is_none());
    }
}
