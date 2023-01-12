#![cfg(feature = "Foundation_NSMutableAttributedString")]
use objc2::rc::{DefaultId, Id, Owned, Shared};
use objc2::{extern_methods, msg_send_id, ClassType};

use crate::Foundation::{self, NSAttributedString, NSMutableAttributedString};

extern_methods!(
    /// Creating mutable attributed strings.
    unsafe impl NSMutableAttributedString {
        /// Construct an empty mutable attributed string.
        #[method_id(new)]
        pub fn new() -> Id<Self, Owned>;

        // TODO: new_with_attributes

        #[doc(alias = "initWithString:")]
        #[cfg(feature = "Foundation_NSString")]
        pub fn from_nsstring(string: &Foundation::NSString) -> Id<Self, Owned> {
            unsafe { msg_send_id![Self::alloc(), initWithString: string] }
        }

        #[doc(alias = "initWithAttributedString:")]
        pub fn from_attributed_nsstring(attributed_string: &NSAttributedString) -> Id<Self, Owned> {
            unsafe { msg_send_id![Self::alloc(), initWithAttributedString: attributed_string] }
        }
    }
);

impl DefaultId for NSMutableAttributedString {
    type Ownership = Owned;

    #[inline]
    fn default_id() -> Id<Self, Self::Ownership> {
        Self::new()
    }
}
