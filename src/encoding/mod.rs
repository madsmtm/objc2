mod never;
mod pointer;
mod primitive;
mod structure;

use std::fmt;

use multi::EncodingsComparator;

pub use self::never::Never;
pub use self::pointer::Pointer;
pub use self::primitive::Primitive;
pub use self::structure::Struct;

pub trait Encoding: fmt::Display {
    type Pointer: ?Sized + PointerEncoding;
    type Struct: ?Sized + StructEncoding;

    fn descriptor(&self) -> Descriptor<Self::Pointer, Self::Struct>;

    fn eq_encoding<T: ?Sized + Encoding>(&self, &T) -> bool;
}

pub trait StructEncoding: Encoding {
    fn name(&self) -> &str;
    fn eq_struct<T: EncodingsComparator>(&self, name: &str, fields: T) -> bool;
}

pub trait PointerEncoding: Encoding {
    type Pointee: ?Sized + Encoding;

    fn pointee(&self) -> &Self::Pointee;
}

pub enum Descriptor<'a, P, S>
        where P: 'a + ?Sized + PointerEncoding,
              S: 'a + ?Sized + StructEncoding {
    Primitive(Primitive),
    Pointer(&'a P),
    Struct(&'a S),
}

impl<'a, P, S> Descriptor<'a, P, S>
        where P: 'a + ?Sized + PointerEncoding,
              S: 'a + ?Sized + StructEncoding {
    pub fn eq_encoding<T: ?Sized + Encoding>(&self, other: &T) -> bool {
        match *self {
            Descriptor::Primitive(p) => p.eq_encoding(other),
            Descriptor::Pointer(p) => p.eq_encoding(other),
            Descriptor::Struct(s) => s.eq_encoding(other),
        }
    }
}
