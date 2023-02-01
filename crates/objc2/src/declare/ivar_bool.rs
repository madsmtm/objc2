use crate::encode::{Encode, Encoding};

use super::InnerIvarType;

/// Ivar of [`bool`].
///
/// This is used to work around the fact that `bool` is not [`Encode`].
///
/// If you want to access this instance variable to Objective-C, you must do
/// so using C99 `_Bool`; if you want to use `BOOL` in Objective-C, you should
/// use `IvarEncode<Bool>`.
#[repr(transparent)]
pub struct IvarBool(bool);

unsafe impl Encode for IvarBool {
    const ENCODING: Encoding = Encoding::Bool;
}

impl super::ivar::private::Sealed for IvarBool {}

// SAFETY: IvarBool is `#[repr(transparent)]`, and `bool` is safe to
// zero-initialize
unsafe impl InnerIvarType for IvarBool {
    type Output = bool;

    #[inline]
    unsafe fn __deref(&self) -> &Self::Output {
        &self.0
    }

    #[inline]
    unsafe fn __deref_mut(&mut self) -> &mut Self::Output {
        &mut self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use core::mem;

    #[test]
    fn needs_drop() {
        assert!(!mem::needs_drop::<IvarBool>());
        assert_eq!(mem::size_of::<IvarBool>(), mem::size_of::<bool>());
    }
}
