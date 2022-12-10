use core::fmt;
use core::panic::{RefUnwindSafe, UnwindSafe};

use objc2::rc::{Id, Shared};
use objc2::ClassType;

use crate::Foundation::{
    NSCopying, NSError, NSErrorDomain, NSErrorUserInfoKey, NSInteger, NSLocalizedDescriptionKey,
};

// SAFETY: Error objects are immutable data containers.
unsafe impl Sync for NSError {}
unsafe impl Send for NSError {}

impl UnwindSafe for NSError {}
impl RefUnwindSafe for NSError {}

/// Creation methods.
impl NSError {
    /// Construct a new [`NSError`] with the given code in the given domain.
    pub fn new(code: NSInteger, domain: &NSErrorDomain) -> Id<Self, Shared> {
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

impl fmt::Debug for NSError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("NSError")
            .field("domain", &self.domain())
            .field("code", &self.code())
            .field("user_info", &self.userInfo())
            .finish()
    }
}

impl fmt::Display for NSError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.localizedDescription())
    }
}

impl std::error::Error for NSError {}

unsafe impl NSCopying for NSError {
    type Ownership = Shared;
    type Output = Self;
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::format;

    use crate::ns_string;

    #[test]
    fn custom_domain() {
        let error = NSError::new(42, ns_string!("MyDomain"));
        assert_eq!(error.code(), 42);
        assert_eq!(&*error.domain(), ns_string!("MyDomain"));
        let expected = if cfg!(feature = "apple") {
            "The operation couldn’t be completed. (MyDomain error 42.)"
        } else {
            "MyDomain 42"
        };
        assert_eq!(format!("{error}"), expected);
    }

    #[test]
    fn basic() {
        let error = NSError::new(-999, ns_string!("NSURLErrorDomain"));
        let expected = if cfg!(feature = "apple") {
            "The operation couldn’t be completed. (NSURLErrorDomain error -999.)"
        } else {
            "NSURLErrorDomain -999"
        };
        assert_eq!(format!("{error}"), expected);
    }
}
