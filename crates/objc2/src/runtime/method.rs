use core::ffi::c_uint;
use core::ffi::CStr;
use core::fmt;
use core::hash;
use core::panic::{RefUnwindSafe, UnwindSafe};
use core::ptr::{self, NonNull};

use crate::ffi;

use super::MallocCStr;
use super::MethodEncodingIter;
use super::Sel;

/// A pointer to the start of a method implementation.
///
/// The first argument is a pointer to the receiver, the second argument is
/// the selector, and the rest of the arguments follow.
///
///
/// # Safety
///
/// This is a "catch all" type; it must be transmuted to the correct type
/// before being called!
///
/// Also note that this is non-null! If you require an Imp that can be null,
/// use `Option<Imp>`.
#[doc(alias = "IMP")]
pub type Imp = unsafe extern "C-unwind" fn();

#[derive(Debug, PartialEq, Eq, Hash)]
pub(crate) struct MethodDescription {
    pub(crate) sel: Sel,
    pub(crate) types: &'static CStr,
}

impl MethodDescription {
    pub(crate) unsafe fn from_raw(raw: ffi::objc_method_description) -> Option<Self> {
        let sel = raw.name?;
        if raw.types.is_null() {
            return None;
        }
        // SAFETY: We've checked that the pointer is not NULL, rest is checked
        // by caller.
        let types = unsafe { CStr::from_ptr(raw.types) };
        Some(Self { sel, types })
    }
}

/// A type that represents a method in a class definition.
///
/// See [Apple's documentation](https://developer.apple.com/documentation/objectivec/method?language=objc).
#[repr(C)]
#[doc(alias = "objc_method")]
pub struct Method {
    _priv: [u8; 0],
    _p: ffi::OpaqueData,
}

// SAFETY: Method is immutable (and can be retrieved from AnyClass anyhow).
unsafe impl Sync for Method {}
unsafe impl Send for Method {}
impl UnwindSafe for Method {}
impl RefUnwindSafe for Method {}

impl Method {
    // Note: We don't take `&mut` here, since the operations on methods work
    // atomically.
    #[inline]
    fn as_mut_ptr(&self) -> *mut Self {
        let ptr: *const Self = self;
        ptr as _
    }

    /// Returns the name of self.
    #[inline]
    #[doc(alias = "method_getName")]
    pub fn name(&self) -> Sel {
        unsafe { ffi::method_getName(self).unwrap() }
    }

    /// Returns the `Encoding` of self's return type.
    #[doc(alias = "method_copyReturnType")]
    pub fn return_type(&self) -> MallocCStr!() {
        unsafe {
            let encoding = ffi::method_copyReturnType(self);
            MallocCStr::from_c_str(encoding)
        }
    }

    /// Returns the `Encoding` of a single parameter type of self, or
    /// [`None`] if self has no parameter at the given index.
    #[doc(alias = "method_copyArgumentType")]
    pub fn argument_type(&self, index: usize) -> Option<MallocCStr!()> {
        unsafe {
            let encoding = ffi::method_copyArgumentType(self, index as c_uint);
            NonNull::new(encoding).map(|encoding| MallocCStr::from_c_str(encoding.as_ptr()))
        }
    }

    /// An iterator over the method's types.
    ///
    /// It is approximately equivalent to:
    ///
    /// ```ignore
    /// let types = method.types();
    /// assert_eq!(types.next()?, method.return_type());
    /// for i in 0..method.arguments_count() {
    ///    assert_eq!(types.next()?, method.argument_type(i)?);
    /// }
    /// assert!(types.next().is_none());
    /// ```
    #[doc(alias = "method_getTypeEncoding")]
    pub(crate) fn types(&self) -> MethodEncodingIter<'_> {
        // SAFETY: The method pointer is valid and non-null
        let cstr = unsafe { ffi::method_getTypeEncoding(self) };
        if cstr.is_null() {
            panic!("method type encoding was NULL");
        }
        // SAFETY: `method_getTypeEncoding` returns a C-string, and we just
        // checked that it is non-null.
        let encoding = unsafe { CStr::from_ptr(cstr) };
        let s = encoding
            .to_str()
            .expect("method type encoding must be UTF-8");
        MethodEncodingIter::new(s)
    }

    /// Returns the number of arguments accepted by self.
    #[inline]
    #[doc(alias = "method_getNumberOfArguments")]
    pub fn arguments_count(&self) -> usize {
        unsafe { ffi::method_getNumberOfArguments(self) as usize }
    }

    /// Returns the implementation of this method.
    #[doc(alias = "method_getImplementation")]
    pub fn implementation(&self) -> Imp {
        unsafe { ffi::method_getImplementation(self).expect("null IMP") }
    }

    /// Set the implementation of this method.
    ///
    /// Note that any thread may at any point be changing method
    /// implementations, so if you intend to call the previous method as
    /// returned by e.g. [`Self::implementation`], beware that that may now be
    /// stale.
    ///
    /// The previous implementation is returned from this function though, so
    /// you can call that instead.
    ///
    /// See [Apple's documentation](https://developer.apple.com/documentation/objectivec/1418707-method_setimplementation?language=objc).
    ///
    ///
    /// # Safety
    ///
    /// The given implementation function pointer must:
    ///
    /// 1. Have the signature expected by the Objective-C runtime and callers
    ///    of this method.
    ///
    /// 2. Be at least as safe as the existing method, i.e. by overriding the
    ///    previous method, it should not be possible for the program to cause
    ///    UB.
    ///
    ///    A common mistake would be expecting e.g. a pointer to not be null,
    ///    where the null case was handled before.
    #[doc(alias = "method_setImplementation")]
    pub unsafe fn set_implementation(&self, imp: Imp) -> Imp {
        // SAFETY: The new impl is not NULL, and the rest is upheld by the
        // caller.
        unsafe { ffi::method_setImplementation(self.as_mut_ptr(), imp).expect("null IMP") }
    }

    /// Exchange the implementation of two methods.
    ///
    /// See [Apple's documentation](https://developer.apple.com/documentation/objectivec/1418769-method_exchangeimplementations?language=objc).
    ///
    ///
    /// # Safety
    ///
    /// The two methods must be perfectly compatible, both in signature, and
    /// in expected (in terms of safety, not necessarily behaviour) input and
    /// output.
    ///
    ///
    /// # Example
    ///
    /// This is an atomic version of the following:
    ///
    /// ```
    /// use objc2::runtime::Method;
    /// # use objc2::runtime::NSObject;
    /// # use objc2::sel;
    /// # use crate::objc2::ClassType;
    ///
    /// let m1: &Method;
    /// let m2: &Method;
    /// #
    /// # // Use the same method twice, to avoid actually changing anything
    /// # m1 = NSObject::class().instance_method(sel!(hash)).unwrap();
    /// # m2 = NSObject::class().instance_method(sel!(hash)).unwrap();
    ///
    /// unsafe {
    ///     let imp = m2.set_implementation(m1.implementation());
    ///     m1.set_implementation(imp);
    /// }
    /// ```
    #[inline]
    #[doc(alias = "method_exchangeImplementations")]
    pub unsafe fn exchange_implementation(&self, other: &Self) {
        // TODO: Consider checking that `self.types()` and `other.types()`
        // match when debug assertions are enabled?

        // SAFETY: Verified by caller
        unsafe { ffi::method_exchangeImplementations(self.as_mut_ptr(), other.as_mut_ptr()) }
    }
}

standard_pointer_impls!(Method);

impl fmt::Debug for Method {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Method")
            .field("name", &self.name())
            .field("types", &self.types())
            .field("implementation", &self.implementation())
            .finish_non_exhaustive()
    }
}
