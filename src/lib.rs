mod descriptor;
mod encodings;
mod parse;

use std::fmt;

pub use descriptor::Descriptor;
pub use encodings::{Primitive, Pointer, Struct, EncodingTuple};
pub use parse::StrEncoding;

pub trait Encoding: fmt::Display {
    fn descriptor(&self) -> Descriptor;
}

pub trait FieldsComparator {
    fn eq_next<T: ?Sized + Encoding>(&mut self, &T) -> bool;
    fn is_finished(&self) -> bool;
}

pub trait StructEncoding: Encoding {
    fn name(&self) -> &str;
    fn fields_eq<T: FieldsComparator>(&self, T) -> bool;
}

pub trait PointerEncoding: Encoding {
    type Pointee: Encoding;

    fn pointee(&self) -> &Self::Pointee;
}
