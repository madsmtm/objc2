#![cfg(feature = "Foundation_NSAttributedString")]
use core::panic::{RefUnwindSafe, UnwindSafe};

use objc2::rc::{DefaultId, Id, Shared};
use objc2::runtime::Object;
use objc2::{extern_methods, ClassType};

use crate::Foundation::{self, NSAttributedString, NSAttributedStringKey};

// SAFETY: `NSAttributedString` is immutable and `NSMutableAttributedString`
// can only be mutated from `&mut` methods.
unsafe impl Sync for NSAttributedString {}
unsafe impl Send for NSAttributedString {}

// Same reasoning as `NSString`.
impl UnwindSafe for NSAttributedString {}
impl RefUnwindSafe for NSAttributedString {}

extern_methods!(
    /// Creating attributed strings.
    unsafe impl NSAttributedString {
        /// Construct an empty attributed string.
        #[method_id(new)]
        pub fn new() -> Id<Self, Shared>;

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
            attributes: &Foundation::NSDictionary<NSAttributedStringKey, Object>,
        ) -> Id<Self, Shared> {
            unsafe { Self::initWithString_attributes(Self::alloc(), string, Some(attributes)) }
        }

        /// Creates a new attributed string without any attributes.
        #[doc(alias = "initWithString:")]
        #[cfg(feature = "Foundation_NSString")]
        pub fn from_nsstring(string: &Foundation::NSString) -> Id<Self, Shared> {
            Self::initWithString(Self::alloc(), string)
        }
    }
);

impl DefaultId for NSAttributedString {
    type Ownership = Shared;

    #[inline]
    fn default_id() -> Id<Self, Self::Ownership> {
        Self::new()
    }
}
