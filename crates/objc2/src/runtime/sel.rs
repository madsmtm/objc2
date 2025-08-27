use core::ffi::c_char;
use core::ffi::{c_void, CStr};
use core::fmt;
use core::hash;
use core::panic::{RefUnwindSafe, UnwindSafe};
use core::ptr::{self, NonNull};

use crate::encode::{Encode, Encoding, OptionEncode};
use crate::ffi;

/// A method selector.
///
/// The Rust equivalent of Objective-C's `SEL _Nonnull` type. You can create
/// this statically using the [`sel!`] macro.
///
/// The main reason the Objective-C runtime uses a custom type for selectors,
/// as opposed to a plain c-string, is to support efficient comparison - a
/// a selector is effectively an [interned string], so this makes equiality
/// comparisons very cheap.
///
/// This struct guarantees the null-pointer optimization, namely that
/// `Option<Sel>` is the same size as `Sel`.
///
/// Selectors are immutable.
///
/// [`sel!`]: crate::sel
/// [interned string]: https://en.wikipedia.org/wiki/String_interning
#[repr(transparent)]
#[derive(Copy, Clone)]
#[doc(alias = "SEL")]
#[doc(alias = "objc_selector")]
pub struct Sel {
    ptr: NonNull<c_void>,
}

// SAFETY: Sel is immutable (and can be retrieved from any thread using the
// `sel!` macro).
unsafe impl Sync for Sel {}
unsafe impl Send for Sel {}
impl UnwindSafe for Sel {}
impl RefUnwindSafe for Sel {}

impl Sel {
    #[inline]
    #[doc(hidden)]
    pub const unsafe fn __internal_from_ptr(ptr: *const u8) -> Self {
        // Used in static selectors.
        // SAFETY: Upheld by caller.
        let ptr = unsafe { NonNull::new_unchecked(ptr as *mut c_void) };
        Self { ptr }
    }

    #[inline]
    pub(crate) unsafe fn from_ptr(ptr: *const c_void) -> Option<Self> {
        // SAFETY: Caller verifies that the pointer is valid.
        NonNull::new(ptr as *mut c_void).map(|ptr| Self { ptr })
    }

    #[inline]
    pub(crate) const fn as_ptr(&self) -> *const c_void {
        self.ptr.as_ptr()
    }

    // We explicitly don't do #[track_caller] here, since we expect the error
    // to never actually happen.
    pub(crate) unsafe fn register_unchecked(name: *const c_char) -> Self {
        let ptr = unsafe { ffi::sel_registerName(name) };
        // SAFETY: `sel_registerName` declares return type as `SEL _Nonnull`,
        // at least when input is also `_Nonnull` (which it is in our case).
        //
        // Looking at the source code, it can fail and will return NULL if
        // allocating space for the selector failed (which then subsequently
        // invokes UB by calling `memcpy` with a NULL argument):
        // <https://github.com/apple-oss-distributions/objc4/blob/objc4-841.13/runtime/objc-os.h#L1002-L1004>
        //
        // I suspect this will be really uncommon in practice, since the
        // called selector is almost always going to be present in the binary
        // already; but alas, we'll handle it!
        ptr.expect("failed allocating selector")
    }

    /// Registers a selector with the Objective-C runtime.
    ///
    /// This is the dynamic version of the [`sel!`] macro, prefer to use that
    /// when your selector is static.
    ///
    /// [`sel!`]: crate::sel
    ///
    ///
    /// # Panics
    ///
    /// Panics if the runtime failed allocating space for the selector.
    #[inline]
    #[doc(alias = "sel_registerName")]
    pub fn register(name: &CStr) -> Self {
        // SAFETY: Input is a non-null, NUL-terminated C-string pointer.
        unsafe { Self::register_unchecked(name.as_ptr()) }
    }

    /// Returns the string representation of the selector.
    #[inline]
    #[doc(alias = "sel_getName")]
    pub fn name(self) -> &'static CStr {
        // SAFETY: Input is non-null selector. Declares return type as
        // `const char * _Nonnull`, source code agrees.
        let ptr = unsafe { ffi::sel_getName(self) };
        // SAFETY: The string is a valid C-style NUL-terminated string, and
        // has static lifetime since the selector has static lifetime.
        unsafe { CStr::from_ptr(ptr) }
    }

    pub(crate) fn number_of_arguments(self) -> usize {
        self.name()
            .to_bytes()
            .iter()
            .filter(|&&b| b == b':')
            .count()
    }
}

impl PartialEq for Sel {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        if cfg!(feature = "gnustep-1-7") {
            // GNUStep implements "typed" selectors, which means their pointer
            // values sometimes differ; so let's use the runtime-provided
            // `sel_isEqual`.
            unsafe { ffi::sel_isEqual(*self, *other).as_bool() }
        } else {
            // `ffi::sel_isEqual` uses pointer comparison on Apple (the
            // documentation explicitly notes this); so as an optimization,
            // let's do that as well!
            ptr::eq(self.as_ptr(), other.as_ptr())
        }
    }
}

impl Eq for Sel {}

impl hash::Hash for Sel {
    #[inline]
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        if cfg!(feature = "gnustep-1-7") {
            // Note: We hash the name instead of the pointer on GNUStep, since
            // they're typed.
            self.name().hash(state);
        } else {
            self.as_ptr().hash(state);
        }
    }
}

// SAFETY: `Sel` is FFI compatible, and the encoding is `Sel`.
unsafe impl Encode for Sel {
    const ENCODING: Encoding = Encoding::Sel;
}

unsafe impl OptionEncode for Sel {}

// RefEncode is not implemented for Sel, because there is literally no API
// that takes &Sel, while the user could get confused and accidentally attempt
// that.

impl fmt::Display for Sel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Selectors are basically always UTF-8, so it's _fine_ to do a lossy
        // conversion here.
        fmt::Display::fmt(&self.name().to_string_lossy(), f)
    }
}

impl fmt::Debug for Sel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Sel").field(&self.name()).finish()
    }
}

impl fmt::Pointer for Sel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Pointer::fmt(&self.ptr, f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime::{test_utils, AnyClass, ClassBuilder, MessageReceiver, NSObject};
    use crate::{sel, ClassType};
    use alloc::ffi::CString;
    use alloc::string::ToString;

    #[test]
    fn test_multiple_colon() {
        let class = test_utils::custom_class();
        let res: i32 = unsafe {
            MessageReceiver::send_message(class, sel!(test::test::), (1i32, 2i32, 3i32, 4i32))
        };
        assert_eq!(res, 10);

        let obj = test_utils::custom_object();
        let res: i32 = unsafe {
            MessageReceiver::send_message(&*obj, sel!(test::test::), (1i32, 2i32, 3i32, 4i32))
        };
        assert_eq!(res, 24);
    }

    #[test]
    fn test_selector() {
        macro_rules! test_sel {
            ($s:literal, $($tt:tt)+) => {{
                let sel = $crate::sel!($($tt)*);
                let expected = Sel::register(&CString::new($s).unwrap());
                assert_eq!(sel, expected);
                assert_eq!(sel.name().to_str(), Ok($s));
            }}
        }
        test_sel!("abc", abc);
        test_sel!("abc:", abc:);
        test_sel!("abc:def:", abc:def:);
        test_sel!("abc:def:ghi:", abc:def:ghi:);
        test_sel!("functionWithControlPoints::::", functionWithControlPoints::::);
        test_sel!("initWithControlPoints::::", initWithControlPoints::::);
        test_sel!("setFloatValue::", setFloatValue::);
        test_sel!("isSupported::", isSupported::);
        test_sel!("addEventListener:::", addEventListener:::);
        test_sel!("test::arg::", test::arg::);
        test_sel!("test::::with::spaces::", test : :: : with : : spaces : :);
        test_sel!("a::b:", a::b:);
    }

    #[test]
    fn test_empty_selector() {
        let s = CStr::from_bytes_with_nul(b"\0").unwrap();
        let sel = Sel::register(&s);
        assert_eq!(sel.name(), &*s);
        let s = CStr::from_bytes_with_nul(b":\0").unwrap();
        let sel = Sel::register(&s);
        assert_eq!(sel.name(), &*s);
        let s = CStr::from_bytes_with_nul(b"::\0").unwrap();
        let sel = Sel::register(&s);
        assert_eq!(sel.name(), &*s);
    }

    #[test]
    fn test_non_utf8_roundtrip() {
        // Some invalid UTF-8 character
        let s = CStr::from_bytes_with_nul(b"\x9F\0").unwrap();

        let sel = Sel::register(s);
        assert_eq!(sel.name(), s);
        assert_eq!(sel.to_string(), char::REPLACEMENT_CHARACTER.to_string());

        let cls = ClassBuilder::new(s, NSObject::class()).unwrap().register();
        assert_eq!(cls.name(), s);
        assert_eq!(cls.to_string(), char::REPLACEMENT_CHARACTER.to_string());

        let cls_runtime = AnyClass::get(s).unwrap();
        assert_eq!(cls, cls_runtime);
    }
}
