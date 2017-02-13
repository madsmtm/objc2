use std::fmt;

use super::{Descriptor, Encoding, PointerEncoding, Never};

pub struct Pointer<T>(T) where T: Encoding;

impl<T> Pointer<T> where T: Encoding {
    pub fn new(pointee: T) -> Pointer<T> {
        Pointer(pointee)
    }
}

impl<T> Encoding for Pointer<T> where T: Encoding {
    type Pointer = Self;
    type Struct = Never;

    fn descriptor(&self) -> Descriptor<Self, Never> {
        Descriptor::Pointer(self)
    }

    fn eq_encoding<E: ?Sized + Encoding>(&self, other: &E) -> bool {
        if let Descriptor::Pointer(p) = other.descriptor() {
            self.0.eq_encoding(p.pointee())
        } else {
            false
        }
    }
}

impl<T> PointerEncoding for Pointer<T> where T: Encoding {
    type Pointee = T;

    fn pointee(&self) -> &T {
        &self.0
    }
}

impl<T> fmt::Display for Pointer<T> where T: Encoding {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "^{}", self.0)
    }
}
