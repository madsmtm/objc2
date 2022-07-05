use objc2::rc::{DefaultId, Id, Owned, Shared};
use objc2::runtime::{Class, Object};

use super::ffi;
use super::NSString;

object! {
    @__inner
    unsafe pub struct NSObject<>: Object {}
}

impl NSObject {
    pub fn new() -> Id<Self, Owned> {
        unsafe { Id::cast(ffi::NSObject::new().unwrap()).into_owned() }
    }

    fn r(&self) -> &ffi::NSObject {
        unsafe { &*(self as *const Self as *const ffi::NSObject) }
    }

    pub fn hash_code(&self) -> usize {
        unsafe { self.r().hash() }
    }

    pub fn is_equal(&self, other: &NSObject) -> bool {
        unsafe { self.r().isEqual_(other.r()).as_bool() }
    }

    pub fn description(&self) -> Id<NSString, Shared> {
        // TODO: Verify that description always returns a non-null string
        unsafe { Id::cast(self.r().description().unwrap()).into_shared() }
    }

    pub fn is_kind_of(&self, cls: &Class) -> bool {
        unsafe { self.r().isKindOfClass_(cls).as_bool() }
    }
}

impl DefaultId for NSObject {
    type Ownership = Owned;

    #[inline]
    fn default_id() -> Id<Self, Self::Ownership> {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::format;

    #[test]
    fn test_deref() {
        let mut obj: Id<NSObject, Owned> = NSObject::new();
        let _: &NSObject = &obj;
        let _: &mut NSObject = &mut obj;
        let _: &Object = &obj;
        let _: &mut Object = &mut obj;
    }

    #[test]
    fn test_as_ref_borrow() {
        use core::borrow::{Borrow, BorrowMut};

        fn impls_as_ref<T: AsRef<U> + Borrow<U> + ?Sized, U: ?Sized>(_: &T) {}
        fn impls_as_mut<T: AsMut<U> + BorrowMut<U> + ?Sized, U: ?Sized>(_: &mut T) {}

        let mut obj = NSObject::new();
        impls_as_ref::<Id<NSObject, Owned>, NSObject>(&obj);
        impls_as_mut::<Id<NSObject, Owned>, NSObject>(&mut obj);
        impls_as_ref::<NSObject, NSObject>(&obj);
        impls_as_mut::<NSObject, NSObject>(&mut obj);
        impls_as_ref::<NSObject, Object>(&obj);
        impls_as_mut::<NSObject, Object>(&mut obj);
    }

    #[test]
    fn test_equality() {
        let obj1 = NSObject::new();
        assert_eq!(obj1, obj1);

        let obj2 = NSObject::new();
        assert_ne!(obj1, obj2);
    }

    #[test]
    fn test_hash_code() {
        let obj = NSObject::new();
        assert_eq!(obj.hash_code(), obj.hash_code());
    }

    #[test]
    fn test_debug() {
        let obj = NSObject::new();
        let expected = format!("<NSObject: {:p}>", &*obj);
        assert_eq!(format!("{:?}", obj), format!("{:?}", expected));
    }

    #[test]
    fn test_is_kind_of() {
        let obj = NSObject::new();
        assert!(obj.is_kind_of(NSObject::class()));
        assert!(!obj.is_kind_of(NSString::class()));
    }
}
