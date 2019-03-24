use core::fmt;

use crate::{Descriptor, Encoding};
use super::never::Never;

/// An encoding for a pointer.
#[derive(Clone, Copy, Debug)]
pub struct Pointer<T>(T) where T: Encoding;

impl<T> Pointer<T> where T: Encoding {
    /// Constructs an encoding for a pointer to a target with the given encoding.
    pub fn new(target: T) -> Pointer<T> {
        Pointer(target)
    }
}

impl<T> Encoding for Pointer<T> where T: Encoding {
    type PointerTarget = T;
    type ArrayItem = Never;
    type StructFields = Never;
    type UnionMembers = Never;

    fn descriptor(&self) -> Descriptor<T, Never, Never, Never> {
        Descriptor::Pointer(&self.0)
    }
}

impl<T> fmt::Display for Pointer<T> where T: Encoding {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        self.write(formatter)
    }
}

impl<T, E: ?Sized> PartialEq<E> for Pointer<T> where T: Encoding, E: Encoding {
    fn eq(&self, other: &E) -> bool {
        self.eq_encoding(other)
    }
}

#[cfg(test)]
mod tests {
    use std::string::ToString;
    use crate::encoding::Primitive;
    use super::*;

    #[test]
    fn test_pointer_display() {
        let e = Pointer::new(Primitive::Int);
        assert_eq!(e.to_string(), "^i");
    }

    #[test]
    fn test_eq_encoding() {
        let i = Primitive::Int;

        let p = Pointer::new(i);
        assert!(p == p);
        assert!(p != i);
    }
}
