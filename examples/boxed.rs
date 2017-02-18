extern crate objc_encode;

use std::fmt;

use objc_encode::{Encoding, Encodings, EncodingsIterateCallback, Descriptor};
use objc_encode::encoding::Primitive;
use objc_encode::parse::{StrEncoding, ParseEncodingError};

#[derive(Debug)]
enum BoxedEncoding {
    Primitive(Primitive),
    Pointer(Box<BoxedEncoding>),
    Array(u32, Box<BoxedEncoding>),
    Struct(String, Vec<BoxedEncoding>),
    Union(String, Vec<BoxedEncoding>),
}

impl BoxedEncoding {
    fn from_encoding<T: ?Sized + Encoding>(encoding: &T) -> BoxedEncoding {
        use BoxedEncoding::*;
        match encoding.descriptor() {
            Descriptor::Primitive(p) => Primitive(p),
            Descriptor::Pointer(t) =>
                Pointer(Box::new(BoxedEncoding::from_encoding(t))),
            Descriptor::Array(len, item) =>
                Array(len, Box::new(BoxedEncoding::from_encoding(item))),
            Descriptor::Struct(name, fields) => {
                let mut collector = BoxedEncodingsCollector::new();
                fields.each(&mut collector);
                Struct(name.to_string(), collector.into_encodings())
            }
            Descriptor::Union(name, members) => {
                let mut collector = BoxedEncodingsCollector::new();
                members.each(&mut collector);
                Union(name.to_string(), collector.into_encodings())
            }
        }
    }

    fn from_str(s: &str) -> Result<BoxedEncoding, ParseEncodingError<&str>> {
        StrEncoding::from_str(s).map(|e| BoxedEncoding::from_encoding(e))
    }
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

struct BoxedEncodingsCollector {
    encs: Vec<BoxedEncoding>,
}

impl BoxedEncodingsCollector {
    fn new() -> BoxedEncodingsCollector {
        BoxedEncodingsCollector { encs: Vec::new() }
    }

    fn into_encodings(self) -> Vec<BoxedEncoding> {
        self.encs
    }
}

impl EncodingsIterateCallback for BoxedEncodingsCollector {
    fn call<T: ?Sized + Encoding>(&mut self, encoding: &T) -> bool {
        self.encs.push(BoxedEncoding::from_encoding(encoding));
        false
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

    let enc = BoxedEncoding::from_str("{CGPoint=ci}");
    println!("{:?}", enc);
}
