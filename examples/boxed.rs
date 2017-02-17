extern crate objc_encode;

use std::fmt;

use objc_encode::{Encoding, Encodings, Descriptor};
use objc_encode::encoding::Primitive;

enum BoxedEncoding {
    Primitive(Primitive),
    Pointer(Box<BoxedEncoding>),
    Array(u32, Box<BoxedEncoding>),
    Struct(String, Vec<BoxedEncoding>),
    Union(String, Vec<BoxedEncoding>),
}

impl Encoding for BoxedEncoding {
    type PointerTarget = BoxedEncoding;
    type ArrayItem = BoxedEncoding;
    type StructFields = [BoxedEncoding];
    type UnionMembers = [BoxedEncoding];

    fn descriptor(&self) -> Descriptor<BoxedEncoding, BoxedEncoding, [BoxedEncoding], [BoxedEncoding]> {
        use BoxedEncoding::*;
        match *self {
            Primitive(p) => Descriptor::Primitive(p),
            Pointer(ref t) => Descriptor::Pointer(t),
            Array(len, ref item) => Descriptor::Array(len, item),
            Struct(ref name, ref fields) => Descriptor::Struct(name, fields),
            Union(ref name, ref members) => Descriptor::Union(name, members),
        }
    }
}

impl fmt::Display for BoxedEncoding {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        use BoxedEncoding::*;
        match *self {
            Primitive(p) => write!(formatter, "{}", p),
            Pointer(ref t) => write!(formatter, "^{}", t),
            Array(len, ref item) => write!(formatter, "[{}^{}]", len, item),
            Struct(ref name, ref fields) => {
                write!(formatter, "{{{}=", name)?;
                fields.write_all(formatter)?;
                write!(formatter, "}}")
            },
            Union(ref name, ref members) => {
                write!(formatter, "({}=", name)?;
                members.write_all(formatter)?;
                write!(formatter, ")")
            },
        }
    }
}

fn main() {
    let enc = BoxedEncoding::Struct("CGPoint".to_string(), vec![
        BoxedEncoding::Primitive(Primitive::Char),
        BoxedEncoding::Primitive(Primitive::Int),
    ]);
    println!("{}", enc);

    let enc = BoxedEncoding::Pointer(Box::new(enc));
    println!("{}", enc);

    let enc = BoxedEncoding::Array(12, Box::new(enc));
    println!("{}", enc);

    let enc = BoxedEncoding::Union("CGSize".to_string(), vec![
        BoxedEncoding::Primitive(Primitive::Int),
        enc,
    ]);
    println!("{}", enc);
}
