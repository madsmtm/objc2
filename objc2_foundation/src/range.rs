use core::ops::Range;

use objc2::{Encode, Encoding};

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NSRange {
    pub location: usize,
    pub length: usize,
}

impl NSRange {
    pub fn from_range(range: Range<usize>) -> NSRange {
        assert!(range.end >= range.start);
        NSRange {
            location: range.start,
            length: range.end - range.start,
        }
    }

    pub fn as_range(&self) -> Range<usize> {
        Range {
            start: self.location,
            end: self.location + self.length,
        }
    }
}

unsafe impl Encode for NSRange {
    const ENCODING: Encoding<'static> =
        Encoding::Struct("_NSRange", &[usize::ENCODING, usize::ENCODING]);
}
