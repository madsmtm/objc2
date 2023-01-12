#![cfg(feature = "Foundation_NSMutableString")]
use core::cmp;
use core::fmt;
use core::ops::AddAssign;
use core::str;

use objc2::rc::DefaultId;

use crate::common::*;
use crate::Foundation::{NSMutableString, NSString};

extern_methods!(
    /// Creating mutable strings.
    unsafe impl NSMutableString {
        /// Construct an empty [`NSMutableString`].
        #[method_id(new)]
        pub fn new() -> Id<Self, Owned>;

        /// Creates a new [`NSMutableString`] by copying the given string slice.
        #[doc(alias = "initWithBytes:length:encoding:")]
        #[allow(clippy::should_implement_trait)] // Not really sure of a better name
        pub fn from_str(string: &str) -> Id<Self, Owned> {
            unsafe {
                let obj = super::string::from_str(Self::class(), string);
                Id::new(obj.cast()).unwrap()
            }
        }
    }
);

impl DefaultId for NSMutableString {
    type Ownership = Owned;

    #[inline]
    fn default_id() -> Id<Self, Self::Ownership> {
        Self::new()
    }
}

impl AddAssign<&NSString> for NSMutableString {
    #[inline]
    fn add_assign(&mut self, other: &NSString) {
        self.appendString(other)
    }
}

impl PartialEq<NSString> for NSMutableString {
    #[inline]
    fn eq(&self, other: &NSString) -> bool {
        PartialEq::eq(&**self, other)
    }
}

impl PartialEq<NSMutableString> for NSString {
    #[inline]
    fn eq(&self, other: &NSMutableString) -> bool {
        PartialEq::eq(self, &**other)
    }
}

impl PartialOrd for NSMutableString {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        PartialOrd::partial_cmp(&**self, &**other)
    }
}

impl PartialOrd<NSString> for NSMutableString {
    #[inline]
    fn partial_cmp(&self, other: &NSString) -> Option<cmp::Ordering> {
        PartialOrd::partial_cmp(&**self, other)
    }
}

impl PartialOrd<NSMutableString> for NSString {
    #[inline]
    fn partial_cmp(&self, other: &NSMutableString) -> Option<cmp::Ordering> {
        PartialOrd::partial_cmp(self, &**other)
    }
}

impl Ord for NSMutableString {
    #[inline]
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        Ord::cmp(&**self, &**other)
    }
}

impl fmt::Write for NSMutableString {
    fn write_str(&mut self, s: &str) -> Result<(), fmt::Error> {
        let nsstring = NSString::from_str(s);
        self.appendString(&nsstring);
        Ok(())
    }
}

impl fmt::Display for NSMutableString {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&**self, f)
    }
}
