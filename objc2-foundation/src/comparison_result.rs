use core::cmp::Ordering;

use objc2::{Encode, Encoding, RefEncode};

use crate::ffi;

/// Constants that indicate sort order.
///
/// See [Apple's documentation](https://developer.apple.com/documentation/foundation/nscomparisonresult?language=objc).
#[repr(isize)] // NSInteger
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum NSComparisonResult {
    /// The left operand is smaller than the right operand.
    Ascending = ffi::NSComparisonResult_NSOrderedAscending,
    /// The two operands are equal.
    Same = ffi::NSComparisonResult_NSOrderedSame,
    /// The left operand is greater than the right operand.
    Descending = ffi::NSComparisonResult_NSOrderedDescending,
}

impl Default for NSComparisonResult {
    #[inline]
    fn default() -> Self {
        Self::Same
    }
}

unsafe impl Encode for NSComparisonResult {
    const ENCODING: Encoding<'static> = isize::ENCODING;
}

unsafe impl RefEncode for NSComparisonResult {
    const ENCODING_REF: Encoding<'static> = Encoding::Pointer(&Self::ENCODING);
}

impl From<Ordering> for NSComparisonResult {
    #[inline]
    fn from(order: Ordering) -> Self {
        match order {
            Ordering::Less => Self::Ascending,
            Ordering::Equal => Self::Same,
            Ordering::Greater => Self::Descending,
        }
    }
}

impl From<NSComparisonResult> for Ordering {
    #[inline]
    fn from(comparison_result: NSComparisonResult) -> Self {
        match comparison_result {
            NSComparisonResult::Ascending => Self::Less,
            NSComparisonResult::Same => Self::Equal,
            NSComparisonResult::Descending => Self::Greater,
        }
    }
}
