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
assert!(i32::encode().eq_encoding(parsed));
```

# Generating encoding strings

The string representation of an `Encoding` can be generated via its `write`
method:

```
# use objc_encode::{Encode, Encoding};
let mut result = String::new();
i32::encode().write(&mut result).unwrap();
assert_eq!(result, "i");
```

The encodings defined in this crate also implement `Display` for convenience,
allowing the `to_string` method to be used:

```
# use objc_encode::Encode;
assert_eq!(i32::encode().to_string(), "i");
```
*/

#![no_std]

#[cfg(test)]
extern crate std;

pub mod encoding;
mod encode;
mod multi;
pub mod parse;

use core::fmt;

pub use encode::Encode;
pub use multi::{Encodings, EncodingsIterateCallback};

/// An Objective-C type encoding.
pub trait Encoding {
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

    /// Writes the string representation of self to the given writer.
    fn write<W: fmt::Write>(&self, writer: &mut W) -> fmt::Result {
        use Descriptor::*;
        match self.descriptor() {
            Primitive(p) => write!(writer, "{}", p),
            Pointer(t) => {
                writer.write_char('^')?;
                t.write(writer)
            }
            Array(len, item) => {
                write!(writer, "[{}", len)?;
                item.write(writer)?;
                writer.write_char(']')
            }
            Struct(name, fields) => {
                write!(writer, "{{{}=", name)?;
                fields.write_all(writer)?;
                writer.write_char('}')
            }
            Union(name, members) => {
                write!(writer, "({}=", name)?;
                members.write_all(writer)?;
                writer.write_char(')')
            }
        }
    }
}

/**
A type which describes an `Encoding`.

In a sense, descriptors allow a form of downcasting for `Encoding`s.
While accepting an `Encoding` of any type, through its descriptor we can
still know what kind it is and access specific information about it, like
the length of an array or the name of a struct. This allows encodings of
different types to be compared and interoperate.
*/
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
