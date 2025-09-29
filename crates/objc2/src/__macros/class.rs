use core::ffi::{c_char, CStr};
use core::ptr;
use core::str;
use core::sync::atomic::{AtomicPtr, Ordering};

use crate::ffi;
use crate::runtime::AnyClass;

/// Gets a reference to an [`AnyClass`] from the given name.
///
/// If you have an object that implements [`ClassType`], consider using the
/// [`ClassType::class`] method instead.
///
/// [`AnyClass`]: crate::runtime::AnyClass
/// [`ClassType`]: crate::ClassType
/// [`ClassType::class`]: crate::ClassType::class
///
///
/// # Panics
///
/// Panics if no class with the given name can be found.
///
/// To dynamically check for a class that may not exist, use [`AnyClass::get`].
///
/// [`AnyClass::get`]: crate::runtime::AnyClass::get
///
///
/// # Features
///
/// If the experimental `"unstable-static-class"` feature is enabled, this
/// will emit special statics that will be replaced by dyld when the program
/// starts up.
///
/// Errors that were previously runtime panics may now turn into linker errors
/// if you try to use a class which is not available. Additionally, you may
/// have to call `msg_send![cls, class]` on the result if you want to use it
/// in a dynamic context (e.g. when dynamically creating classes).
///
/// See the [corresponding section][sel#features] in the [`sel!`] macro for
/// more details on the limitations of this. The
/// `"unstable-static-class-inlined"` corresponds to the
/// `"unstable-static-sel-inlined"` feature here.
///
/// [sel#features]: crate::sel#features
/// [`sel!`]: crate::sel
///
///
/// # Examples
///
/// Get and compare the class with one returned from [`ClassType::class`].
///
/// ```
/// use objc2::runtime::NSObject;
/// use objc2::{class, ClassType};
///
/// let cls1 = class!(NSObject);
/// let cls2 = NSObject::class();
/// assert_eq!(cls1, cls2);
/// ```
///
/// Try to get a non-existing class (this will panic, or fail to link).
///
#[cfg_attr(not(feature = "unstable-static-class"), doc = "```should_panic")]
#[cfg_attr(feature = "unstable-static-class", doc = "```ignore")]
/// use objc2::class;
///
/// let _ = class!(NonExistentClass);
/// ```
#[macro_export]
macro_rules! class {
    ($name:ident) => {{
        $crate::__class_inner!(
            $crate::__macros::stringify!($name),
            $crate::__hash_idents!($name)
        )
    }};
}

#[doc(hidden)]
#[macro_export]
#[cfg(not(feature = "unstable-static-class"))]
macro_rules! __class_inner {
    ($name:expr, $_hash:expr) => {{
        static CACHED_CLASS: $crate::__macros::CachedClass = $crate::__macros::CachedClass::new();
        #[allow(unused_unsafe)]
        unsafe {
            CACHED_CLASS.get($crate::__macros::concat!($name, '\0'))
        }
    }};
}

#[doc(hidden)]
#[macro_export]
#[cfg(all(
    feature = "unstable-static-class",
    feature = "unstable-darwin-objc",
    not(feature = "gnustep-1-7"),
))]
macro_rules! __class_inner {
    ($name:expr, $_hash:expr) => {{
        let ptr = $crate::__macros::core_darwin_objc::class!($name);
        let ptr = ptr.cast_const().cast::<$crate::runtime::AnyClass>();
        #[allow(unused_unsafe)]
        let r: &'static $crate::runtime::AnyClass = unsafe { &*ptr };
        r
    }};
}

#[doc(hidden)]
#[macro_export]
#[cfg(all(
    target_vendor = "apple",
    not(all(target_os = "macos", target_arch = "x86"))
))]
macro_rules! __statics_class {
    {
        ($name:expr)
        ($hash:expr)
    } => {
        extern "C" {
            /// Link to the Objective-C class static.
            ///
            /// This uses the special symbol that static and dynamic linkers
            /// knows about.
            ///
            /// Failure modes:
            /// - Unknown class: Static linker error.
            /// - OS version < Class introduced version: Dynamic linker error
            ///   on program startup.
            /// - Deployment target > Class introduced version: No error,
            ///   though _should_ be a static linker error.
            ///
            /// Ideally, we'd have some way of allowing this to be weakly
            /// linked, and return `Option<&AnyClass>` in that case, but Rust
            /// doesn't have the capability to do so yet!
            /// <https://github.com/rust-lang/rust/issues/29603>
            /// <https://stackoverflow.com/a/16936512>
            /// <http://sealiesoftware.com/blog/archive/2010/4/8/Do-it-yourself_Objective-C_weak_import.html>
            #[link_name = $crate::__macros::concat!("OBJC_CLASS_$_", $name)]
            static CLASS: $crate::runtime::AnyClass;
        }

        /// SAFETY: Same as `REF` in `__statics_sel!`.
        #[link_section = "__DATA,__objc_classrefs,regular"]
        #[export_name = $crate::__macros::concat!(
            "\x01L_OBJC_CLASSLIST_REFERENCES_$_",
            $hash,
        )]
        static REF: $crate::__macros::SyncUnsafeCell<&$crate::runtime::AnyClass> = unsafe {
            $crate::__macros::SyncUnsafeCell::new(&CLASS)
        };

        $crate::__statics_image_info!($hash);
    };
}

#[doc(hidden)]
#[macro_export]
#[cfg(all(target_vendor = "apple", all(target_os = "macos", target_arch = "x86")))]
macro_rules! __statics_class {
    {
        ($name:expr)
        ($hash:expr)
    } => {
        const X: &[$crate::__macros::u8] = $name.as_bytes();

        /// Similar to NAME_DATA in `__statics_sel!`.
        #[link_section = "__TEXT,__cstring,cstring_literals"]
        #[export_name = $crate::__macros::concat!(
            "\x01L_OBJC_CLASS_NAME_",
            $hash,
        )]
        static NAME_DATA: [$crate::__macros::u8; X.len()] = $crate::__statics_string_to_known_length_bytes!(X);

        /// SAFETY: Same as `REF` in `__statics_sel!`.
        #[link_section = "__OBJC,__cls_refs,literal_pointers"]
        #[export_name = $crate::__macros::concat!(
            "\x01L_OBJC_CLASS_REFERENCES_",
            $hash,
        )]
        static REF: $crate::__macros::SyncUnsafeCell<&$crate::runtime::AnyClass> = unsafe {
            let ptr: *const $crate::runtime::AnyClass = NAME_DATA.as_ptr().cast();
            $crate::__macros::SyncUnsafeCell::new(&*ptr)
        };

        $crate::__statics_image_info!($hash);
        $crate::__statics_module_info!($hash);
    }
}

#[doc(hidden)]
#[macro_export]
#[cfg(all(
    feature = "unstable-static-class",
    not(feature = "unstable-darwin-objc"),
    not(feature = "gnustep-1-7"),
    not(feature = "unstable-static-class-inlined")
))]
macro_rules! __class_inner {
    ($name:expr, $hash:expr) => {{
        $crate::__statics_class! {
            ($name)
            ($hash)
        }

        #[inline(never)]
        fn objc_static_workaround() -> &'static $crate::runtime::AnyClass {
            // SAFETY: Same as __sel_inner
            unsafe { *REF.get() }
        }

        objc_static_workaround()
    }};
}

#[doc(hidden)]
#[macro_export]
#[cfg(all(
    feature = "unstable-static-class",
    not(feature = "unstable-darwin-objc"),
    not(feature = "gnustep-1-7"),
    feature = "unstable-static-class-inlined"
))]
macro_rules! __class_inner {
    ($name:expr, $hash:expr) => {{
        $crate::__statics_class! {
            ($name)
            ($hash)
        }

        #[allow(unused_unsafe)]
        // SAFETY: See above
        unsafe {
            *REF.get()
        }
    }};
}

#[doc(hidden)]
#[macro_export]
// The linking changed in libobjc2 v2.0
#[cfg(all(feature = "gnustep-1-7", not(feature = "gnustep-2-0"),))]
macro_rules! __class_static_name {
    ($name:expr) => {
        $crate::__macros::concat!("_OBJC_CLASS_", $name)
    };
}

#[doc(hidden)]
#[macro_export]
#[cfg(feature = "gnustep-2-0")]
macro_rules! __class_static_name {
    ($name:expr) => {
        $crate::__macros::concat!("._OBJC_CLASS_", $name)
    };
}

#[doc(hidden)]
#[macro_export]
#[cfg(all(feature = "unstable-static-class", feature = "gnustep-1-7"))]
macro_rules! __class_inner {
    ($name:expr, $_hash:expr) => {{
        // NOTE: This is not verified for correctness in any way whatsoever.
        extern "C" {
            #[link_name = $crate::__class_static_name!($name)]
            static CLASS: $crate::runtime::AnyClass;

            // Others:
            // __objc_class_name_
            // _OBJC_CLASS_REF_
        }

        #[allow(unused_unsafe)]
        unsafe {
            $crate::__macros::disallow_in_static(&CLASS)
        }
    }};
}

/// Allows storing a [`AnyClass`] reference in a static and lazily loading it.
#[derive(Debug)]
pub struct CachedClass {
    ptr: AtomicPtr<AnyClass>,
}

impl CachedClass {
    /// Constructs a new [`CachedClass`].
    #[allow(clippy::new_without_default)]
    pub const fn new() -> CachedClass {
        CachedClass {
            ptr: AtomicPtr::new(ptr::null_mut()),
        }
    }

    // Mark as cold since this should only ever be called once (or maybe twice
    // if running on multiple threads).
    #[cold]
    #[track_caller]
    unsafe fn fetch(&self, name: *const c_char) -> &'static AnyClass {
        let ptr: *const AnyClass = unsafe { ffi::objc_getClass(name) }.cast();
        self.ptr.store(ptr as *mut AnyClass, Ordering::Relaxed);
        if let Some(cls) = unsafe { ptr.as_ref() } {
            cls
        } else {
            // Recover the name from the pointer. We do it like this so that
            // we don't have to pass the length of the class to this method,
            // improving binary size.
            let name = unsafe { CStr::from_ptr(name) };
            let name = str::from_utf8(name.to_bytes()).unwrap();
            panic!("class {name} could not be found")
        }
    }

    /// Returns the cached class. If no class is yet cached, gets one with
    /// the given name and stores it.
    #[inline]
    #[track_caller]
    pub unsafe fn get(&self, name: &str) -> &'static AnyClass {
        // `Relaxed` should be fine since `objc_getClass` is thread-safe.
        let ptr = self.ptr.load(Ordering::Relaxed);
        if let Some(cls) = unsafe { ptr.as_ref() } {
            cls
        } else {
            // SAFETY: Checked by caller
            unsafe { self.fetch(name.as_ptr().cast()) }
        }
    }
}

/// Disallow using this passed in value in const and statics for forwards
/// compatibility (this function is not a `const` function).
#[inline]
pub fn disallow_in_static<T>(item: &'static T) -> &'static T {
    item
}

#[cfg(test)]
mod tests {
    #[test]
    #[should_panic = "class NonExistentClass could not be found"]
    #[cfg(not(feature = "unstable-static-class"))]
    fn test_not_found() {
        let _ = crate::class!(NonExistentClass);
    }
}
