use std::fmt;

use super::{Descriptor, Encoding, StructEncoding, Never};
use multi::{Encodings, EncodingsComparator, EncodingTupleComparator};

pub struct Struct<S, T> where S: AsRef<str>, T: Encodings {
    name: S,
    fields: T,
}

impl<S, T> Struct<S, T> where S: AsRef<str>, T: Encodings {
    pub fn new(name: S, fields: T) -> Struct<S, T> {
        Struct { name: name, fields: fields }
    }
}

impl<S, T> Encoding for Struct<S, T> where S: AsRef<str>, T: Encodings {
    type Pointer = Never;
    type Struct = Self;

    fn descriptor(&self) -> Descriptor<Never, Self> {
        Descriptor::Struct(self)
    }

    fn eq_encoding<E: ?Sized + Encoding>(&self, other: &E) -> bool {
        if let Descriptor::Struct(s) = other.descriptor() {
            s.eq_struct(self.name(), EncodingTupleComparator::new(&self.fields))
        } else {
            false
        }
    }
}

impl<S, T> StructEncoding for Struct<S, T> where S: AsRef<str>, T: Encodings {
    fn name(&self) -> &str {
        self.name.as_ref()
    }

    fn eq_struct<C: EncodingsComparator>(&self, name: &str, fields: C) -> bool {
        self.name() == name && self.fields.eq(fields)
    }
}

impl<S, T> fmt::Display for Struct<S, T> where S: AsRef<str>, T: Encodings {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{{{}=", self.name())?;
        self.fields.write_all(formatter)?;
        write!(formatter, "}}")
    }
}
