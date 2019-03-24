/*!
Objective-C type encoding creation and parsing in Rust.

The Objective-C compiler encodes types as strings for usage in the runtime.
This crate aims to provide a strongly-typed (rather than stringly-typed) way
to create and describe these type encodings without memory allocation in Rust.

# Implementing Encode

This crate declares an `Encode` trait that can be implemented for types that
the Objective-C compiler can encode. Implementing this trait looks like:

``` ignore
unsafe impl Encode for CGPoint {
    type Encoding = Struct<&'static str, (Primitive, Primitive)>;

    fn encode() -> Self::Encoding {
        Struct::new("CGPoint", (CGFloat::encode(), CGFloat::encode()))
    }
}
```

For an example of how this works with more complex types, like structs
containing structs, see the `core_graphics` example.

# Comparing with encoding strings

If you have an encoding string from the Objective-C runtime, it can be parsed
and compared with another encoding through a `StrEncoding`:

```
# use objc_encode::{Encode, Encoding};
# use objc_encode::parse::StrEncoding;
let parsed = StrEncoding::from_str("i").unwrap();
assert!(parsed == &i32::encode());
```

# Generating encoding strings

Every `Encoding` implements `Display` as its string representation.
This can be generated conveniently through the `to_string` method:

```
# use objc_encode::Encode;
assert_eq!(i32::encode().to_string(), "i");
```
*/

#![no_std]

extern crate libc;
#[cfg(test)]
extern crate std;

pub mod encoding;
mod encode;
mod descriptor;
mod multi;
pub mod parse;

use core::fmt;

pub use crate::encode::Encode;
pub use crate::descriptor::Descriptor;
pub use crate::multi::{Encodings, EncodingsIterateCallback};

/// An Objective-C type encoding.
pub trait Encoding: fmt::Display {
    /// The type of `Encoding` that Self will use if it is an encoding for
    /// a pointer to describe its target.
    type PointerTarget: ?Sized + Encoding;
    /// The type of `Encoding` that Self will use if it is an encoding for
    /// an array to describe its items.
    type ArrayItem: ?Sized + Encoding;
    /// The type of `Encodings` that Self will use if it is an encoding for
    /// a struct to describe its fields.
    type StructFields: ?Sized + Encodings;
    /// The type of `Encodings` that Self will use if it is an encoding for
    /// a union to describe its members.
    type UnionMembers: ?Sized + Encodings;

    /// Returns a `Descriptor` that describes what kind of encoding self is.
    fn descriptor(&self) -> Descriptor<Self::PointerTarget,
                                       Self::ArrayItem,
                                       Self::StructFields,
                                       Self::UnionMembers>;

    /// Returns whether self is equal to the given `Encoding`.
    fn eq_encoding<T: ?Sized + Encoding>(&self, other: &T) -> bool {
        descriptor::encodings_eq(self, other)
    }

    /// Writes the string representation of self to the given writer.
    fn write<W: fmt::Write>(&self, writer: &mut W) -> fmt::Result {
        descriptor::write_encoding(writer, self)
    }
}
