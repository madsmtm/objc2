use crate::runtime::{Class, Object, Sel};
use crate::{Encode, Encoding, RefEncode};

unsafe impl Encode for Sel {
    const ENCODING: Encoding<'static> = Encoding::Sel;
}

unsafe impl RefEncode for Object {
    const ENCODING_REF: Encoding<'static> = Encoding::Object;
}

unsafe impl RefEncode for Class {
    const ENCODING_REF: Encoding<'static> = Encoding::Class;
}

#[cfg(test)]
mod tests {
    use crate::runtime::{Class, Object, Sel};
    use crate::Encode;
    use alloc::string::ToString;

    #[test]
    fn test_encode() {
        assert!(<&Object>::ENCODING.to_string() == "@");
        assert!(<*mut Object>::ENCODING.to_string() == "@");
        assert!(<&Class>::ENCODING.to_string() == "#");
        assert!(Sel::ENCODING.to_string() == ":");
    }
}
