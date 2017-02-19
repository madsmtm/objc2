use core::fmt;

use {Descriptor, Encoding};
use multi::Encodings;
use super::never::Never;

/// An encoding for a struct.
#[derive(Clone, Copy, Debug)]
pub struct Struct<S, T> where S: AsRef<str>, T: Encodings {
    name: S,
    fields: T,
}

impl<S, T> Struct<S, T> where S: AsRef<str>, T: Encodings {
    /// Constructs an encoding for a struct with the given name and
    /// fields with the given encodings.
    pub fn new(name: S, fields: T) -> Struct<S, T> {
        Struct { name: name, fields: fields }
    }

    fn name(&self) -> &str {
        self.name.as_ref()
    }
}

impl<S, T> Encoding for Struct<S, T> where S: AsRef<str>, T: Encodings {
    type PointerTarget = Never;
    type ArrayItem = Never;
    type StructFields = T;
    type UnionMembers = Never;

    fn descriptor(&self) -> Descriptor<Never, Never, T, Never> {
        Descriptor::Struct(self.name(), &self.fields)
    }
}

impl<S, T> fmt::Display for Struct<S, T> where S: AsRef<str>, T: Encodings {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        self.write(formatter)
    }
}

impl<S, T, E> PartialEq<E> for Struct<S, T>
        where S: AsRef<str>, T: Encodings, E: ?Sized + Encoding {
    fn eq(&self, other: &E) -> bool {
        self.eq_encoding(other)
    }
}

#[cfg(test)]
mod tests {
    use std::string::ToString;
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
        assert!(s == s);
        assert!(s != i);

        let s2 = StrEncoding::new_unchecked("{CGPoint=ci}");
        assert!(s2 == s2);
        assert!(s == s2);
    }
}
