use core::fmt;
use core::hash;

use objc2::rc::{DefaultId, Id, Owned, Shared};
use objc2::runtime::{Class, Object};
use objc2::{class, msg_send, msg_send_bool, msg_send_id};

use super::NSString;

__inner_extern_class! {
    @__inner
    unsafe pub struct NSObject<>: Object {}
}

impl NSObject {
    /// Get a reference to the Objective-C class `NSObject`.
    #[inline]
    pub fn class() -> &'static Class {
        class!(NSObject)
    }
}

impl NSObject {
    unsafe_def_fn!(pub fn new -> Owned);

    pub fn hash_code(&self) -> usize {
        unsafe { msg_send![self, hash] }
    }

    pub fn is_equal(&self, other: &NSObject) -> bool {
        unsafe { msg_send_bool![self, isEqual: other] }
    }

    pub fn description(&self) -> Id<NSString, Shared> {
        // TODO: Verify that description always returns a non-null string
        unsafe { msg_send_id![self, description].unwrap() }
    }

    pub fn is_kind_of(&self, cls: &Class) -> bool {
        unsafe { msg_send_bool![self, isKindOfClass: cls] }
    }
}

/// Objective-C equality has approximately the same semantics as Rust
/// equality (although less aptly specified).
///
/// At the very least, equality is _expected_ to be symmetric and
/// transitive, and that's about the best we can do.
///
/// See also <https://nshipster.com/equality/>
impl PartialEq for NSObject {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.is_equal(other)
    }
}

/// Most types' equality is reflexive.
impl Eq for NSObject {}

/// Hashing in Objective-C has the exact same requirement as in Rust:
///
/// > If two objects are equal (as determined by the isEqual: method),
/// > they must have the same hash value.
///
/// See <https://developer.apple.com/documentation/objectivec/1418956-nsobject/1418859-hash>
impl hash::Hash for NSObject {
    #[inline]
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.hash_code().hash(state);
    }
}

impl fmt::Debug for NSObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.description(), f)
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
