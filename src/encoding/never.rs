use core::fmt;

use crate::{Descriptor, Encoding};
use crate::multi::{Encodings, EncodingsIterateCallback};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Never { }

impl Encoding for Never {
    type PointerTarget = Never;
    type ArrayItem = Never;
    type StructFields = Never;
    type UnionMembers = Never;

    fn descriptor(&self) -> Descriptor<Never, Never, Never, Never> {
        match *self { }
    }
}

impl Encodings for Never {
    fn each<F: EncodingsIterateCallback>(&self, _: &mut F) {
        match *self { }
    }

    fn eq_encodings<E: ?Sized + Encodings>(&self, _: &E) -> bool {
        match *self { }
    }
}

impl fmt::Display for Never  {
    fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
        match *self { }
    }
}
