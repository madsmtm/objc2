use std::fmt;

pub trait Encoding: fmt::Display {
    fn as_primitive(&self) -> Option<Primitive> { None }
    fn as_pointer(&self) -> Option<(&Encoding, bool)> { None }
}

#[derive(Clone, Copy)]
pub enum Primitive {
    Char,
    Short,
    Int,
    LongLong,
    UChar,
    UShort,
    UInt,
    ULongLong,
    Float,
    Double,
    Bool,
    Void,
    Object,
    Class,
    Sel,
    Unknown,
}

impl Encoding for Primitive {
    fn as_primitive(&self) -> Option<Primitive> {
        Some(*self)
    }
}

impl fmt::Display for Primitive {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let s = match *self {
            Primitive::Int => "i",
            _ => panic!(),
        };
        fmt::Display::fmt(s, formatter)
    }
}

pub struct Pointer<T> where T: Encoding {
    t: T,
    is_const: bool,
}

impl<T> Encoding for Pointer<T> where T: Encoding {
    fn as_pointer(&self) -> Option<(&Encoding, bool)> {
        Some((&self.t, self.is_const))
    }
}

impl<T> fmt::Display for Pointer<T> where T: Encoding {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{}^{}", if self.is_const {"r"} else {""}, self.t)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_int_display() {
        assert_eq!(Primitive::Int.to_string(), "i");
    }

    #[test]
    fn test_pointer_display() {
        let e = Pointer { t: Primitive::Int, is_const: false };
        assert_eq!(e.to_string(), "^i");
    }
}
