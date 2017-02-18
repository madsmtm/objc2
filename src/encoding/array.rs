use core::fmt;

use {Descriptor, Encoding};
use super::never::Never;

#[derive(Clone, Copy, Debug)]
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
}

impl<T> fmt::Display for Array<T> where T: Encoding {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        self.write(formatter)
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
