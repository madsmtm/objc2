use alloc::borrow::ToOwned;
use core::fmt;
use core::hash;
use core::marker::PhantomData;
use core::ptr::NonNull;

use crate::encode::{Encoding, RefEncode};
use crate::rc::{autoreleasepool, Id, Ownership};
use crate::runtime::__nsstring::nsstring_to_str;
use crate::runtime::{NSObjectProtocol, Object, Protocol};
use crate::Message;

/// Marks types that represent specific protocols.
///
/// This is the protocol equivalent of [`ClassType`].
///
/// This is implemented automatically by the [`extern_protocol!`] macro.
///
/// [`ClassType`]: crate::ClassType
/// [`extern_protocol!`]: crate::extern_protocol
///
///
/// # Safety
///
/// This is meant to be a sealed trait, and should not be implemented outside
/// of the [`extern_protocol!`] macro.
///
///
/// # Examples
///
/// Use the trait to access the [`Protocol`] of different objects.
///
/// ```
/// use objc2::ProtocolType;
/// use objc2::runtime::NSObject;
/// // Get a protocol object representing `NSObject`
/// let protocol = NSObject::protocol();
/// ```
///
/// Use the [`extern_protocol!`] macro to implement this trait for a type.
///
/// ```no_run
/// use objc2::{extern_protocol, ProtocolType};
///
/// extern_protocol!(
///     unsafe trait MyProtocol {
///         #[method(aMethod)]
///         fn a_method(&self);
///     }
///
///     unsafe impl ProtocolType for dyn MyProtocol {}
/// );
///
/// let protocol = <dyn MyProtocol>::protocol();
/// ```
pub unsafe trait ProtocolType {
    /// The name of the Objective-C protocol that this type represents.
    const NAME: &'static str;

    /// Get a reference to the Objective-C protocol object that this type
    /// represents.
    ///
    /// May register the protocol with the runtime if it wasn't already.
    ///
    /// Note that some protocols [are not registered with the runtime][p-obj],
    /// depending on various factors. In those cases, this function may return
    /// `None`.
    ///
    /// [p-obj]: https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/ObjectiveC/Chapters/ocProtocols.html#//apple_ref/doc/uid/TP30001163-CH15-TPXREF149
    ///
    ///
    /// # Panics
    ///
    /// This may panic if something went wrong with getting or declaring the
    /// protocol, e.g. if the program is not properly linked to the framework
    /// that defines the protocol.
    fn protocol() -> Option<&'static Protocol> {
        Protocol::get(Self::NAME)
    }

    #[doc(hidden)]
    const __INNER: ();
}

/// An internal helper trait for [`ProtocolObject`].
///
///
/// # Safety
///
/// This is meant to be a sealed trait, and should not be implemented outside
/// of the [`extern_protocol!`] macro.
///
/// [`extern_protocol!`]: crate::extern_protocol
pub unsafe trait ImplementedBy<P: ?Sized + Message> {
    #[doc(hidden)]
    const __INNER: ();
}

// SAFETY: Trivial
// unsafe impl<P: ?Sized + ProtocolType> ConformsTo<P> for ProtocolObject<P> {
//     fn as_protocol(&self) -> &ProtocolObject<P> {
//         self
//     }
//
//     fn as_protocol_mut(&mut self) -> &mut ProtocolObject<P> {
//         self
//     }
// }

/// TODO
#[repr(C)]
pub struct ProtocolObject<P: ?Sized + ProtocolType> {
    inner: Object,
    p: PhantomData<P>,
}

// SAFETY: The type is `#[repr(C)]` and `Object` internally
unsafe impl<P: ?Sized + ProtocolType> RefEncode for ProtocolObject<P> {
    const ENCODING_REF: Encoding = Encoding::Object;
}

// SAFETY: The type is `Object` internally, and is mean to be messaged as-if
// it's an object.
unsafe impl<P: ?Sized + ProtocolType> Message for ProtocolObject<P> {}

impl<P: ?Sized + ProtocolType> ProtocolObject<P> {
    /// Get an immutable type-erased reference from a type implementing a
    /// protocol.
    #[inline]
    pub fn from_ref<T: Message>(obj: &T) -> &Self
    where
        P: ImplementedBy<T>,
    {
        let ptr: NonNull<T> = NonNull::from(obj);
        let ptr: NonNull<Self> = ptr.cast();
        // SAFETY: Implementer ensures that the object conforms to the
        // protocol; so converting the reference here is safe.
        unsafe { ptr.as_ref() }
    }

    /// Get a mutable type-erased reference from a type implementing a
    /// protocol.
    #[inline]
    pub fn from_mut<T: Message>(obj: &mut T) -> &mut Self
    where
        P: ImplementedBy<T>,
    {
        let ptr: NonNull<T> = NonNull::from(obj);
        let mut ptr: NonNull<Self> = ptr.cast();
        // SAFETY: Same as `as_protocol`.
        //
        // Since the reference came from a mutable reference to start with,
        // returning a mutable reference here is safe (the lifetime of the
        // returned reference is bound to the input).
        unsafe { ptr.as_mut() }
    }

    /// Get a type-erased object from a type implementing a protocol.
    #[inline]
    pub fn from_id<T: Message, O: Ownership>(obj: Id<T, O>) -> Id<Self, O>
    where
        P: ImplementedBy<T> + 'static,
        T: 'static,
    {
        // SAFETY:
        // - The type can be represented as the casted-to type, since
        //   `T: ConformsTo` guarantees that it implements the protocol.
        // - Both types are `'static` (this could maybe be relaxed a bit, but
        //   let's just be on the safe side)!
        unsafe { Id::cast::<Self>(obj) }
    }
}

impl<P: ?Sized + ProtocolType + NSObjectProtocol> PartialEq for ProtocolObject<P> {
    #[inline]
    #[doc(alias = "isEqual:")]
    fn eq(&self, other: &Self) -> bool {
        self.__isEqual(other)
    }
}

impl<P: ?Sized + ProtocolType + NSObjectProtocol> Eq for ProtocolObject<P> {}

impl<P: ?Sized + ProtocolType + NSObjectProtocol> hash::Hash for ProtocolObject<P> {
    #[inline]
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.__hash().hash(state);
    }
}

impl<P: ?Sized + ProtocolType + NSObjectProtocol> fmt::Debug for ProtocolObject<P> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let description = self.__description();

        match description {
            // Attempt to format description string
            Some(description) => {
                let s = autoreleasepool(|pool| {
                    // SAFETY: `description` selector is guaranteed to always
                    // return an instance of `NSString`.
                    let s = unsafe { nsstring_to_str(&description, pool) };
                    // The call to `to_owned` is unfortunate, but is required
                    // to work around `f` not being `AutoreleaseSafe`.
                    // TODO: Fix this!
                    s.to_owned()
                });
                fmt::Display::fmt(&s, f)
            }
            // If description was `NULL`, use `Object`'s `Debug` impl instead
            None => {
                let obj: &Object = &self.inner;
                fmt::Debug::fmt(obj, f)
            }
        }
    }
}

impl<P, T> AsRef<ProtocolObject<T>> for ProtocolObject<P>
where
    P: ?Sized + ProtocolType,
    T: ?Sized + ProtocolType + ImplementedBy<ProtocolObject<P>>,
{
    #[inline]
    fn as_ref(&self) -> &ProtocolObject<T> {
        ProtocolObject::from_ref(self)
    }
}

impl<P, T> AsMut<ProtocolObject<T>> for ProtocolObject<P>
where
    P: ?Sized + ProtocolType,
    T: ?Sized + ProtocolType + ImplementedBy<ProtocolObject<P>>,
{
    #[inline]
    fn as_mut(&mut self) -> &mut ProtocolObject<T> {
        ProtocolObject::from_mut(self)
    }
}

// TODO: Maybe iplement Borrow + BorrowMut?

#[cfg(test)]
#[allow(clippy::missing_safety_doc)]
mod tests {
    use alloc::format;

    use super::*;
    use crate::rc::Owned;
    use crate::runtime::{NSObject, NSObjectProtocol};
    use crate::{declare_class, extern_methods, extern_protocol, ClassType};

    extern_protocol!(
        unsafe trait Foo {
            #[method(foo)]
            fn foo_class();

            #[method(foo)]
            fn foo_instance(&self);
        }

        unsafe impl ProtocolType for dyn Foo {}
    );

    extern_protocol!(
        unsafe trait Bar: NSObjectProtocol {
            #[method(bar)]
            fn bar_class();

            #[method(bar)]
            fn bar_instance(&self);
        }

        unsafe impl ProtocolType for dyn Bar {}
    );

    extern_protocol!(
        unsafe trait FooBar: Foo + Bar {
            #[method(foobar)]
            fn foobar_class();

            #[method(foobar)]
            fn foobar_instance(&self);
        }

        unsafe impl ProtocolType for dyn FooBar {}
    );

    extern_protocol!(
        unsafe trait FooFooBar: Foo + FooBar {
            #[method(foofoobar)]
            fn foofoobar_class();

            #[method(foofoobar)]
            fn foofoobar_instance(&self);
        }

        unsafe impl ProtocolType for dyn FooFooBar {}
    );

    declare_class!(
        #[derive(Debug, PartialEq, Eq, Hash)]
        struct DummyClass;

        unsafe impl ClassType for DummyClass {
            type Super = NSObject;
            const NAME: &'static str = "ProtocolTestsDummyClass";
        }
    );

    extern_methods!(
        unsafe impl DummyClass {
            #[method_id(new)]
            fn new() -> Id<Self, Owned>;
        }
    );

    unsafe impl NSObjectProtocol for DummyClass {}
    unsafe impl Foo for DummyClass {}
    unsafe impl Bar for DummyClass {}
    unsafe impl FooBar for DummyClass {}
    // unsafe impl FooFooBar for DummyClass {}

    #[test]
    /// The out-commented ones here are tested in `test-ui/ui/protocol.rs`
    fn impl_traits() {
        fn impl_nsobject<T: NSObjectProtocol>() {}
        fn impl_foo<T: Foo>() {}
        fn impl_bar<T: Bar>() {}
        fn impl_foobar<T: FooBar>() {}
        fn impl_foofoobar<T: FooFooBar>() {}

        impl_nsobject::<NSObject>();
        impl_nsobject::<ProtocolObject<NSObject>>();
        // impl_nsobject::<ProtocolObject<dyn Foo>>();
        impl_nsobject::<ProtocolObject<dyn Bar>>();
        impl_nsobject::<ProtocolObject<dyn FooBar>>();
        impl_nsobject::<ProtocolObject<dyn FooFooBar>>();
        impl_nsobject::<DummyClass>();

        impl_foo::<ProtocolObject<dyn Foo>>();
        // impl_foo::<ProtocolObject<dyn Bar>>();
        impl_foo::<ProtocolObject<dyn FooBar>>();
        impl_foo::<ProtocolObject<dyn FooFooBar>>();
        impl_foo::<DummyClass>();

        // impl_bar::<ProtocolObject<dyn Foo>>();
        impl_bar::<ProtocolObject<dyn Bar>>();
        impl_bar::<ProtocolObject<dyn FooBar>>();
        impl_bar::<ProtocolObject<dyn FooFooBar>>();
        impl_bar::<DummyClass>();

        // impl_foobar::<ProtocolObject<dyn Foo>>();
        // impl_foobar::<ProtocolObject<dyn Bar>>();
        impl_foobar::<ProtocolObject<dyn FooBar>>();
        impl_foobar::<ProtocolObject<dyn FooFooBar>>();
        impl_foobar::<DummyClass>();

        // impl_foofoobar::<ProtocolObject<dyn Foo>>();
        // impl_foofoobar::<ProtocolObject<dyn Bar>>();
        // impl_foofoobar::<ProtocolObject<dyn FooBar>>();
        impl_foofoobar::<ProtocolObject<dyn FooFooBar>>();
        // impl_foofoobar::<DummyClass>();
    }

    #[test]
    fn convertible() {
        let mut obj = DummyClass::new();
        let foobar: &ProtocolObject<dyn FooBar> = ProtocolObject::from_ref(&*obj);
        let foobar: &ProtocolObject<dyn FooBar> = ProtocolObject::from_ref(foobar);

        let _bar: &ProtocolObject<dyn Bar> = ProtocolObject::from_ref(foobar);
        let bar: &ProtocolObject<dyn Bar> = ProtocolObject::from_ref(&*obj);
        let bar: &ProtocolObject<dyn Bar> = ProtocolObject::from_ref(bar);

        let _foo: &ProtocolObject<dyn Foo> = ProtocolObject::from_ref(foobar);
        let foo: &ProtocolObject<dyn Foo> = ProtocolObject::from_ref(&*obj);
        let _foo: &ProtocolObject<dyn Foo> = ProtocolObject::from_ref(foo);

        let _nsobject: &ProtocolObject<NSObject> = ProtocolObject::from_ref(foobar);
        let _nsobject: &ProtocolObject<NSObject> = ProtocolObject::from_ref(bar);
        let nsobject: &ProtocolObject<NSObject> = ProtocolObject::from_ref(&*obj);
        let _nsobject: &ProtocolObject<NSObject> = ProtocolObject::from_ref(nsobject);

        let _foobar: &mut ProtocolObject<dyn FooBar> = ProtocolObject::from_mut(&mut *obj);
        let _foobar: Id<ProtocolObject<dyn FooBar>, _> = ProtocolObject::from_id(obj);
    }

    #[test]
    fn test_traits() {
        use core::hash::Hasher;
        use std::collections::hash_map::DefaultHasher;
        use std::hash::Hash;

        let obj = DummyClass::new();
        let obj2 = DummyClass::new();

        let foobar: &ProtocolObject<dyn FooBar> = ProtocolObject::from_ref(&*obj);
        let foobar2: &ProtocolObject<dyn FooBar> = ProtocolObject::from_ref(&*obj2);

        assert_eq!(
            format!("{obj:?}"),
            format!("DummyClass {{ __inner: {foobar:?} }}")
        );
        assert_eq!(obj == obj2, foobar == foobar2);

        let mut hashstate_a = DefaultHasher::new();
        let mut hashstate_b = DefaultHasher::new();

        obj.hash(&mut hashstate_a);
        foobar.hash(&mut hashstate_b);

        assert_eq!(hashstate_a.finish(), hashstate_b.finish());
    }
}
