#![no_std]

#[cfg(test)]
extern crate std;

mod encoding;
mod encode;
mod multi;
mod parse;

pub use encoding::{Descriptor, Encoding};
pub use encoding::{Primitive, Pointer, Array, Struct, Union};
pub use encode::Encode;
pub use multi::{Encodings, EncodingsComparator, IndexEncodings};
pub use parse::{StrEncoding, ParseEncodingError};
