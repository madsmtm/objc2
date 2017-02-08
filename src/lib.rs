use std::fmt;
use descriptor::Descriptor;

pub mod descriptor;
pub mod encodings;
pub mod parse;

pub trait Encoding: fmt::Display {
    fn descriptor(&self) -> Descriptor;
}
