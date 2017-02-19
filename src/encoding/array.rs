use core::fmt;

use {Descriptor, Encoding};
use super::never::Never;

/// An encoding for an array.
#[derive(Clone, Copy, Debug)]
pub struct Array<T> where T: Encoding {
    len: u32,
    item: T,
}

impl<T> Array<T> where T: Encoding {
    /// Constructs an encoding for an array with the given length and of items
    /// with the given encoding.
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
}

impl<T> fmt::Display for Array<T> where T: Encoding {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        self.write(formatter)
    }
}

impl<T, E> PartialEq<E> for Array<T> where T: Encoding, E: ?Sized + Encoding {
    fn eq(&self, other: &E) -> bool {
        self.eq_encoding(other)
    }
}

#[cfg(test)]
mod tests {
    use std::string::ToString;
    use encoding::Primitive;
    use super::*;

    #[test]
    fn test_array_display() {
        let e = Array::new(12, Primitive::Int);
        assert_eq!(e.to_string(), "[12i]");
    }
}
