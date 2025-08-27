use core::ffi::CStr;
use core::fmt;
use core::hash;
use core::panic::{RefUnwindSafe, UnwindSafe};
use core::ptr::{self, NonNull};

use crate::encode::{Encode, Encoding};
use crate::ffi;

use super::AnyObject;

/// An opaque type that represents an instance variable.
///
/// See [Apple's documentation](https://developer.apple.com/documentation/objectivec/ivar?language=objc).
#[repr(C)]
#[doc(alias = "objc_ivar")]
pub struct Ivar {
    _priv: [u8; 0],
    _p: ffi::OpaqueData,
}

// SAFETY: Ivar is immutable (and can be retrieved from AnyClass anyhow).
unsafe impl Sync for Ivar {}
unsafe impl Send for Ivar {}
impl UnwindSafe for Ivar {}
impl RefUnwindSafe for Ivar {}

impl Ivar {
    /// Returns the instance variable's name.
    ///
    /// See [Apple's documentation](https://developer.apple.com/documentation/objectivec/1418922-ivar_getname?language=objc).
    #[inline]
    #[doc(alias = "ivar_getName")]
    pub fn name(&self) -> &CStr {
        unsafe { CStr::from_ptr(ffi::ivar_getName(self)) }
    }

    /// Returns the instance variable's offset from the object base.
    ///
    /// See [Apple's documentation](https://developer.apple.com/documentation/objectivec/1418976-ivar_getoffset?language=objc).
    #[inline]
    #[doc(alias = "ivar_getOffset")]
    pub fn offset(&self) -> isize {
        unsafe { ffi::ivar_getOffset(self) }
    }

    /// Returns the instance variable's `@encode(type)` string.
    ///
    /// See [Apple's documentation](https://developer.apple.com/documentation/objectivec/1418569-ivar_gettypeencoding?language=objc).
    #[inline]
    #[doc(alias = "ivar_getTypeEncoding")]
    pub fn type_encoding(&self) -> &CStr {
        unsafe { CStr::from_ptr(ffi::ivar_getTypeEncoding(self)) }
    }

    #[inline]
    pub(crate) fn debug_assert_encoding(&self, _expected: &Encoding) {
        #[cfg(all(debug_assertions, not(feature = "disable-encoding-assertions")))]
        {
            let encoding = self.type_encoding();
            let encoding = encoding.to_str().expect("encoding must be UTF-8");
            assert!(
                _expected.equivalent_to_str(encoding),
                "wrong encoding. Tried to retrieve ivar with encoding {encoding}, but the encoding of the given type was {_expected}",
            );
        }
    }

    /// Returns a pointer to the instance variable / ivar on the given object.
    ///
    /// This is similar to [`UnsafeCell::get`], see that for more information
    /// on what is and isn't safe to do.
    ///
    /// Usually you will have defined the instance variable yourself with
    /// [`ClassBuilder::add_ivar`], the type of the ivar `T` must match the
    /// type used in that.
    ///
    /// Library implementors are strongly encouraged to expose a safe
    /// interface to the ivar.
    ///
    /// [`UnsafeCell::get`]: core::cell::UnsafeCell::get
    /// [`ClassBuilder::add_ivar`]: crate::runtime::ClassBuilder::add_ivar
    ///
    ///
    /// # Panics
    ///
    /// Panics when `debug_assertions` are enabled if the type encoding of the
    /// ivar differs from the type encoding of `T`. This can be disabled with
    /// the `"disable-encoding-assertions"` Cargo feature flag.
    ///
    ///
    /// # Safety
    ///
    /// The object must have the given instance variable on it, and it must be
    /// of type `T`. Any invariants that the object have assumed about the
    /// value of the instance variable must not be violated.
    ///
    /// Note that an object can have multiple instance variables with the same
    /// name; you must ensure that when the instance variable was retrieved,
    /// was retrieved from the class that it was defined on. In particular,
    /// getting a class dynamically using e.g. [`AnyObject::class`], and using
    /// an instance variable from that here is _not_ sound in general.
    ///
    /// No thread synchronization is done on accesses to the variable, so you
    /// must ensure that any access to the returned pointer do not cause data
    /// races, and that Rust's mutability rules are not otherwise violated.
    #[inline]
    pub unsafe fn load_ptr<T: Encode>(&self, obj: &AnyObject) -> *mut T {
        self.debug_assert_encoding(&T::ENCODING);

        let ptr = NonNull::from(obj);
        // SAFETY: That the ivar is valid is ensured by the caller
        let ptr = unsafe { AnyObject::ivar_at_offset::<T>(ptr, self.offset()) };

        // Safe as *mut T because `self` is `UnsafeCell`
        ptr.as_ptr()
    }

    /// Returns a reference to the instance variable with the given name.
    ///
    /// See [`Ivar::load_ptr`] for more information.
    ///
    ///
    /// # Panics
    ///
    /// Panics when `debug_assertions` are enabled if the type encoding of the
    /// ivar differs from the type encoding of `T`. This can be disabled with
    /// the `"disable-encoding-assertions"` Cargo feature flag.
    ///
    ///
    /// # Safety
    ///
    /// The object must have the given instance variable on it, and it must be
    /// of type `T`.
    ///
    /// No thread synchronization is done, so you must ensure that no other
    /// thread is concurrently mutating the variable. This requirement can be
    /// considered upheld if all mutation happens through [`Ivar::load_mut`]
    /// (since that takes the object mutably).
    #[inline]
    pub unsafe fn load<'obj, T: Encode>(&self, obj: &'obj AnyObject) -> &'obj T {
        // SAFETY: That the ivar is valid as `&T` is ensured by the caller,
        // and the reference is properly bound to the object.
        unsafe { self.load_ptr::<T>(obj).as_ref().unwrap_unchecked() }
    }

    /// Returns a mutable reference to the ivar with the given name.
    ///
    /// See [`Ivar::load_ptr`] for more information.
    ///
    ///
    /// # Panics
    ///
    /// Panics when `debug_assertions` are enabled if the type encoding of the
    /// ivar differs from the type encoding of `T`. This can be disabled with
    /// the `"disable-encoding-assertions"` Cargo feature flag.
    ///
    ///
    /// # Safety
    ///
    /// The object must have an instance variable with the given name, and it
    /// must be of type `T`.
    ///
    /// This access happens through `&mut`, which means we know it to be the
    /// only reference, hence you do not need to do any work to ensure that
    /// data races do not happen.
    #[inline]
    pub unsafe fn load_mut<'obj, T: Encode>(&self, obj: &'obj mut AnyObject) -> &'obj mut T {
        self.debug_assert_encoding(&T::ENCODING);

        let ptr = NonNull::from(obj);
        // SAFETY: That the ivar is valid is ensured by the caller
        let mut ptr = unsafe { AnyObject::ivar_at_offset::<T>(ptr, self.offset()) };

        // SAFETY: That the ivar is valid as `&mut T` is ensured by taking an
        // `&mut` object
        unsafe { ptr.as_mut() }
    }
}

standard_pointer_impls!(Ivar);

impl fmt::Debug for Ivar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Ivar")
            .field("name", &self.name())
            .field("offset", &self.offset())
            .field("type_encoding", &self.type_encoding())
            .finish_non_exhaustive()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::class;
    use crate::runtime::{test_utils, AnyClass};

    fn get_ivar_layout(cls: &AnyClass) -> *const u8 {
        unsafe { ffi::class_getIvarLayout(cls) }
    }

    #[test]
    #[cfg_attr(
        feature = "gnustep-1-7",
        ignore = "ivar layout is still used on GNUStep"
    )]
    fn test_layout_does_not_matter_any_longer() {
        assert!(get_ivar_layout(class!(NSObject)).is_null());
        assert!(get_ivar_layout(class!(NSArray)).is_null());
        assert!(get_ivar_layout(class!(NSException)).is_null());
        assert!(get_ivar_layout(class!(NSNumber)).is_null());
        assert!(get_ivar_layout(class!(NSString)).is_null());
    }

    #[test]
    #[cfg_attr(
        all(debug_assertions, not(feature = "disable-encoding-assertions")),
        should_panic = "wrong encoding. Tried to retrieve ivar with encoding I, but the encoding of the given type was C"
    )]
    fn test_object_ivar_wrong_type() {
        let obj = test_utils::custom_object();
        let cls = test_utils::custom_class();
        let ivar = cls
            .instance_variable(CStr::from_bytes_with_nul(b"_foo\0").unwrap())
            .unwrap();
        let _ = unsafe { *ivar.load::<u8>(&obj) };
    }
}
