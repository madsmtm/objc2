#![no_std]

#[cfg(test)]
extern crate std;

pub mod encoding;
mod encode;
mod multi;
mod parse;

use core::fmt;

pub use encode::Encode;
pub use multi::{Encodings, EncodingsComparator, IndexEncodings};
pub use parse::{StrEncoding, ParseEncodingError};

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

#[derive(Clone, Copy, Debug)]
pub enum Descriptor<'a, T, I, F, M>
        where T: 'a + ?Sized + Encoding,
              I: 'a + ?Sized + Encoding,
              F: 'a + ?Sized + Encodings,
              M: 'a + ?Sized + Encodings {
    Primitive(encoding::Primitive),
    Pointer(&'a T),
    Array(u32, &'a I),
    Struct(&'a str, &'a F),
    Union(&'a str, &'a M),
}
