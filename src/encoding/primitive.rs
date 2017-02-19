use core::fmt;

use {Descriptor, Encoding};
use super::never::Never;

/// An encoding for a "primitive" type which is not a composition of other types.
#[derive(Clone, Copy, Debug)]
pub enum Primitive {
    Char,
    Short,
    Int,
    Long,
    LongLong,
    UChar,
    UShort,
    UInt,
    ULong,
    ULongLong,
    Float,
    Double,
    Bool,
    Void,
    String,
    Object,
    Block,
    Class,
    Sel,
    Unknown,
    BitField(u32),
}

impl Encoding for Primitive {
    type PointerTarget = Never;
    type ArrayItem = Never;
    type StructFields = Never;
    type UnionMembers = Never;

    fn descriptor(&self) -> Descriptor<Never, Never, Never, Never> {
        Descriptor::Primitive(*self)
    }
}

impl fmt::Display for Primitive {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        self.write(formatter)
    }
}

impl PartialEq for Primitive {
    fn eq(&self, other: &Primitive) -> bool {
        self.eq_encoding(other)
    }
}

impl Eq for Primitive { }

#[cfg(test)]
mod tests {
    use std::string::ToString;
    use super::*;

    #[test]
    fn test_int_display() {
        assert_eq!(Primitive::Int.to_string(), "i");
    }

    #[test]
    fn test_eq_encoding() {
        let i = Primitive::Int;
        let c = Primitive::Char;

        assert!(i.eq_encoding(&i));
        assert!(!i.eq_encoding(&c));
    }
}
