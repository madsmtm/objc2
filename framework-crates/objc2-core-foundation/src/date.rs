#![cfg(feature = "CFBase")]
use core::{cmp::Ordering, ptr};

use crate::{CFDate, CFDateCompare};

impl PartialOrd for CFDate {
    #[inline]
    #[doc(alias = "CFDateCompare")]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for CFDate {
    #[inline]
    #[doc(alias = "CFDateCompare")]
    fn cmp(&self, other: &Self) -> Ordering {
        // Documented that one should pass NULL here.
        let context = ptr::null_mut();
        unsafe { CFDateCompare(self, Some(other), context) }.into()
    }
}

#[cfg(test)]
mod test {
    use crate::{CFAbsoluteTimeGetCurrent, CFDateCreate, CFDateGetAbsoluteTime};

    use super::*;

    #[test]
    fn cmp() {
        let now = unsafe { CFDateCreate(None, CFAbsoluteTimeGetCurrent()).unwrap() };
        let past = unsafe { CFDateCreate(None, CFDateGetAbsoluteTime(&now) - 1.0).unwrap() };
        assert_eq!(now.cmp(&past), Ordering::Greater);
        assert_eq!(now.cmp(&now), Ordering::Equal);
        assert_eq!(past.cmp(&now), Ordering::Less);

        assert_eq!(now, now);
        assert_ne!(now, past);
    }
}
