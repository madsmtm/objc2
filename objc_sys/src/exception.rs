use core::ffi::c_void;

use crate::{
    objc_exception_matcher, objc_exception_preprocessor, objc_object,
    objc_uncaught_exception_handler,
};

extern "C" {
    pub fn objc_begin_catch(exc_buf: *mut c_void) -> *mut objc_object;
    pub fn objc_end_catch();
    pub fn objc_exception_throw(exception: *mut objc_object);
    pub fn objc_exception_rethrow();

    pub fn objc_setEnumerationMutationHandler(
        handler: Option<unsafe extern "C" fn(arg1: *mut objc_object)>,
    );
    pub fn objc_setExceptionMatcher(fn_: objc_exception_matcher) -> objc_exception_matcher;
    pub fn objc_setExceptionPreprocessor(
        fn_: objc_exception_preprocessor,
    ) -> objc_exception_preprocessor;
    pub fn objc_setUncaughtExceptionHandler(
        fn_: objc_uncaught_exception_handler,
    ) -> objc_uncaught_exception_handler;
}
