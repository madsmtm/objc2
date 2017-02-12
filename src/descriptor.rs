use {Encoding, PointerEncoding, StructEncoding};
use encodings::Primitive;

pub enum Descriptor<'a, P, S>
        where P: 'a + ?Sized + PointerEncoding,
              S: 'a + ?Sized + StructEncoding {
    Primitive(Primitive),
    Pointer(&'a P),
    Struct(&'a S),
}

impl<'a, P, S> Descriptor<'a, P, S>
        where P: 'a + ?Sized + PointerEncoding,
              S: 'a + ?Sized + StructEncoding {
    pub fn eq_encoding<T: ?Sized + Encoding>(&self, other: &T) -> bool {
        match *self {
            Descriptor::Primitive(p) => p.eq_encoding(other),
            Descriptor::Pointer(p) => p.eq_encoding(other),
            Descriptor::Struct(s) => s.eq_encoding(other),
        }
    }
}
