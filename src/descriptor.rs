use core::fmt;

use {Encoding, Encodings};
use encoding::Primitive;

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
    Primitive(Primitive),
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

pub fn encodings_eq<T, U>(e1: &T, e2: &U) -> bool
        where T: ?Sized + Encoding, U: ?Sized + Encoding {
    use Descriptor::*;
    match (e1.descriptor(), e2.descriptor()) {
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

pub fn write_encoding<W, T>(writer: &mut W, encoding: &T) -> fmt::Result
        where W: fmt::Write, T: ?Sized + Encoding {
    use Descriptor::*;
    match encoding.descriptor() {
        Primitive(p) => write_primitive(writer, p),
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

fn write_primitive<W: fmt::Write>(writer: &mut W, p: Primitive) -> fmt::Result {
    use encoding::Primitive::*;
    let code = match p {
        Char      => "c",
        Short     => "s",
        Int       => "i",
        Long      => "l",
        LongLong  => "q",
        UChar     => "C",
        UShort    => "S",
        UInt      => "I",
        ULong     => "L",
        ULongLong => "Q",
        Float     => "f",
        Double    => "d",
        Bool      => "B",
        Void      => "v",
        String    => "*",
        Object    => "@",
        Block     => "@?",
        Class     => "#",
        Sel       => ":",
        Unknown   => "?",
        BitField(b) => {
            return write!(writer, "b{}", b);
        }
    };
    writer.write_str(code)
}
