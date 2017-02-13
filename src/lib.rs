mod descriptor;
mod encodings;
mod multi;
mod parse;

use std::fmt;

pub use descriptor::Descriptor;
pub use encodings::{Primitive, Pointer, Struct};
pub use multi::{Encodings, EncodingsComparator};
pub use parse::StrEncoding;

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
