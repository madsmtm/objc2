use std::fmt;

use multi::{Encodings, EncodingsComparator};
use super::{Descriptor, Encoding};

pub enum Never { }

impl Encoding for Never {
    type PointerTarget = Never;
    type StructFields = Never;

    fn descriptor(&self) -> Descriptor<Never, Never> {
        match self { }
    }

    fn eq_encoding<T: ?Sized + Encoding>(&self, _: &T) -> bool {
        match self { }
    }
}

impl Encodings for Never {
    fn eq<C: EncodingsComparator>(&self, _: C) -> bool {
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
