#[cfg(feature = "Foundation_NSString")]
use core::fmt;
use core::panic::{RefUnwindSafe, UnwindSafe};

use crate::Foundation::NSError;

impl UnwindSafe for NSError {}
impl RefUnwindSafe for NSError {}

/// Creation methods.
impl NSError {
    /// Construct a new [`NSError`] with the given code in the given domain.
    #[cfg(feature = "Foundation_NSDictionary")]
    #[cfg(feature = "Foundation_NSString")]
    pub fn new(code: objc2::ffi::NSInteger, domain: &crate::NSErrorDomain) -> objc2::rc::Id<Self> {
        use objc2::ClassType;
        // SAFETY: `domain` and `user_info` are copied to the error object, so
        // even if the `&NSString` came from a `&mut NSMutableString`, we're
        // still good!
        unsafe { Self::initWithDomain_code_userInfo(Self::alloc(), domain, code, None) }
    }
}

/// Accessor methods.
impl NSError {
    #[cfg(feature = "Foundation_NSString")]
    pub fn NSLocalizedDescriptionKey() -> &'static crate::NSErrorUserInfoKey {
        unsafe { crate::NSLocalizedDescriptionKey }
    }
}

#[cfg(feature = "Foundation_NSString")]
#[cfg(feature = "Foundation_NSDictionary")]
impl std::error::Error for NSError {}

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
