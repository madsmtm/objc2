#![cfg(feature = "Foundation_NSMutableAttributedString")]
use objc2::rc::DefaultId;

use crate::common::*;
use crate::Foundation::{self, NSAttributedString, NSMutableAttributedString};

/// Creating mutable attributed strings.
impl NSMutableAttributedString {
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

impl DefaultId for NSMutableAttributedString {
    #[inline]
    fn default_id() -> Id<Self> {
        Self::new()
    }
}
