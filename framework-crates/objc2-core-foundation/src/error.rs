use core::{
    ffi::{c_char, CStr},
    ptr::NonNull,
};

use crate::{kCFAllocatorNull, CFAllocator, CFError, CFIndex, CFRetained, CFType};

#[cfg(feature = "CFString")]
impl core::fmt::Display for CFError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.description())
    }
}

#[cfg(feature = "std")] // use core::error::Error from Rust 1.81 once in MSRV.
#[cfg(feature = "CFString")]
impl std::error::Error for CFError {}

impl CFError {
    /// An error used when a `Result<T, CFRetained<CFError>>` function fails
    /// but (incorrectly) does not set the error.
    #[cold] // Mark the NULL error branch as cold
    #[doc(hidden)] // Internal, though public API used in other crates.
    pub fn __null() -> CFRetained<Self> {
        // TODO: Replace with c string literals once in MSRV.
        // TODO: Replace with `cf_string!` macro once we add that.

        // Matches the NULL error returned by `msg_send!`.
        //
        // SAFETY: The string is NUL terminated.
        let domain = unsafe { CStr::from_bytes_with_nul_unchecked(b"__objc2.missingError\0") };
        // kCFStringEncodingUTF8
        let domain = new_string(domain, 0x08000100).expect("failed creating domain string");

        let code = 0;

        // SAFETY: The user info is NULL.
        unsafe { new_error(None, &domain, code, None) }.expect("failed allocating NUL error")
    }
}

// Duplicate of `CFError::new`, but without any Cargo feature bounds.
#[inline]
unsafe fn new_error(
    allocator: Option<&CFAllocator>,
    domain: &CFType,
    code: CFIndex,
    user_info: Option<&CFType>,
) -> Option<CFRetained<CFError>> {
    extern "C-unwind" {
        fn CFErrorCreate(
            allocator: Option<&CFAllocator>,
            domain: &CFType,
            code: CFIndex,
            user_info: Option<&CFType>,
        ) -> Option<NonNull<CFError>>;
    }
    let ret = unsafe { CFErrorCreate(allocator, domain, code, user_info) };
    ret.map(|ret| unsafe { CFRetained::from_raw(ret) })
}

// Duplicate of `CFString::with_c_string_no_copy`, but without any Cargo feature bounds.
fn new_string(c_str: &'static CStr, encoding: u32) -> Option<CFRetained<CFType>> {
    extern "C-unwind" {
        fn CFStringCreateWithCStringNoCopy(
            alloc: Option<&CFAllocator>,
            c_str: NonNull<c_char>,
            encoding: u32,
            contents_deallocator: Option<&CFAllocator>,
        ) -> Option<NonNull<CFType>>;
    }
    let c_str = NonNull::new(c_str.as_ptr().cast_mut()).unwrap();
    let ret = unsafe { CFStringCreateWithCStringNoCopy(None, c_str, encoding, kCFAllocatorNull) };
    ret.map(|ret| unsafe { CFRetained::from_raw(ret) })
}

#[cfg(test)]
#[cfg(feature = "CFString")]
mod tests {
    use std::string::ToString;

    use super::*;

    #[test]
    fn null() {
        let err = CFError::__null();
        assert_eq!(err.domain().to_string(), "__objc2.missingError");
        assert_eq!(err.code(), 0);
    }
}
