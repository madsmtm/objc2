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
    type PointerTarget: ?Sized + Encoding;
    type StructFields: ?Sized + Encodings;

    fn descriptor(&self) -> Descriptor<Self::PointerTarget, Self::StructFields>;

    fn eq_encoding<T: ?Sized + Encoding>(&self, &T) -> bool;
}

pub enum Descriptor<'a, T, F>
        where T: 'a + ?Sized + Encoding,
              F: 'a + ?Sized + Encodings {
    Primitive(Primitive),
    Pointer(&'a T),
    Struct(&'a str, &'a F),
}
