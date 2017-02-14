mod never;
mod pointer;
mod primitive;
mod structure;

use std::fmt;

use multi::Encodings;

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

pub trait PointerEncoding: Encoding {
    type Target: ?Sized + Encoding;

    fn target(&self) -> &Self::Target;
}

pub trait StructEncoding: Encoding {
    type Fields: ?Sized + Encodings;

    fn fields(&self) -> (&str, &Self::Fields);
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
