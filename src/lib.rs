#![no_std]

#[cfg(test)]
extern crate std;

pub mod encoding;
mod encode;
mod multi;
mod parse;

use core::fmt;

pub use encode::Encode;
pub use multi::{Encodings, EncodingsIterateCallback};
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

    fn eq_encoding<T: ?Sized + Encoding>(&self, other: &T) -> bool {
        use Descriptor::*;
        match (self.descriptor(), other.descriptor()) {
            (Primitive(p1), Primitive(p2)) => p1 == p2,
            (Pointer(t1), Pointer(t2)) => t1.eq_encoding(t2),
            (Array(l1, i1), Array(l2, i2)) =>
                l1 == l2 && i1.eq_encoding(i2),
            (Struct(n1, f1), Struct(n2, f2)) =>
                n1 == n2 && f1.eq_encodings(f2),
            (Union(n1, m1), Union(n2, m2)) =>
                n1 == n2 && m1.eq_encodings(m2),
            _ => false,
        }
    }
}

#[derive(Debug)]
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

impl<'a, T, I, F, M> Copy for Descriptor<'a, T, I, F, M>
        where T: 'a + ?Sized + Encoding,
              I: 'a + ?Sized + Encoding,
              F: 'a + ?Sized + Encodings,
              M: 'a + ?Sized + Encodings { }

impl<'a, T, I, F, M> Clone for Descriptor<'a, T, I, F, M>
        where T: 'a + ?Sized + Encoding,
              I: 'a + ?Sized + Encoding,
              F: 'a + ?Sized + Encodings,
              M: 'a + ?Sized + Encodings {
    fn clone(&self) -> Descriptor<'a, T, I, F, M> {
        *self
    }
}
