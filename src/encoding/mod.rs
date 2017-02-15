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
    type ArrayItem: ?Sized + Encoding;
    type StructFields: ?Sized + Encodings;
    type UnionMembers: ?Sized + Encodings;

    fn descriptor(&self) -> Descriptor<Self::PointerTarget,
                                       Self::ArrayItem,
                                       Self::StructFields,
                                       Self::UnionMembers>;

    fn eq_encoding<T: ?Sized + Encoding>(&self, &T) -> bool;
}

pub enum Descriptor<'a, T, I, F, M>
        where T: 'a + ?Sized + Encoding,
              I: 'a + ?Sized + Encoding,
              F: 'a + ?Sized + Encodings,
              M: 'a + ?Sized + Encodings {
    Primitive(Primitive),
    Pointer(&'a T),
    Array(u32, &'a I),
    Struct(&'a str, &'a F),
    Union(&'a str, &'a M),
}
