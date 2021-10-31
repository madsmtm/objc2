use core::cmp::Ordering;

use objc2::{Encode, Encoding};

#[repr(isize)]
#[derive(Clone, Copy)]
pub enum NSComparisonResult {
    Ascending = -1,
    Same = 0,
    Descending = 1,
}

unsafe impl Encode for NSComparisonResult {
    const ENCODING: Encoding<'static> = isize::ENCODING;
}

impl NSComparisonResult {
    pub fn from_ordering(order: Ordering) -> NSComparisonResult {
        match order {
            Ordering::Less => NSComparisonResult::Ascending,
            Ordering::Equal => NSComparisonResult::Same,
            Ordering::Greater => NSComparisonResult::Descending,
        }
    }

    pub fn as_ordering(&self) -> Ordering {
        match *self {
            NSComparisonResult::Ascending => Ordering::Less,
            NSComparisonResult::Same => Ordering::Equal,
            NSComparisonResult::Descending => Ordering::Greater,
        }
    }
}
