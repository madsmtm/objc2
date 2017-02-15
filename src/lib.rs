mod encoding;
mod multi;
mod parse;

pub use encoding::{Descriptor, Encoding};
pub use encoding::{Primitive, Pointer, Array, Struct};
pub use multi::{Encodings, EncodingsComparator};
pub use parse::StrEncoding;
