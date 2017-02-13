use std::fmt;

use Encoding;

pub trait Encodings {
    fn eq<C: EncodingsComparator>(&self, C) -> bool;

    fn encoding_at_eq<T: ?Sized + Encoding>(&self, u8, &T) -> bool;

    fn len(&self) -> u8;

    fn write_all<W: fmt::Write>(&self, &mut W) -> fmt::Result;
}

impl<A, B> Encodings for (A, B) where A: Encoding, B: Encoding {
    fn eq<C: EncodingsComparator>(&self, mut fields: C) -> bool {
        fields.eq_next(&self.0) && fields.eq_next(&self.1) && fields.is_finished()
    }

    fn write_all<W: fmt::Write>(&self, formatter: &mut W) -> fmt::Result {
        write!(formatter, "{}{}", self.0, self.1)
    }

    fn encoding_at_eq<T: ?Sized + Encoding>(&self, index: u8, other: &T) -> bool {
        match index {
            0 => self.0.eq_encoding(other),
            1 => self.1.eq_encoding(other),
            _ => false,
        }
    }

    fn len(&self) -> u8 { 2 }
}

pub trait EncodingsComparator {
    fn eq_next<T: ?Sized + Encoding>(&mut self, &T) -> bool;
    fn is_finished(&self) -> bool;
}

pub struct EncodingTupleComparator<'a, T> where T: 'a + Encodings {
    encs: &'a T,
    index: u8,
}

impl<'a, T> EncodingTupleComparator<'a, T> where T: 'a + Encodings {
    pub fn new(encs: &'a T) -> EncodingTupleComparator<'a, T> {
        EncodingTupleComparator { encs: encs, index: 0 }
    }
}

impl<'a, T> EncodingsComparator for EncodingTupleComparator<'a, T>
        where T: 'a + Encodings {
    fn eq_next<E: ?Sized + Encoding>(&mut self, other: &E) -> bool {
        let index = self.index;
        if index < self.encs.len() {
            self.index += 1;
            self.encs.encoding_at_eq(index, other)
        } else {
            false
        }
    }

    fn is_finished(&self) -> bool {
        self.index >= self.encs.len()
    }
}
