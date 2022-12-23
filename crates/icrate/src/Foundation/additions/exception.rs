use core::fmt;
use core::hint::unreachable_unchecked;
use core::panic::{RefUnwindSafe, UnwindSafe};

use objc2::exception::Exception;
use objc2::rc::{Id, Shared};
use objc2::runtime::Object;
use objc2::{extern_methods, msg_send_id, sel, ClassType};

use crate::Foundation::{
    NSCopying, NSDictionary, NSException, NSExceptionName, NSObject, NSString,
};

// SAFETY: Exception objects are immutable data containers, and documented as
// thread safe.
unsafe impl Sync for NSException {}
unsafe impl Send for NSException {}

impl UnwindSafe for NSException {}
impl RefUnwindSafe for NSException {}

extern_methods!(
    unsafe impl NSException {
        /// Create a new [`NSException`] object.
        ///
        /// Returns `None` if the exception couldn't be created (example: If the
        /// process is out of memory).
        pub fn new(
            name: &NSExceptionName,
            reason: Option<&NSString>,
            user_info: Option<&NSDictionary<Object, Object>>,
        ) -> Option<Id<Self, Shared>> {
            unsafe {
                msg_send_id![
                    Self::alloc(),
                    initWithName: name,
                    reason: reason,
                    userInfo: user_info,
                ]
            }
        }

        #[method(raise)]
        unsafe fn raise_raw(&self);

        /// Raises the exception, causing program flow to jump to the local
        /// exception handler.
        ///
        /// This is equivalent to using `objc2::exception::throw`.
        ///
        ///
        /// # Safety
        ///
        /// Same as `objc2::exception::throw`.
        pub unsafe fn raise(&self) -> ! {
            // SAFETY: We only create `Shared` NSExceptions, so it is safe to give
            // to the place where `@catch` receives it.
            unsafe { self.raise_raw() };
            // SAFETY: `raise` will throw an exception, or abort if something
            // unexpected happened.
            unsafe { unreachable_unchecked() }
        }

        /// Convert this into an [`Exception`] object.
        pub fn into_exception(this: Id<Self, Shared>) -> Id<Exception, Shared> {
            // SAFETY: Downcasting to "subclass"
            unsafe { Id::cast(this) }
        }

        fn is_nsexception(obj: &Exception) -> bool {
            if obj.class().responds_to(sel!(isKindOfClass:)) {
                // SAFETY: We only use `isKindOfClass:` on NSObject
                let obj: *const Exception = obj;
                let obj = unsafe { obj.cast::<NSObject>().as_ref().unwrap() };
                obj.is_kind_of::<Self>()
            } else {
                false
            }
        }

        /// Create this from an [`Exception`] object.
        ///
        /// This should be considered a hint; it may return `Err` in very, very
        /// few cases where the object is actually an instance of `NSException`.
        pub fn from_exception(
            obj: Id<Exception, Shared>,
        ) -> Result<Id<Self, Shared>, Id<Exception, Shared>> {
            if Self::is_nsexception(&obj) {
                // SAFETY: Just checked the object is an NSException
                Ok(unsafe { Id::cast::<Self>(obj) })
            } else {
                Err(obj)
            }
        }
    }
);

unsafe impl NSCopying for NSException {
    type Ownership = Shared;
    type Output = NSException;
}

impl alloc::borrow::ToOwned for NSException {
    type Owned = Id<NSException, Shared>;
    fn to_owned(&self) -> Self::Owned {
        self.copy()
    }
}

impl fmt::Debug for NSException {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let obj: &Object = self.as_ref();
        write!(f, "{obj:?} '{}'", self.name())?;
        if let Some(reason) = self.reason() {
            write!(f, " reason:{reason}")?;
        } else {
            write!(f, " reason:(NULL)")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use alloc::format;

    use super::*;

    #[test]
    fn create_and_query() {
        let exc = NSException::new(
            &NSString::from_str("abc"),
            Some(&NSString::from_str("def")),
            None,
        )
        .unwrap();

        assert_eq!(exc.name(), NSString::from_str("abc"));
        assert_eq!(exc.reason().unwrap(), NSString::from_str("def"));
        assert!(exc.userInfo().is_none());

        let debug = format!("<NSException: {exc:p}> 'abc' reason:def");
        assert_eq!(format!("{exc:?}"), debug);

        let description = if cfg!(feature = "gnustep-1-7") {
            format!("<NSException: {exc:p}> NAME:abc REASON:def")
        } else {
            "def".into()
        };

        let obj: &NSObject = &exc;
        assert_eq!(format!("{obj:?}"), description);

        let exc = NSException::into_exception(exc);

        // Test `Debug` impl of Exception
        assert_eq!(format!("{exc:?}"), format!("exception {debug}"));
        // Test `Display` impl of Exception
        assert_eq!(format!("{exc}"), "def");
    }

    #[test]
    #[should_panic = "'abc' reason:def"]
    fn unwrap() {
        let exc = NSException::new(
            &NSString::from_str("abc"),
            Some(&NSString::from_str("def")),
            None,
        )
        .unwrap();

        let _: () = Err(exc).unwrap();
    }

    // Further tests in `tests::exception`
}
