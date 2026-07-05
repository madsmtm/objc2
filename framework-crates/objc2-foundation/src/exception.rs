use core::fmt;
use core::hint::unreachable_unchecked;
use core::panic::{RefUnwindSafe, UnwindSafe};

use objc2::exception::Exception;
use objc2::rc::Retained;
use objc2::runtime::{AnyObject, NSObject, NSObjectProtocol};
use objc2::{msg_send, sel, ClassType};

use crate::{util, NSException};

// SAFETY: Exception objects are immutable data containers, and documented as
// thread safe.
//
// NOTE: This means that their user info dictionaries also must be
// thread-safe, which is not explicitly stated, but should generally be true.
unsafe impl Sync for NSException {}
unsafe impl Send for NSException {}

impl UnwindSafe for NSException {}
impl RefUnwindSafe for NSException {}

impl NSException {
    /// Create a new [`NSException`] object.
    #[cfg(all(
        feature = "NSDictionary",
        feature = "NSObjCRuntime",
        feature = "NSString"
    ))]
    pub fn new(name: &crate::NSExceptionName, reason: Option<&crate::NSString>) -> Retained<Self> {
        use objc2::AnyThread;
        // SAFETY: The dictionary is `NULL`, which is valid (and thread-safe).
        unsafe { Self::initWithName_reason_userInfo(Self::alloc(), name, reason, None) }
    }

    /// Raises the exception, causing program flow to jump to the local
    /// exception handler.
    ///
    /// This is equivalent to using `objc2::exception::throw`.
    pub fn raise(&self) -> ! {
        self.__raise();
        // SAFETY: `raise` will throw an exception, or abort if something
        // unexpected happened.
        unsafe { unreachable_unchecked() }
    }

    /// Convert this into an [`Exception`] object.
    pub fn into_exception(this: Retained<Self>) -> Retained<Exception> {
        // SAFETY: Downcasting to "subclass"
        unsafe { Retained::cast_unchecked(this) }
    }

    fn is_nsexception(obj: &Exception) -> bool {
        if obj.class().responds_to(sel!(isKindOfClass:)) {
            // SAFETY: We only use `isKindOfClass:` on NSObject
            let obj: *const Exception = obj;
            let obj = unsafe { obj.cast::<NSObject>().as_ref().unwrap() };
            obj.isKindOfClass(Self::class())
        } else {
            false
        }
    }

    /// Create this from an [`Exception`] object.
    ///
    /// This should be considered a hint; it may return `Err` in very, very
    /// few cases where the object is actually an instance of `NSException`.
    pub fn from_exception(obj: Retained<Exception>) -> Result<Retained<Self>, Retained<Exception>> {
        if Self::is_nsexception(&obj) {
            // SAFETY: Just checked the object is an NSException
            Ok(unsafe { Retained::cast_unchecked::<Self>(obj) })
        } else {
            Err(obj)
        }
    }
}

impl fmt::Debug for NSException {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let obj: &AnyObject = self.as_ref();
        write!(f, "{obj:?}")?;

        write!(f, " '")?;
        let name: Retained<NSObject> = unsafe { msg_send![self, name] };
        // SAFETY: `name` returns `NSExceptionName`, which is `NSString`.
        unsafe { util::display_string(&name, f)? };
        write!(f, "'")?;

        write!(f, " reason: ")?;
        let reason: Option<Retained<NSObject>> = unsafe { msg_send![self, reason] };
        if let Some(reason) = reason {
            // SAFETY: `reason` returns `NSString`.
            unsafe { util::display_string(&reason, f)? };
        } else {
            write!(f, "(NULL)")?;
        }
        Ok(())
    }
}
