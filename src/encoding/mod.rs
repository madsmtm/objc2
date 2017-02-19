//! Types for statically defined encodings.

mod array;
mod never;
mod pointer;
mod primitive;
mod structure;
mod union;

pub use self::array::Array;
pub use self::pointer::Pointer;
pub use self::primitive::Primitive;
pub use self::structure::Struct;
pub use self::union::Union;
