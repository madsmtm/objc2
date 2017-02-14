use std::fmt;

use super::{Descriptor, Encoding, PointerEncoding, StructEncoding};
use multi::EncodingsComparator;

pub enum Never { }

impl Encoding for Never {
    type Pointer = Never;
    type Struct = Never;

    fn descriptor(&self) -> Descriptor<Never, Never> {
        match self { }
    }

    fn eq_encoding<T: ?Sized + Encoding>(&self, _: &T) -> bool {
        match self { }
    }
}

impl PointerEncoding for Never {
    type Target = Never;

    fn target(&self) -> &Never {
        match self { }
    }
}

impl StructEncoding for Never {
    fn name(&self) -> &str {
        match self { }
    }

    fn eq_struct<C: EncodingsComparator>(&self, _: &str, _: C) -> bool {
        match self { }
    }
}

impl fmt::Display for Never  {
    fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
        match self { }
    }
}
