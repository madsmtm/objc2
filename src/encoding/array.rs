use std::fmt;

use super::{Descriptor, Encoding, Never};

pub struct Array<T> where T: Encoding {
    len: u32,
    item: T,
}

impl<T> Array<T> where T: Encoding {
    pub fn new(len: u32, item: T) -> Array<T> {
        Array { len: len, item: item }
    }
}

impl<T> Encoding for Array<T> where T: Encoding {
    type PointerTarget = Never;
    type ArrayItem = T;
    type StructFields = Never;
    type UnionMembers = Never;

    fn descriptor(&self) -> Descriptor<Never, T, Never, Never> {
        Descriptor::Array(self.len, &self.item)
    }

    fn eq_encoding<E: ?Sized + Encoding>(&self, other: &E) -> bool {
        if let Descriptor::Array(len, item) = other.descriptor() {
            self.len == len && self.item.eq_encoding(item)
        } else {
            false
        }
    }
}

impl<T> fmt::Display for Array<T> where T: Encoding {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "[{}^{}]", self.len, self.item)
    }
}

#[cfg(test)]
mod tests {
    use encoding::Primitive;
    use super::*;

    #[test]
    fn test_array_display() {
        let e = Array::new(12, Primitive::Int);
        assert_eq!(e.to_string(), "[12^i]");
    }
}
