//! External helper methods for catching Objective-C exceptions.
//!
//! This exists as a separate crate to avoid having to compile a `build.rs`
//! script in `objc2` in most cases, and to properly version the compiled
//! binary with [the `links` Cargo manifest key][cargo-links].
//!
//! You should not need to use this crate directly.
//!
//! [cargo-links]: https://doc.rust-lang.org/cargo/reference/build-scripts.html#the-links-manifest-key
#![no_std]
#![doc(html_root_url = "https://docs.rs/objc2-exception-helper/0.1.1")]

// Forwards-compatibility
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

use core::ffi::c_void;

type TryCatchClosure = extern "C-unwind" fn(*mut c_void);

// `try_catch` is `extern "C-unwind"`, since it does not use `@catch (...)`,
// but instead let unhandled exceptions pass through.
extern "C-unwind" {
    /// Call the given function inside an Objective-C `@try/@catch` block.
    ///
    /// Defined in `src/try_catch.m` and compiled in `build.rs`.
    ///
    /// Alternatively, we could manually write assembly for this function like
    /// [`objrs` does][manual-asm] does, that would cut down on a build stage
    /// (and would probably give us a bit better performance), but it gets
    /// unwieldy _very_ quickly, so I chose the much more stable option.
    ///
    /// Another thing to remember: While Rust's and Objective-C's unwinding
    /// mechanisms are similar now, Rust's is explicitly unspecified, and they
    /// may diverge significantly in the future; so handling this in pure Rust
    /// (using mechanisms like core::intrinsics::r#try) is not an option!
    ///
    /// [manual-asm]: https://gitlab.com/objrs/objrs/-/blob/b4f6598696b3fa622e6fddce7aff281770b0a8c2/src/exception.rs
    ///
    ///
    /// # Panics
    ///
    /// This panics / continues unwinding if the unwind is not triggered by an
    /// Objective-C exception (i.e. it was triggered by Rust/C++/...).
    #[link_name = "objc2_exception_helper_0_1_try_catch"]
    pub fn try_catch(f: TryCatchClosure, context: *mut c_void, error: *mut *mut c_void) -> u8;
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::ptr;

    #[link(name = "objc", kind = "dylib")]
    extern "C" {}

    #[test]
    fn context_is_passed_through_correctly() {
        struct SyncPtr(*mut c_void);
        unsafe impl Sync for SyncPtr {}

        static VALUE: SyncPtr = SyncPtr(&VALUE.0 as *const *mut c_void as *mut c_void);

        extern "C-unwind" fn check_value(value: *mut c_void) {
            assert_eq!(VALUE.0, value);
        }

        let mut error: *mut c_void = ptr::null_mut();
        let res = unsafe { try_catch(check_value, VALUE.0, &mut error) };
        assert_eq!(res, 0);
        assert!(error.is_null());
    }
}
