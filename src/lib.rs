mod descriptor;
mod encodings;
mod parse;

use std::fmt;

pub use descriptor::Descriptor;
pub use encodings::{Primitive, Pointer, Struct, EncodingTuple};
pub use parse::StrEncoding;

pub trait Encoding: fmt::Display {
    type Pointer: ?Sized + PointerEncoding;
    type Struct: ?Sized + StructEncoding;

    fn descriptor(&self) -> Descriptor<Self::Pointer, Self::Struct>;

    fn eq_encoding<T: ?Sized + Encoding>(&self, &T) -> bool;
}

pub trait FieldsComparator {
    fn eq_next<T: ?Sized + Encoding>(&mut self, &T) -> bool;
    fn is_finished(&self) -> bool;
}

pub trait StructEncoding: Encoding {
    fn name(&self) -> &str;
    fn eq_struct<T: FieldsComparator>(&self, name: &str, fields: T) -> bool;
}

pub trait PointerEncoding: Encoding {
    type Pointee: ?Sized + Encoding;

    fn pointee(&self) -> &Self::Pointee;
}
