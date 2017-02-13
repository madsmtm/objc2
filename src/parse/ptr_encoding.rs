use std::fmt;
use std::mem;

use encoding::{Descriptor, Encoding, PointerEncoding, Never};
use super::encoding::StrEncoding;

pub struct StrPointerEncoding(str);

impl StrPointerEncoding {
    pub fn from_str_unchecked(s: &str) -> &StrPointerEncoding {
        unsafe { mem::transmute(s) }
    }
}

impl Encoding for StrPointerEncoding {
    type Pointer = StrPointerEncoding;
    type Struct = Never;

    fn descriptor(&self) -> Descriptor<StrPointerEncoding, Never> {
        Descriptor::Pointer(self)
    }

    fn eq_encoding<T: ?Sized + Encoding>(&self, other: &T) -> bool {
        if let Descriptor::Pointer(p) = other.descriptor() {
            self.pointee().eq_encoding(p)
        } else {
            false
        }
    }
}

impl PointerEncoding for StrPointerEncoding {
    type Pointee = StrEncoding;

    fn pointee(&self) -> &StrEncoding {
        StrEncoding::from_str_unchecked(&self.0[1..])
    }
}

impl fmt::Display for StrPointerEncoding {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.0, formatter)
    }
}
