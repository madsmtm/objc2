#![cfg(feature = "Foundation_NSError")]
use core::fmt;
use core::panic::{RefUnwindSafe, UnwindSafe};

use objc2::rc::Id;
use objc2::ClassType;

use crate::Foundation::{
    self, NSError, NSErrorDomain, NSErrorUserInfoKey, NSInteger, NSLocalizedDescriptionKey,
};

// SAFETY: Error objects are immutable data containers.
unsafe impl Sync for NSError {}
unsafe impl Send for NSError {}

impl UnwindSafe for NSError {}
impl RefUnwindSafe for NSError {}

/// Creation methods.
impl NSError {
    /// Construct a new [`NSError`] with the given code in the given domain.
    pub fn new(code: NSInteger, domain: &NSErrorDomain) -> Id<Self> {
        // SAFETY: `domain` and `user_info` are copied to the error object, so
        // even if the `&NSString` came from a `&mut NSMutableString`, we're
        // still good!
        unsafe { Self::initWithDomain_code_userInfo(Self::alloc(), domain, code, None) }
    }
}

/// Accessor methods.
impl NSError {
    #[allow(non_snake_case)]
    pub fn NSLocalizedDescriptionKey() -> &'static NSErrorUserInfoKey {
        unsafe { NSLocalizedDescriptionKey }
    }
}

#[cfg(feature = "Foundation_NSString")]
#[cfg(feature = "Foundation_NSDictionary")]
impl std::error::Error for Foundation::NSError {}

#[cfg(feature = "Foundation_NSString")]
#[cfg(feature = "Foundation_NSDictionary")]
impl fmt::Debug for NSError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("NSError")
            .field("domain", &self.domain())
            .field("code", &self.code())
            .field("user_info", &self.userInfo())
            .finish()
    }
}

#[cfg(feature = "Foundation_NSString")]
impl fmt::Display for NSError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.localizedDescription())
    }
}
