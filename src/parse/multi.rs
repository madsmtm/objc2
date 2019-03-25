use super::chomp;
use super::encoding::StrEncoding;

pub struct StrEncodingsIter<'a> {
    fields: &'a str,
}

impl<'a> StrEncodingsIter<'a> {
    pub fn new(fields: &str) -> StrEncodingsIter {
        StrEncodingsIter { fields }
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
        let mut fields = StrEncodingsIter::new("ci");
        assert_eq!(fields.next().unwrap().as_str(), "c");
        assert_eq!(fields.next().unwrap().as_str(), "i");
        assert!(fields.next().is_none());
    }
}
