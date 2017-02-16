use std::fmt;

use super::{Descriptor, Encoding, Never};
use multi::{Encodings, IndexEncodings, IndexEncodingsComparator};

#[derive(Clone, Copy, Debug)]
pub struct Union<S, T> where S: AsRef<str>, T: IndexEncodings {
    name: S,
    members: T,
}

impl<S, T> Union<S, T> where S: AsRef<str>, T: IndexEncodings {
    pub fn new(name: S, members: T) -> Union<S, T> {
        Union { name: name, members: members }
    }

    fn name(&self) -> &str {
        self.name.as_ref()
    }
}

impl<S, T> Encoding for Union<S, T> where S: AsRef<str>, T: IndexEncodings {
    type PointerTarget = Never;
    type ArrayItem = Never;
    type StructFields = Never;
    type UnionMembers = T;

    fn descriptor(&self) -> Descriptor<Never, Never, Never, T> {
        Descriptor::Union(self.name(), &self.members)
    }

    fn eq_encoding<E: ?Sized + Encoding>(&self, other: &E) -> bool {
        if let Descriptor::Union(name, members) = other.descriptor() {
            name == self.name() &&
                members.eq(IndexEncodingsComparator::new(&self.members))
        } else {
            false
        }
    }
}

impl<S, T> fmt::Display for Union<S, T> where S: AsRef<str>, T: IndexEncodings {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "({}=", self.name())?;
        self.members.write_all(formatter)?;
        write!(formatter, ")")
    }
}

#[cfg(test)]
mod tests {
    use encoding::Primitive;
    use parse::StrEncoding;
    use super::*;

    #[test]
    fn test_union_display() {
        let f = (Primitive::Char, Primitive::Int);
        let s = Union::new("Onion", f);
        assert_eq!(s.to_string(), "(Onion=ci)");
    }

    #[test]
    fn test_eq_encoding() {
        let i = Primitive::Int;
        let c = Primitive::Char;

        let u = Union::new("Onion", (c, i));
        assert!(u.eq_encoding(&u));
        assert!(!u.eq_encoding(&i));

        let u2 = StrEncoding::new("(Onion=ci)").unwrap();
        assert!(u.eq_encoding(&u2));
    }
}
