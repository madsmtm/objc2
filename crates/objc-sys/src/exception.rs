//! Defined in:
//! Apple: `objc-exception.h`
//! GNUStep: `eh_personality.c`, which is a bit brittle to rely on, but I
//!   think it's fine...

// A few things here are defined differently depending on the __OBJC2__
// variable, which is set for all platforms except 32-bit macOS.

#[cfg(any(
    doc,
    all(
        target_vendor = "apple",
        not(all(target_os = "macos", target_arch = "x86"))
    )
))]
use crate::objc_class;
use crate::objc_object;

/// Remember that this is non-null!
#[cfg(any(
    doc,
    all(
        target_vendor = "apple",
        not(all(target_os = "macos", target_arch = "x86"))
    )
))]
pub type objc_exception_matcher = unsafe extern "C" fn(
    catch_type: *mut objc_class,
    exception: *mut objc_object,
) -> std::os::raw::c_int;

/// Remember that this is non-null!
#[cfg(any(
    doc,
    all(
        target_vendor = "apple",
        not(all(target_os = "macos", target_arch = "x86"))
    )
))]
pub type objc_exception_preprocessor =
    unsafe extern "C" fn(exception: *mut objc_object) -> *mut objc_object;

/// Remember that this is non-null!
#[cfg(any(
    doc,
    all(
        target_vendor = "apple",
        not(all(target_os = "macos", target_arch = "x86"))
    )
))]
pub type objc_uncaught_exception_handler = unsafe extern "C" fn(exception: *mut objc_object);

#[cfg(feature = "unstable-objfw")]
pub type objc_uncaught_exception_handler =
    Option<unsafe extern "C" fn(exception: *mut objc_object)>;

/// Remember that this is non-null!
#[cfg(any(
    doc,
    all(target_vendor = "apple", target_os = "macos", not(target_arch = "x86"))
))]
pub type objc_exception_handler =
    unsafe extern "C" fn(unused: *mut objc_object, context: *mut core::ffi::c_void);

#[cfg(all(feature = "unstable-exception", not(feature = "unstable-c-unwind")))]
type TryCatchClosure = extern "C" fn(*mut core::ffi::c_void);
#[cfg(all(feature = "unstable-exception", feature = "unstable-c-unwind"))]
type TryCatchClosure = extern "C-unwind" fn(*mut core::ffi::c_void);

extern_c_unwind! {
    /// See [`objc-exception.h`].
    ///
    /// [`objc-exception.h`]: https://github.com/apple-oss-distributions/objc4/blob/objc4-818.2/runtime/objc-exception.h
    #[cold]
    pub fn objc_exception_throw(exception: *mut objc_object) -> !;

    #[cfg(all(target_vendor = "apple", not(feature = "gnustep-1-7"), not(all(target_os = "macos", target_arch = "x86"))))]
    #[cold]
    pub fn objc_exception_rethrow() -> !;

    #[cfg(feature = "gnustep-1-7")]
    #[cold]
    pub fn objc_exception_rethrow(exc_buf: *mut core::ffi::c_void) -> !;
}

extern_c! {
    #[cfg(any(doc, feature = "gnustep-1-7", all(target_vendor = "apple", not(all(target_os = "macos", target_arch = "x86")))))]
    pub fn objc_begin_catch(exc_buf: *mut core::ffi::c_void) -> *mut objc_object;
    #[cfg(any(doc, feature = "gnustep-1-7", all(target_vendor = "apple", not(all(target_os = "macos", target_arch = "x86")))))]
    pub fn objc_end_catch();

    #[cfg(any(doc, all(target_vendor = "apple", target_os = "macos", target_arch = "x86")))]
    pub fn objc_exception_try_enter(exception_data: *const core::ffi::c_void);

    #[cfg(any(doc, all(target_vendor = "apple", target_os = "macos", target_arch = "x86")))]
    pub fn objc_exception_try_exit(exception_data: *const core::ffi::c_void);

    // objc_exception_extract
    // objc_exception_match
    // objc_exception_get_functions
    // objc_exception_set_functions

    #[cfg(any(doc, all(target_vendor = "apple", not(all(target_os = "macos", target_arch = "x86")))))]
    pub fn objc_setExceptionMatcher(f: objc_exception_matcher) -> objc_exception_matcher;
    #[cfg(any(doc, all(target_vendor = "apple", not(all(target_os = "macos", target_arch = "x86")))))]
    pub fn objc_setExceptionPreprocessor(
        f: objc_exception_preprocessor,
    ) -> objc_exception_preprocessor;
    #[cfg(any(doc, all(target_vendor = "apple", not(all(target_os = "macos", target_arch = "x86"))), feature = "unstable-objfw"))]
    pub fn objc_setUncaughtExceptionHandler(
        f: objc_uncaught_exception_handler,
    ) -> objc_uncaught_exception_handler;

    #[cfg(any(doc, all(target_vendor = "apple", target_os = "macos", not(target_arch = "x86"))))]
    pub fn objc_addExceptionHandler(f: objc_exception_handler, context: *mut core::ffi::c_void) -> usize;
    #[cfg(any(doc, all(target_vendor = "apple", target_os = "macos", not(target_arch = "x86"))))]
    pub fn objc_removeExceptionHandler(token: usize);

    // Only available when ENABLE_OBJCXX is set, and a useable C++ runtime is
    // present when building libobjc2.
    //
    // #[cfg(any(doc, feature = "gnustep-1-7"))]
    // pub fn objc_set_apple_compatible_objcxx_exceptions(newValue: std::os::raw::c_int) -> std::os::raw::c_int;

    #[cold]
    #[cfg(any(doc, all(target_vendor = "apple", not(all(target_os = "macos", target_arch = "x86")))))]
    pub fn objc_terminate() -> !;
}

#[cfg(feature = "unstable-exception")]
#[deprecated = "re-exported from `objc2-exception-helper`"]
pub unsafe extern "C" fn try_catch(
    f: TryCatchClosure,
    context: *mut core::ffi::c_void,
    error: *mut *mut objc_object,
) -> std::os::raw::c_uchar {
    unsafe { objc2_exception_helper::try_catch(std::mem::transmute(f), context, error.cast()) }
}
