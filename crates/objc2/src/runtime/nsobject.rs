use core::fmt;
use core::hash;

use crate::mutability::Root;
use crate::rc::{DefaultId, Id};
use crate::runtime::{AnyClass, AnyObject, AnyProtocol, ImplementedBy, ProtocolObject};
use crate::{extern_methods, msg_send, msg_send_id, Message};
use crate::{ClassType, ProtocolType};

crate::__emit_struct! {
    (
        /// The root class of most Objective-C class hierarchies.
        ///
        /// This represents the [`NSObject` class][cls]. The name "NSObject" also
        /// refers to a protocol, see [`NSObjectProtocol`] for that.
        ///
        /// Since this class is only available with the `Foundation` framework,
        /// `objc2` links to it for you.
        ///
        /// This is exported under `icrate::Foundation::NSObject`, you probably
        /// want to use that path instead.
        ///
        /// [cls]: https://developer.apple.com/documentation/objectivec/nsobject?language=objc
    )
    (pub)
    (NSObject)
    (
        __inner: AnyObject,
    )
}

crate::__extern_class_impl_traits! {
    unsafe impl () for NSObject {
        INHERITS = [AnyObject];

        fn as_super(&self) {
            &self.__inner
        }

        fn as_super_mut(&mut self) {
            &mut self.__inner
        }
    }
}

unsafe impl ClassType for NSObject {
    type Super = AnyObject;
    type Mutability = Root;
    const NAME: &'static str = "NSObject";

    #[inline]
    fn class() -> &'static AnyClass {
        #[cfg(feature = "apple")]
        {
            crate::__class_inner!("NSObject", "NSObject")
        }
        #[cfg(feature = "gnustep-1-7")]
        {
            extern "C" {
                // The linking changed in libobjc2 v2.0
                #[cfg_attr(feature = "gnustep-2-0", link_name = "._OBJC_CLASS_NSObject")]
                #[cfg_attr(not(feature = "gnustep-2-0"), link_name = "_OBJC_CLASS_NSObject")]
                static OBJC_CLASS_NSObject: AnyClass;
                // Others:
                // __objc_class_name_NSObject
                // _OBJC_CLASS_REF_NSObject
            }

            unsafe { &OBJC_CLASS_NSObject }
        }
    }

    #[inline]
    fn as_super(&self) -> &Self::Super {
        &self.__inner
    }

    #[inline]
    fn as_super_mut(&mut self) -> &mut Self::Super {
        &mut self.__inner
    }
}

/// The methods that are fundamental to most Objective-C objects.
///
/// This represents the [`NSObject` protocol][proto].
///
/// You should rarely need to use this for anything other than as a trait
/// bound in [`extern_protocol!`], to allow your protocol to implement `Debug`
/// `Hash`, `PartialEq` and `Eq`.
///
/// This trait is exported under `icrate::Foundation::NSObjectProtocol`, you
/// probably want to use that path instead.
///
/// [proto]: https://developer.apple.com/documentation/objectivec/1418956-nsobject?language=objc
/// [`extern_protocol!`]: crate::extern_protocol!
///
///
/// # Safety
///
/// Like with [other protocols](ProtocolType), the type must represent a class
/// that implements the `NSObject` protocol.
#[allow(non_snake_case)]
pub unsafe trait NSObjectProtocol {
    #[doc(hidden)]
    fn __isEqual(&self, other: &Self) -> bool
    where
        Self: Sized + Message,
    {
        unsafe { msg_send![self, isEqual: other] }
    }

    #[doc(hidden)]
    fn __hash(&self) -> usize
    where
        Self: Sized + Message,
    {
        unsafe { msg_send![self, hash] }
    }

    #[doc(hidden)]
    fn __description(&self) -> Option<Id<NSObject>>
    where
        Self: Sized + Message,
    {
        unsafe { msg_send_id![self, description] }
    }

    #[doc(hidden)]
    fn __isKindOfClass(&self, cls: &AnyClass) -> bool
    where
        Self: Sized + Message,
    {
        unsafe { msg_send![self, isKindOfClass: cls] }
    }

    /// Check if the object is an instance of the class, or one of it's
    /// subclasses.
    ///
    /// See [Apple's documentation][apple-doc] for more details on what you
    /// may (and what you may not) do with this information.
    ///
    /// [apple-doc]: https://developer.apple.com/documentation/objectivec/1418956-nsobject/1418511-iskindofclass
    #[doc(alias = "isKindOfClass")]
    fn is_kind_of<T: ClassType>(&self) -> bool
    where
        Self: Sized + Message,
    {
        self.__isKindOfClass(T::class())
    }

    // Note: We don't provide a method to convert `NSObject` to `T` based on
    // `is_kind_of`, since that is not possible to do in general!
    //
    // For example, something may have a return type of `NSString`, while
    // behind the scenes they really return `NSMutableString` and expect it to
    // not be modified.
}

crate::__inner_extern_protocol!(
    ()
    (NSObjectProtocol)
    (dyn NSObjectProtocol)
    ("NSCopying")
);

unsafe impl NSObjectProtocol for NSObject {}

unsafe impl ProtocolType for NSObject {
    const NAME: &'static str = "NSObject";

    fn protocol() -> Option<&'static AnyProtocol> {
        Some(
            AnyProtocol::get(<Self as ProtocolType>::NAME)
                .expect("could not find NSObject protocol"),
        )
    }

    const __INNER: () = ();
}

unsafe impl<T> ImplementedBy<T> for NSObject
where
    T: ?Sized + Message + NSObjectProtocol,
{
    const __INNER: () = ();
}

extern_methods!(
    unsafe impl NSObject {
        /// Create a new empty `NSObject`.
        #[method_id(new)]
        pub fn new() -> Id<Self>;
    }
);

/// Objective-C equality has approximately the same semantics as Rust
/// equality (although less aptly specified).
///
/// At the very least, equality is _expected_ to be symmetric and
/// transitive, and that's about the best we can do.
///
/// See also <https://nshipster.com/equality/>
impl PartialEq for NSObject {
    #[inline]
    #[doc(alias = "isEqual:")]
    fn eq(&self, other: &Self) -> bool {
        self.__isEqual(other)
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
        self.__hash().hash(state);
    }
}

impl fmt::Debug for NSObject {
    #[inline]
    #[doc(alias = "description")]
    #[doc(alias = "debugDescription")]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let obj: &ProtocolObject<NSObject> = ProtocolObject::from_ref(self);
        obj.fmt(f)
    }
}

impl DefaultId for NSObject {
    #[inline]
    fn default_id() -> Id<Self> {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::format;

    use crate::extern_class;
    use crate::mutability::Mutable;
    use crate::rc::__RcTestObject;

    extern_class!(
        #[derive(Debug, PartialEq, Eq, Hash)]
        struct NSObjectMutable;

        unsafe impl ClassType for NSObjectMutable {
            type Super = NSObject;
            type Mutability = Mutable;
            const NAME: &'static str = "NSObject";
        }
    );

    impl NSObjectMutable {
        fn new() -> Id<Self> {
            unsafe { Id::cast(NSObject::new()) }
        }
    }

    #[test]
    fn test_deref() {
        let obj: Id<NSObject> = NSObject::new();
        let _: &NSObject = &obj;
        let _: &AnyObject = &obj;
    }

    #[test]
    fn test_deref_mut() {
        let mut obj: Id<NSObjectMutable> = NSObjectMutable::new();
        let _: &NSObjectMutable = &obj;
        let _: &mut NSObjectMutable = &mut obj;
        let _: &NSObject = &obj;
        let _: &mut NSObject = &mut obj;
        let _: &AnyObject = &obj;
        let _: &mut AnyObject = &mut obj;
    }

    #[test]
    fn test_as_ref_borrow() {
        use core::borrow::{Borrow, BorrowMut};

        fn impls_as_ref<T: AsRef<U> + Borrow<U> + ?Sized, U: ?Sized>(_: &T) {}
        fn impls_as_mut<T: AsMut<U> + BorrowMut<U> + ?Sized, U: ?Sized>(_: &mut T) {}

        let mut obj = NSObjectMutable::new();
        impls_as_ref::<Id<NSObjectMutable>, NSObjectMutable>(&obj);
        impls_as_mut::<Id<NSObjectMutable>, NSObjectMutable>(&mut obj);
        impls_as_ref::<NSObjectMutable, NSObjectMutable>(&obj);
        impls_as_mut::<NSObjectMutable, NSObjectMutable>(&mut obj);
        impls_as_ref::<NSObject, NSObject>(&obj);
        impls_as_mut::<NSObject, NSObject>(&mut obj);
        impls_as_ref::<NSObject, AnyObject>(&obj);
        impls_as_mut::<NSObject, AnyObject>(&mut obj);

        let obj = NSObject::new();
        impls_as_ref::<Id<NSObject>, NSObject>(&obj);
        impls_as_ref::<NSObject, NSObject>(&obj);
        impls_as_ref::<NSObject, AnyObject>(&obj);
    }

    #[test]
    fn test_equality() {
        let obj1 = NSObject::new();
        assert_eq!(obj1, obj1);

        let obj2 = NSObject::new();
        assert_ne!(obj1, obj2);
    }

    #[test]
    fn test_hash() {
        use core::hash::Hasher;
        use std::collections::hash_map::DefaultHasher;
        use std::hash::Hash;

        let obj1 = NSObject::new();

        let mut hashstate1 = DefaultHasher::new();
        let mut hashstate2 = DefaultHasher::new();

        obj1.hash(&mut hashstate1);
        obj1.hash(&mut hashstate2);

        assert_eq!(hashstate1.finish(), hashstate2.finish());

        let obj2 = NSObject::new();
        let mut hashstate2 = DefaultHasher::new();
        obj2.hash(&mut hashstate2);
        assert_ne!(hashstate1.finish(), hashstate2.finish());
    }

    #[test]
    fn test_debug() {
        let obj = NSObject::new();
        let expected = format!("<NSObject: {:p}>", &*obj);
        assert_eq!(format!("{obj:?}"), expected);
    }

    #[test]
    fn test_is_kind_of() {
        let obj = NSObject::new();
        assert!(obj.is_kind_of::<NSObject>());
        assert!(!obj.is_kind_of::<__RcTestObject>());

        let obj = __RcTestObject::new();
        assert!(obj.is_kind_of::<NSObject>());
        assert!(obj.is_kind_of::<__RcTestObject>());
    }
}
