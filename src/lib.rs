mod descriptor;
pub mod encodings;
pub mod parse;

use std::fmt;

pub use descriptor::Descriptor;

pub trait Encoding: fmt::Display {
    fn descriptor(&self) -> Descriptor;
}
