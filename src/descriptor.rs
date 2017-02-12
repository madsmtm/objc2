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

#[cfg(test)]
mod tests {
    use super::*;
    use encodings::*;
    use parse::StrEncoding;

    #[test]
    fn test_static_struct() {
        let f = (Primitive::Char, Primitive::Int);
        let s = Struct::new("CGPoint", f);
        assert_eq!(s.to_string(), "{CGPoint=ci}");

        let (name, mut fields) = match s.descriptor().0 {
            DescriptorKind::Struct(name, fields) => (name, fields),
            _ => panic!("Descriptor was not a struct"),
        };
        assert_eq!(name, "CGPoint");
        assert_eq!(fields.next().unwrap().to_string(), "c");
        assert_eq!(fields.next().unwrap().to_string(), "i");
        assert!(fields.next().is_none());
    }

    #[test]
    fn test_parsed_struct() {
        let s = StrEncoding::new_unchecked("{CGPoint=ci}");

        let (name, mut fields) = match s.descriptor().0 {
            DescriptorKind::Struct(name, fields) => (name, fields),
            _ => panic!("Descriptor was not a struct"),
        };
        assert_eq!(name, "CGPoint");
        assert_eq!(fields.next().unwrap().to_string(), "c");
        assert_eq!(fields.next().unwrap().to_string(), "i");
        assert!(fields.next().is_none());
    }

    #[test]
    fn test_descriptor_eq() {
        let i = Primitive::Int;
        let c = Primitive::Char;

        assert!(i.descriptor().eq(i.descriptor()));
        assert!(!i.descriptor().eq(c.descriptor()));

        let p = Pointer::new(i);
        assert!(p.descriptor().eq(p.descriptor()));
        assert!(!p.descriptor().eq(i.descriptor()));

        let s = Struct::new("CGPoint", (c, i));
        assert!(s.descriptor().eq(s.descriptor()));
        assert!(!s.descriptor().eq(i.descriptor()));

        let s2 = StrEncoding::new_unchecked("{CGPoint=ci}");
        assert!(s2.descriptor().eq(s2.descriptor()));
        assert!(s.descriptor().eq(s2.descriptor()));
    }
}
