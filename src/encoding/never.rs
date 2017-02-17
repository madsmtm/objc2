use core::fmt;

use {Descriptor, Encoding};
use multi::{Encodings, EncodingIterateCallback};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Never { }

impl Encoding for Never {
    type PointerTarget = Never;
    type ArrayItem = Never;
    type StructFields = Never;
    type UnionMembers = Never;

    fn descriptor(&self) -> Descriptor<Never, Never, Never, Never> {
        match self { }
    }

    fn eq_encoding<T: ?Sized + Encoding>(&self, _: &T) -> bool {
        match self { }
    }
}

impl Encodings for Never {
    fn each<F: EncodingIterateCallback>(&self, _: &mut F) {
        match self { }
    }

    fn eq_encodings<E: ?Sized + Encodings>(&self, _: &E) -> bool {
        match self { }
    }

    fn write_all<W: fmt::Write>(&self, _: &mut W) -> fmt::Result {
        match self { }
    }
}

impl fmt::Display for Never  {
    fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
        match self { }
    }
}
