#![cfg(feature = "Foundation_NSAttributedString")]
use core::panic::{RefUnwindSafe, UnwindSafe};

use objc2::extern_methods;

use crate::common::*;
use crate::Foundation::{self, NSAttributedString, NSAttributedStringKey};

// SAFETY: `NSAttributedString` is immutable and `NSMutableAttributedString`
// can only be mutated from `&mut` methods.
unsafe impl Sync for NSAttributedString {}
unsafe impl Send for NSAttributedString {}

// Same reasoning as `NSString`.
impl UnwindSafe for NSAttributedString {}
impl RefUnwindSafe for NSAttributedString {}

impl NSAttributedString {
    /// Creates a new attributed string from the given string and attributes.
    ///
    /// The attributes are associated with every UTF-16 code unit in the
    /// string.
    ///
    /// # Safety
    ///
    /// The attributes must be valid.
    #[doc(alias = "initWithString:")]
    #[cfg(feature = "Foundation_NSDictionary")]
    #[cfg(feature = "Foundation_NSString")]
    pub unsafe fn new_with_attributes(
        string: &Foundation::NSString,
        attributes: &Foundation::NSDictionary<NSAttributedStringKey, AnyObject>,
    ) -> Id<Self> {
        unsafe { Self::initWithString_attributes(Self::alloc(), string, Some(attributes)) }
    }

    /// Creates a new attributed string without any attributes.
    #[doc(alias = "initWithString:")]
    #[cfg(feature = "Foundation_NSString")]
    pub fn from_nsstring(string: &Foundation::NSString) -> Id<Self> {
        Self::initWithString(Self::alloc(), string)
    }
}

#[cfg(feature = "Foundation_NSMutableAttributedString")]
impl Foundation::NSMutableAttributedString {
    // TODO: new_with_attributes

    #[doc(alias = "initWithString:")]
    #[cfg(feature = "Foundation_NSString")]
    pub fn from_nsstring(string: &Foundation::NSString) -> Id<Self> {
        Self::initWithString(Self::alloc(), string)
    }

    #[doc(alias = "initWithAttributedString:")]
    pub fn from_attributed_nsstring(attributed_string: &NSAttributedString) -> Id<Self> {
        Self::initWithAttributedString(Self::alloc(), attributed_string)
    }
}
