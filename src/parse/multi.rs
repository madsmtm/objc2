use core::mem;

use super::chomp;
use super::encoding::StrEncoding;

#[derive(Debug)]
pub struct StrEncodings(str);

impl StrEncodings {
    pub fn from_str_unchecked(s: &str) -> &StrEncodings {
        unsafe { mem::transmute(s) }
    }

    pub fn iter<'a>(&'a self) -> impl Iterator<Item=&'a StrEncoding> {
        StrEncodingsIter::new(self)
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
