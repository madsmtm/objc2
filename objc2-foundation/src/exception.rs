use core::hint::unreachable_unchecked;
use core::panic::{RefUnwindSafe, UnwindSafe};

use objc2::exception::Exception;
use objc2::rc::{Id, Shared};
use objc2::runtime::Object;
use objc2::{msg_send, msg_send_id, sel};

use crate::{NSCopying, NSDictionary, NSObject, NSString};

object! {
    /// A special condition that interrupts the normal flow of program
    /// execution.
    ///
    /// Exceptions can be thrown and caught using the `objc2::exception`
    /// module.
    ///
    /// See also [Apple's documentation][doc].
    ///
    /// [doc]: https://developer.apple.com/documentation/foundation/nsexception?language=objc
    unsafe pub struct NSException: NSObject;
}

// SAFETY: Exception objects are immutable data containers, and documented as
// thread safe.
unsafe impl Sync for NSException {}
unsafe impl Send for NSException {}

impl UnwindSafe for NSException {}
impl RefUnwindSafe for NSException {}

type NSExceptionName = NSString;

impl NSException {
    /// Create a new [`NSException`] object.
    ///
    /// Returns `None` if the exception couldn't be created (example: If the
    /// process is out of memory).
    pub fn new(
        name: &NSExceptionName,
        reason: Option<&NSString>,
        user_info: Option<&NSDictionary<Object, Object>>,
    ) -> Option<Id<Self, Shared>> {
        let obj = unsafe { msg_send_id![Self::class(), alloc] };
        unsafe { msg_send_id![obj, initWithName: name, reason: reason, userInfo: user_info] }
    }

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
        let _: () = unsafe { msg_send![self, raise] };
        unsafe { unreachable_unchecked() }
    }

    /// A that uniquely identifies the type of exception.
    ///
    /// See [Apple's documentation][doc] for some of the different values this
    /// can take.
    ///
    /// [doc]: https://developer.apple.com/documentation/foundation/nsexceptionname?language=objc
    pub fn name(&self) -> Id<NSExceptionName, Shared> {
        // Nullability not documented, but a name is expected in most places.
        unsafe { msg_send_id![self, name].unwrap() }
    }

    /// A human-readable message summarizing the reason for the exception.
    pub fn reason(&self) -> Option<Id<NSString, Shared>> {
        unsafe { msg_send_id![self, reason] }
    }

    /// Application-specific data pertaining to the exception.
    pub fn user_info(&self) -> Option<Id<NSDictionary<Object, Object>, Shared>> {
        unsafe { msg_send_id![self, userInfo] }
    }

    /// Convert this into an [`Exception`] object.
    pub fn into_exception(this: Id<Self, Shared>) -> Id<Exception, Shared> {
        // SAFETY: Downcasting to "subclass"
        unsafe { Id::cast(this) }
    }

    /// Convert this into an [`Exception`] object.
    pub fn from_exception(
        obj: Id<Exception, Shared>,
    ) -> Result<Id<Self, Shared>, Id<Exception, Shared>> {
        if obj.class().responds_to(sel!(isKindOfClass:)) {
            // SAFETY: We only use `isKindOfClass:` on NSObject
            let obj = unsafe { Id::cast::<NSObject>(obj) };
            if obj.is_kind_of(Self::class()) {
                // SAFETY: Just checked the object is an NSException
                Ok(unsafe { Id::cast::<Self>(obj) })
            } else {
                // SAFETY: Cast back
                Err(unsafe { Id::cast(obj) })
            }
        } else {
            Err(obj)
        }
    }
}

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

// TODO: Better Debug impl

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
        assert!(exc.user_info().is_none());

        let description = if cfg!(feature = "gnustep-1-7") {
            format!("<NSException: {:p}> NAME:abc REASON:def", exc)
        } else {
            "def".into()
        };

        assert_eq!(exc.description(), NSString::from_str(&description));
        assert_eq!(format!("{:?}", exc), format!("\"{}\"", description));
    }

    #[test]
    #[cfg_attr(
        feature = "apple",
        should_panic = "called `Result::unwrap()` on an `Err` value: \"def\""
    )]
    #[cfg_attr(feature = "gnustep-1-7", should_panic = "> NAME:abc REASON:def")]
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
