use std::fmt;

use super::{Descriptor, Encoding, Never};
use multi::{Encodings, IndexEncodings, IndexEncodingsComparator};

pub struct Struct<S, T> where S: AsRef<str>, T: IndexEncodings {
    name: S,
    fields: T,
}

impl<S, T> Struct<S, T> where S: AsRef<str>, T: IndexEncodings {
    pub fn new(name: S, fields: T) -> Struct<S, T> {
        Struct { name: name, fields: fields }
    }

    fn name(&self) -> &str {
        self.name.as_ref()
    }
}

impl<S, T> Encoding for Struct<S, T> where S: AsRef<str>, T: IndexEncodings {
    type PointerTarget = Never;
    type StructFields = T;

    fn descriptor(&self) -> Descriptor<Never, T> {
        Descriptor::Struct(self.name(), &self.fields)
    }

    fn eq_encoding<E: ?Sized + Encoding>(&self, other: &E) -> bool {
        if let Descriptor::Struct(name, fields) = other.descriptor() {
            name == self.name() &&
                fields.eq(IndexEncodingsComparator::new(&self.fields))
        } else {
            false
        }
    }
}

impl<S, T> fmt::Display for Struct<S, T> where S: AsRef<str>, T: IndexEncodings {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{{{}=", self.name())?;
        self.fields.write_all(formatter)?;
        write!(formatter, "}}")
    }
}

#[cfg(test)]
mod tests {
    use encoding::Primitive;
    use parse::StrEncoding;
    use super::*;

    #[test]
    fn test_static_struct() {
        let f = (Primitive::Char, Primitive::Int);
        let s = Struct::new("CGPoint", f);
        assert_eq!(s.name(), "CGPoint");
        assert_eq!(s.to_string(), "{CGPoint=ci}");
    }

    #[test]
    fn test_eq_encoding() {
        let i = Primitive::Int;
        let c = Primitive::Char;

        let s = Struct::new("CGPoint", (c, i));
        assert!(s.eq_encoding(&s));
        assert!(!s.eq_encoding(&i));

        let s2 = StrEncoding::new_unchecked("{CGPoint=ci}");
        assert!(s2.eq_encoding(&s2));
        assert!(s.eq_encoding(&s2));
    }
}
