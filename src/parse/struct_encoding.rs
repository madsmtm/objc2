use std::fmt;
use std::mem;

use encoding::{Descriptor, Encoding, StructEncoding, Never};
use multi::Encodings;
use super::parse_struct;
use super::multi::{StrFields, StrFieldsIter};

pub struct StrStructEncoding(str);

impl StrStructEncoding {
    pub fn from_str_unchecked(s: &str) -> &StrStructEncoding {
        unsafe { mem::transmute(s) }
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
            let (name, fields) = self.fields();
            let (other_name, other_fields) = s.fields();
            other_name == name && other_fields.eq(StrFieldsIter::new(fields))
        } else {
            false
        }
    }
}

impl StructEncoding for StrStructEncoding {
    type Fields = StrFields;

    fn fields(&self) -> (&str, &StrFields) {
        let (name, fields) = parse_struct(&self.0).unwrap();
        (name, StrFields::from_str_unchecked(fields))
    }
}

impl fmt::Display for StrStructEncoding {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.0, formatter)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsed_struct() {
        let s = StrStructEncoding::from_str_unchecked("{CGPoint=ci}");

        let (name, fields) = s.fields();
        assert_eq!(name, "CGPoint");

        let mut fields = StrFieldsIter::new(fields);
        assert_eq!(fields.next().unwrap().to_string(), "c");
        assert_eq!(fields.next().unwrap().to_string(), "i");
        assert!(fields.next().is_none());
    }
}
