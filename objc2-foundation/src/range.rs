use core::ops::Range;

use objc2::encode::{Encode, Encoding, RefEncode};

#[repr(C)]
// PartialEq is same as NSEqualRanges
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct NSRange {
    pub location: usize,
    pub length: usize,
}

// impl NSRange {
//     pub fn contains(&self, index: usize) -> bool {
//         // Same as NSLocationInRange
//         <Self as RangeBounds<usize>>::contains(self, &index)
//     }
// }

// impl RangeBounds<usize> for NSRange {
//     fn start_bound(&self) -> Bound<&usize> {
//         Bound::Included(&self.location)
//     }
//     fn end_bound(&self) -> Bound<&usize> {
//         Bound::Excluded(&(self.location + self.length))
//     }
// }

impl From<Range<usize>> for NSRange {
    fn from(range: Range<usize>) -> Self {
        let length = range
            .end
            .checked_sub(range.start)
            .expect("Range end < start");
        Self {
            location: range.start,
            length,
        }
    }
}

impl From<NSRange> for Range<usize> {
    fn from(nsrange: NSRange) -> Self {
        Self {
            start: nsrange.location,
            end: nsrange.location + nsrange.length,
        }
    }
}

unsafe impl Encode for NSRange {
    const ENCODING: Encoding<'static> =
        Encoding::Struct("_NSRange", &[usize::ENCODING, usize::ENCODING]);
}

unsafe impl RefEncode for NSRange {
    const ENCODING_REF: Encoding<'static> = Encoding::Pointer(&Self::ENCODING);
}
