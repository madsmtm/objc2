use core::{fmt, num::NonZeroIsize};

/// An error signalling that the operation timed out.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct DispatchTimeoutError(pub(crate) NonZeroIsize);

impl fmt::Display for DispatchTimeoutError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "the operation timed out with code {}", self.0)
    }
}

#[cfg(feature = "std")]
impl std::error::Error for DispatchTimeoutError {}
