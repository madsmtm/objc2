use std::fmt;

use Encoding;

pub trait EncodingTuple {
    fn eq<F: FieldsComparator>(&self, F) -> bool;

    fn write_all<W: fmt::Write>(&self, &mut W) -> fmt::Result;

    fn encoding_at_eq<T: ?Sized + Encoding>(&self, u8, &T) -> bool;

    fn len(&self) -> u8;
}

impl<A, B> EncodingTuple for (A, B) where A: Encoding, B: Encoding {
    fn eq<F: FieldsComparator>(&self, mut fields: F) -> bool {
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

pub trait FieldsComparator {
    fn eq_next<T: ?Sized + Encoding>(&mut self, &T) -> bool;
    fn is_finished(&self) -> bool;
}

pub struct EncodingTupleComparator<'a, T> where T: 'a + EncodingTuple {
    encs: &'a T,
    index: u8,
}

impl<'a, T> EncodingTupleComparator<'a, T> where T: 'a + EncodingTuple {
    pub fn new(encs: &'a T) -> EncodingTupleComparator<'a, T> {
        EncodingTupleComparator { encs: encs, index: 0 }
    }
}

impl<'a, T> FieldsComparator for EncodingTupleComparator<'a, T>
        where T: 'a + EncodingTuple {
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
