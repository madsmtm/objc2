use core::ffi::{c_char, c_void};
use core::ptr;
use core::str;
use core::sync::atomic::{AtomicPtr, Ordering};

use crate::runtime::Sel;

/// Register a selector with the Objective-C runtime.
///
/// Returns the [`Sel`] corresponding to the specified selector.
///
/// [`Sel`]: crate::runtime::Sel
///
///
/// # Panics
///
/// Panics if the runtime failed allocating space for the selector.
///
///
/// # Specification
///
/// This has similar syntax and functionality as the `@selector` directive in
/// Objective-C.
///
/// This calls [`Sel::register`] internally. The result is cached for
/// efficiency. The cache for certain common selectors (`alloc`, `init` and
/// `new`) is deduplicated to reduce code-size.
///
/// Non-ascii identifiers are ill-tested, if supported at all.
///
/// [`Sel::register`]: crate::runtime::Sel::register
///
///
/// # Features
///
/// If the experimental `"unstable-static-sel"` feature is enabled, this will
/// emit special statics that will be replaced by the dynamic linker (dyld)
/// when the program starts up - in exactly the same manner as normal
/// Objective-C code does.
/// This should be significantly faster (and allow better native debugging),
/// however due to the Rust compilation model, and since we don't have
/// low-level control over it, it is currently unlikely that this will work
/// correctly in all cases.
/// See the source code and [rust-lang/rust#53929] for more info.
///
/// Concretely, this may fail at:
/// - link-time (likely)
/// - dynamic link-time/just before the program is run (fairly likely)
/// - runtime, causing UB (unlikely)
///
/// The `"unstable-static-sel-inlined"` feature is the even more extreme
/// version - it yield better performance and is closer to real
/// Objective-C code, but probably won't work unless your code and its
/// inlining is written in a very certain way.
///
/// Enabling LTO greatly increases the chance that these features work.
///
/// On Apple/Darwin targets, these limitations can be overcome with the
/// `"unstable-darwin-objc"` feature which uses the nightly-only `darwin_objc`
/// language feature. This experimental language feature implements the
/// Objective-C static selector ABI directly in the Rust compiler and should
/// work in more if not all cases. Using `"unstable-darwin-objc"` requires
/// `darwin_objc` to be enabled in every crate that uses this macro, which can
/// be achieved in `objc2` crates by enabling their own
/// `"unstable-darwin-objc"` features and in your own crates by adding
/// `#![feature(darwin_objc)]`.
///
/// See [rust-lang/rust#145496] for the tracking issue for the feature.
///
/// [rust-lang/rust#53929]: https://github.com/rust-lang/rust/issues/53929
/// [rust-lang/rust#145496]: https://github.com/rust-lang/rust/issues/145496
///
///
/// # Examples
///
/// Get a few different selectors:
///
/// ```rust
/// use objc2::sel;
/// let sel = sel!(alloc);
/// let sel = sel!(description);
/// let sel = sel!(_privateMethod);
/// let sel = sel!(storyboardWithName:bundle:);
/// let sel = sel!(
///     otherEventWithType:
///     location:
///     modifierFlags:
///     timestamp:
///     windowNumber:
///     context:
///     subtype:
///     data1:
///     data2:
/// );
/// ```
///
/// Whitespace is ignored:
///
/// ```
/// # use objc2::sel;
/// let sel1 = sel!(setObject:forKey:);
/// let sel2 = sel!(  setObject  :
///
///     forKey  : );
/// assert_eq!(sel1, sel2);
/// ```
///
/// Invalid selector:
///
/// ```compile_fail
/// # use objc2::sel;
/// let sel = sel!(aSelector:withoutTrailingColon);
/// ```
///
/// A selector with internal colons:
///
/// ```
/// # use objc2::sel;
/// let sel = sel!(sel::with:::multiple:internal::::colons:::);
///
/// // Yes, that is possible! The following Objective-C would work:
/// //
/// // @interface MyThing: NSObject
/// // + (void)test:(int)a :(int)b arg:(int)c :(int)d;
/// // @end
/// ```
///
/// Unsupported usage that you may run into when using macros - fails to
/// compile when the `"unstable-static-sel"` feature is enabled.
///
/// Instead, define a wrapper function that retrieves the selector.
///
#[cfg_attr(not(feature = "unstable-static-sel"), doc = "```no_run")]
#[cfg_attr(feature = "unstable-static-sel", doc = "```compile_fail")]
/// use objc2::sel;
/// macro_rules! x {
///     ($x:ident) => {
///         // One of these is fine
///         sel!($x);
///         // But using the identifier again in the same way is not!
///         sel!($x);
///     };
/// }
/// // Identifier `abc`
/// x!(abc);
/// ```
#[macro_export]
macro_rules! sel {
    (new) => ({
        $crate::__macros::new_sel()
    });
    (init) => ({
        $crate::__macros::init_sel()
    });
    (alloc) => ({
        $crate::__macros::alloc_sel()
    });
    (dealloc) => ({
        $crate::__macros::dealloc_sel()
    });
    ($sel:ident) => ({
        $crate::__sel_inner!(
            $crate::__sel_data!($sel),
            $crate::__hash_idents!($sel)
        )
    });
    ($($sel:ident :)*) => ({
        $crate::__sel_inner!(
            $crate::__sel_data!($($sel :)*),
            $crate::__hash_idents!($($sel :)*)
        )
    });
    ($($sel:tt)*) => {
        $crate::__sel_inner!(
            $crate::__sel_helper! {
                ()
                $($sel)*
            },
            $crate::__hash_idents!($($sel)*)
        )
    };
}

/// Handle selectors with internal colons.
///
/// Required since `::` is a different token than `:`.
#[doc(hidden)]
#[macro_export]
macro_rules! __sel_helper {
    // Base-case
    {
        ($($parsed_sel:tt)*)
    } => {
        $crate::__sel_data!($($parsed_sel)*)
    };
    // Single identifier
    {
        ()
        $ident:ident
    } => {
        $crate::__sel_helper! {
            ($ident)
        }
    };
    // Parse identitifer + colon token
    {
        ($($parsed_sel:tt)*)
        $($ident:ident)? : $($rest:tt)*
    } => {
        $crate::__sel_helper! {
            ($($parsed_sel)* $($ident)? :)
            $($rest)*
        }
    };
    // Parse identitifer + path separator token
    {
        ($($parsed_sel:tt)*)
        $($ident:ident)? :: $($rest:tt)*
    } => {
        $crate::__sel_helper! {
            // Notice space between these
            ($($parsed_sel)* $($ident)? : :)
            $($rest)*
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __sel_data {
    ($first:ident $(: $($($rest:ident)? :)*)?) => {
        $crate::__macros::concat!(
            $crate::__macros::stringify!($first),
            $(':', $($($crate::__macros::stringify!($rest),)? ':',)*)?
        )
    };
}

#[doc(hidden)]
#[macro_export]
#[cfg(not(feature = "unstable-static-sel"))]
macro_rules! __sel_inner {
    ($data:expr, $_hash:expr) => {{
        static CACHED_SEL: $crate::__macros::CachedSel = $crate::__macros::CachedSel::new();
        #[allow(unused_unsafe)]
        unsafe {
            CACHED_SEL.get($crate::__macros::concat!($data, '\0'))
        }
    }};
}

#[doc(hidden)]
#[macro_export]
#[cfg(all(feature = "unstable-static-sel", feature = "unstable-darwin-objc"))]
macro_rules! __sel_inner {
    ($data:expr, $_hash:expr) => {{
        let ptr = $crate::__macros::core_darwin_objc::selector!($data);
        let ptr = ptr.cast_const().cast::<$crate::__macros::u8>();
        #[allow(unused_unsafe)]
        unsafe {
            $crate::runtime::Sel::__internal_from_ptr(ptr)
        }
    }};
}

#[doc(hidden)]
#[macro_export]
#[cfg(target_vendor = "apple")]
macro_rules! __statics_sel {
    {
        ($data:expr)
        ($hash:expr)
    } => {
        const X: &[$crate::__macros::u8] = $crate::__macros::concat!($data, '\0').as_bytes();

        /// Clang marks this with LLVM's `unnamed_addr`.
        /// See rust-lang/rust#18297
        /// Should only be an optimization (?)
        #[cfg_attr(
            not(all(target_os = "macos", target_arch = "x86")),
            link_section = "__TEXT,__objc_methname,cstring_literals",
        )]
        #[cfg_attr(
            all(target_os = "macos", target_arch = "x86"),
            link_section = "__TEXT,__cstring,cstring_literals",
        )]
        #[export_name = $crate::__macros::concat!(
            "\x01L_OBJC_METH_VAR_NAME_",
            $hash,
        )]
        static NAME_DATA: [$crate::__macros::u8; X.len()] = $crate::__statics_string_to_known_length_bytes!(X);

        /// Place the constant value in the correct section.
        ///
        /// We use `UnsafeCell` because this somewhat resembles internal
        /// mutation - this pointer will be changed by dyld at startup, so we
        /// _must_ prevent Rust/LLVM from trying to "peek inside" it and just
        /// use a pointer to `NAME_DATA` directly.
        ///
        /// Clang does this by marking `REF` with LLVM's
        /// `externally_initialized`.
        ///
        ///
        /// # Safety
        ///
        /// I'm quite uncertain of how safe this is, since the Rust abstract
        /// machine has no concept of a static that is initialized outside of
        /// it - perhaps it would be better to use `read_volatile` instead of
        /// relying on `UnsafeCell`? Or perhaps `MaybeUninit` would help?
        ///
        /// See the [`ctor`](https://docs.rs/ctor) crate for more info on
        /// "life before main".
        #[cfg_attr(
            not(all(target_os = "macos", target_arch = "x86")),
            // Clang uses `no_dead_strip` in the link section for some unknown reason,
            // but it makes LTO fail to trim the unused symbols.
            // https://github.com/madsmtm/objc2/issues/667
            // https://github.com/llvm/llvm-project/issues/114111
            link_section = "__DATA,__objc_selrefs,literal_pointers",
        )]
        #[cfg_attr(
            all(target_os = "macos", target_arch = "x86"),
            link_section = "__OBJC,__message_refs,literal_pointers",
        )]
        #[export_name = $crate::__macros::concat!("\x01L_OBJC_SELECTOR_REFERENCES_", $hash)]
        static REF: $crate::__macros::SyncUnsafeCell<$crate::runtime::Sel> = unsafe {
            $crate::__macros::SyncUnsafeCell::new($crate::runtime::Sel::__internal_from_ptr(NAME_DATA.as_ptr()))
        };

        $crate::__statics_image_info!($hash);
    };
}

#[doc(hidden)]
#[macro_export]
#[cfg(not(target_vendor = "apple"))]
macro_rules! __statics_sel {
    ($($args:tt)*) => {
        // TODO
        $crate::__macros::compile_error!(
            "The `\"unstable-static-sel\"` feature is not yet supported on GNUStep!"
        )
    };
}

#[doc(hidden)]
#[macro_export]
#[cfg(all(
    feature = "unstable-static-sel",
    not(feature = "unstable-darwin-objc"),
    not(feature = "unstable-static-sel-inlined"),
))]
macro_rules! __sel_inner {
    ($data:expr, $hash:expr) => {{
        $crate::__statics_sel! {
            ($data)
            ($hash)
        }

        /// HACK: Wrap the access in a non-generic, `#[inline(never)]`
        /// function to make the compiler group it into the same codegen unit
        /// as the statics.
        ///
        /// See the following link for details on how the compiler decides
        /// to partition code into codegen units:
        /// <https://doc.rust-lang.org/1.61.0/nightly-rustc/rustc_monomorphize/partitioning/index.html>
        #[inline(never)]
        fn objc_static_workaround() -> $crate::runtime::Sel {
            // SAFETY: The actual selector is replaced by dyld when the
            // program is loaded.
            //
            // This is similar to a volatile read, except it can be stripped
            // if unused.
            unsafe { *REF.get() }
        }

        objc_static_workaround()
    }};
}

#[doc(hidden)]
#[macro_export]
#[cfg(all(
    feature = "unstable-static-sel",
    not(feature = "unstable-darwin-objc"),
    feature = "unstable-static-sel-inlined",
))]
macro_rules! __sel_inner {
    ($data:expr, $hash:expr) => {{
        $crate::__statics_sel! {
            ($data)
            ($hash)
        }

        #[allow(unused_unsafe)]
        // SAFETY: See above
        unsafe {
            *REF.get()
        }
    }};
}

/// Allows storing a [`Sel`] in a static and lazily loading it.
#[derive(Debug)]
pub struct CachedSel {
    ptr: AtomicPtr<c_void>,
}

impl CachedSel {
    /// Constructs a new [`CachedSel`].
    #[allow(clippy::new_without_default)]
    pub const fn new() -> Self {
        Self {
            ptr: AtomicPtr::new(ptr::null_mut()),
        }
    }

    // Mark as cold since this should only ever be called once (or maybe twice
    // if running on multiple threads).
    #[cold]
    unsafe fn fetch(&self, name: *const c_char) -> Sel {
        // The panic inside `Sel::register_unchecked` is unfortunate, but
        // strict correctness is more important than speed

        // SAFETY: Input is a non-null, NUL-terminated C-string pointer.
        //
        // We know this, because we construct it in `sel!` ourselves
        let sel = unsafe { Sel::register_unchecked(name) };
        self.ptr.store(sel.as_ptr() as *mut _, Ordering::Relaxed);
        sel
    }

    /// Returns the cached selector. If no selector is yet cached, registers
    /// one with the given name and stores it.
    #[inline]
    pub unsafe fn get(&self, name: &str) -> Sel {
        // `Relaxed` should be fine since `sel_registerName` is thread-safe.
        let ptr = self.ptr.load(Ordering::Relaxed);
        if let Some(sel) = unsafe { Sel::from_ptr(ptr) } {
            sel
        } else {
            // SAFETY: Checked by caller
            unsafe { self.fetch(name.as_ptr().cast()) }
        }
    }
}

// Common selectors.
//
// These are put here to deduplicate the cached selector, and when using
// `unstable-static-sel`, the statics.
//
// Note that our assembly tests of `unstable-static-sel-inlined` output a GOT
// entry for such accesses, but that is just a limitation of our tests - the
// actual assembly is as one would expect.

#[inline]
pub fn alloc_sel() -> Sel {
    __sel_inner!("alloc", "alloc")
}

#[inline]
pub fn init_sel() -> Sel {
    __sel_inner!("init", "init")
}

#[inline]
pub fn new_sel() -> Sel {
    __sel_inner!("new", "new")
}

#[inline]
pub fn dealloc_sel() -> Sel {
    __sel_inner!("dealloc", "dealloc")
}

/// An undocumented selector called by the Objective-C runtime when
/// initializing instance variables.
#[inline]
#[allow(dead_code)] // May be useful in the future
fn cxx_construct_sel() -> Sel {
    __sel_inner!(".cxx_construct", ".cxx_construct")
}

/// Objective-C runtimes call `.cxx_destruct` as part of the final `dealloc`
/// call inside `NSObject` (and has done so since macOS 10.4).
///
/// While [GCC does document it somewhat][gcc-docs], this is still severely
/// undocumented in clang - but since the selector is emitted into the final
/// binary, it is fine to rely on it being used.
///
/// Unfortunately though, this only works if the class has been defined
/// statically, since in that case a flag is set to inform the runtime that it
/// needs to run destructors. So unfortunately we can't use this on
/// dynamically defined classes.
///
///
/// # ABI
///
/// The ABI of `.cxx_destruct` in Apple's runtime is actually that it does NOT
/// take a selector, unlike every other Objective-C method, see:
/// <https://github.com/apple-oss-distributions/objc4/blob/objc4-906/runtime/objc-class.mm#L457>
///
/// So the signature is `extern "C-unwind" fn(*mut AnyObject)`.
///
/// This is likely because it's not a real Objective-C method that can be
/// called from userspace / objc_msgSend, and it's more efficient to not pass
/// the selector.
///
/// Note that even if Apple decides to suddenly add the selector one day,
/// ignoring it will still be sound, since the function uses the C calling
/// convention, where such an ignored parameter would be allowed on all
/// relevant architectures.
///
/// [gcc-docs]: https://gcc.gnu.org/onlinedocs/gcc/Objective-C-and-Objective-C_002b_002b-Dialect-Options.html#index-fobjc-call-cxx-cdtors
#[inline]
#[allow(dead_code)] // May be useful in the future
fn cxx_destruct_sel() -> Sel {
    __sel_inner!(".cxx_destruct", ".cxx_destruct")
}

#[cfg(test)]
mod tests {
    use alloc::ffi::CString;
    use core::sync::atomic::{AtomicBool, Ordering};

    use crate::rc::Retained;
    use crate::runtime::ClassBuilder;
    use crate::runtime::NSObject;
    use crate::{msg_send, ClassType};

    use super::*;

    /// Test the unfortunate fact that we can't use .cxx_destruct on dynamic classes.
    #[test]
    fn test_destruct_dynamic() {
        static HAS_RUN: AtomicBool = AtomicBool::new(false);

        let name = CString::new("TestCxxDestruct").unwrap();
        let mut builder = ClassBuilder::new(&name, NSObject::class()).unwrap();

        unsafe extern "C" fn destruct(_: *mut NSObject, _: Sel) {
            HAS_RUN.store(true, Ordering::Relaxed);
        }

        // Note: The ABI is not upheld here, but its fine for this test
        unsafe { builder.add_method(cxx_destruct_sel(), destruct as unsafe extern "C" fn(_, _)) };

        let cls = builder.register();

        let obj: Retained<NSObject> = unsafe { msg_send![cls, new] };
        drop(obj);
        let has_run_destruct = HAS_RUN.load(Ordering::Relaxed);

        // This does work on GNUStep, but unfortunately not in Apple's objc4
        if cfg!(feature = "gnustep-1-7") {
            assert!(has_run_destruct);
        } else {
            assert!(!has_run_destruct);
        }
    }
}
