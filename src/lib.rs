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
