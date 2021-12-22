//! A Rust interface for the functionality of the Objective-C runtime.
//!
//! For more information on foreign functions, see Apple's documentation:
//! <https://developer.apple.com/library/mac/documentation/Cocoa/Reference/ObjCRuntimeRef/index.html>

use core::ffi::c_void;
use core::fmt;
use core::panic::{RefUnwindSafe, UnwindSafe};
use core::ptr;
use core::str;
#[cfg(feature = "malloc")]
use malloc_buf::Malloc;
use std::ffi::{CStr, CString};
#[cfg(feature = "malloc")]
use std::os::raw::c_uint;

pub use super::bool::Bool;
use crate::{ffi, Encode, Encoding, RefEncode};

/// Use [`Bool`] or [`ffi::BOOL`] instead.
#[deprecated = "Use `Bool` or `ffi::BOOL` instead"]
#[allow(non_upper_case_globals)]
pub type BOOL = ffi::BOOL;

/// Use [`Bool::YES`] or [`ffi::YES`] instead.
#[deprecated = "Use `Bool::YES` or `ffi::YES` instead"]
pub const YES: ffi::BOOL = ffi::YES;

/// Use [`Bool::NO`] or [`ffi::NO`] instead.
#[deprecated = "Use `Bool::NO` or `ffi::NO` instead"]
pub const NO: ffi::BOOL = ffi::NO;

/// A type that represents a method selector.
#[repr(transparent)]
pub struct Sel {
    ptr: *const ffi::objc_selector,
}

/// A type that represents an instance variable.
#[repr(C)]
pub struct Ivar(ffi::objc_ivar);

/// A type that represents a method in a class definition.
#[repr(C)]
pub struct Method(ffi::objc_method);

/// A type that represents an Objective-C class.
#[repr(C)]
pub struct Class(ffi::objc_class);

/// A type that represents an Objective-C protocol.
#[repr(C)]
pub struct Protocol(ffi::Protocol);

/// A type that represents an instance of a class.
///
/// Note: This is intentionally neither [`Sync`], [`Send`], [`UnwindSafe`],
/// [`RefUnwindSafe`] nor [`Unpin`], since it is something that changes
/// depending on the specific subclass.
///
/// Examples: `NSAutoreleasePool` is not `Send`, it has to be deallocated
/// on the same thread that it was created. `NSLock` is not `Send` either.
#[repr(C)]
pub struct Object(ffi::objc_object);

/// A pointer to the start of a method implementation.
pub type Imp = unsafe extern "C" fn();

impl Sel {
    /// Registers a method with the Objective-C runtime system,
    /// maps the method name to a selector, and returns the selector value.
    pub fn register(name: &str) -> Self {
        let name = CString::new(name).unwrap();
        Self {
            ptr: unsafe { ffi::sel_registerName(name.as_ptr()) },
        }
    }

    /// Returns the name of the method specified by self.
    pub fn name(&self) -> &str {
        let name = unsafe { CStr::from_ptr(ffi::sel_getName(self.ptr)) };
        str::from_utf8(name.to_bytes()).unwrap()
    }

    /// Wraps a raw pointer to a selector into a [`Sel`] object.
    ///
    /// # Safety
    ///
    /// The pointer must a valid, registered selector.
    ///
    /// This is almost never what you want; use [`Sel::register`] instead.
    #[inline]
    pub unsafe fn from_ptr(ptr: *const c_void) -> Self {
        Self {
            ptr: ptr as *const _,
        }
    }

    /// Returns a pointer to the raw selector.
    #[inline]
    pub fn as_ptr(&self) -> *const c_void {
        self.ptr as *const _
    }
}

unsafe impl Encode for Sel {
    const ENCODING: Encoding<'static> = Encoding::Sel;
}

impl PartialEq for Sel {
    fn eq(&self, other: &Sel) -> bool {
        self.ptr == other.ptr
    }
}

impl Eq for Sel {}

// SAFETY: Sel is immutable (and can be retrieved from any thread using the
// `sel!` macro).
unsafe impl Sync for Sel {}
unsafe impl Send for Sel {}
impl UnwindSafe for Sel {}
impl RefUnwindSafe for Sel {}

impl Copy for Sel {}

impl Clone for Sel {
    fn clone(&self) -> Self {
        Self { ptr: self.ptr }
    }
}

impl fmt::Debug for Sel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl Ivar {
    pub(crate) fn as_ptr(&self) -> *const ffi::objc_ivar {
        self as *const Self as *const _
    }

    /// Returns the name of self.
    pub fn name(&self) -> &str {
        let name = unsafe { CStr::from_ptr(ffi::ivar_getName(self.as_ptr())) };
        str::from_utf8(name.to_bytes()).unwrap()
    }

    /// Returns the offset of self.
    pub fn offset(&self) -> isize {
        let offset = unsafe { ffi::ivar_getOffset(self.as_ptr()) };
        offset as isize
    }

    /// Returns the `Encoding` of self.
    pub fn type_encoding(&self) -> &str {
        let encoding = unsafe { CStr::from_ptr(ffi::ivar_getTypeEncoding(self.as_ptr())) };
        str::from_utf8(encoding.to_bytes()).unwrap()
    }
}

// SAFETY: Ivar is immutable (and can be retrieved from Class anyhow).
unsafe impl Sync for Ivar {}
unsafe impl Send for Ivar {}
impl UnwindSafe for Ivar {}
impl RefUnwindSafe for Ivar {}

impl Method {
    pub(crate) fn as_ptr(&self) -> *const ffi::objc_method {
        self as *const Self as *const _
    }

    /// Returns the name of self.
    pub fn name(&self) -> Sel {
        Sel {
            ptr: unsafe { ffi::method_getName(self.as_ptr()) },
        }
    }

    /// Returns the `Encoding` of self's return type.
    #[cfg(feature = "malloc")]
    pub fn return_type(&self) -> Malloc<str> {
        unsafe {
            let encoding = ffi::method_copyReturnType(self.as_ptr());
            Malloc::from_c_str(encoding).unwrap()
        }
    }

    /// Returns the `Encoding` of a single parameter type of self, or
    /// [`None`] if self has no parameter at the given index.
    #[cfg(feature = "malloc")]
    pub fn argument_type(&self, index: usize) -> Option<Malloc<str>> {
        unsafe {
            let encoding = ffi::method_copyArgumentType(self.as_ptr(), index as c_uint);
            if encoding.is_null() {
                None
            } else {
                Some(Malloc::from_c_str(encoding).unwrap())
            }
        }
    }

    // method_getTypeEncoding, efficient version of:
    // -> return_type() + sum(argument_type(i) for i in arguments_count())

    /// Returns the number of arguments accepted by self.
    pub fn arguments_count(&self) -> usize {
        unsafe { ffi::method_getNumberOfArguments(self.as_ptr()) as usize }
    }

    /// Returns the implementation of self.
    pub fn implementation(&self) -> Imp {
        unsafe { ffi::method_getImplementation(self.as_ptr()).expect("Null IMP") }
    }

    // unsafe fn set_implementation(&mut self, imp: Imp) -> Imp;
    // unsafe fn exchange_implementation(&mut self, other: &mut Method);
}

// SAFETY: Method is immutable (and can be retrieved from Class anyhow).
unsafe impl Sync for Method {}
unsafe impl Send for Method {}
impl UnwindSafe for Method {}
impl RefUnwindSafe for Method {}

impl Class {
    pub(crate) fn as_ptr(&self) -> *const ffi::objc_class {
        self as *const Self as *const _
    }

    /// Returns the class definition of a specified class, or [`None`] if the
    /// class is not registered with the Objective-C runtime.
    pub fn get(name: &str) -> Option<&'static Self> {
        let name = CString::new(name).unwrap();
        unsafe {
            let cls = ffi::objc_getClass(name.as_ptr());
            if cls.is_null() {
                None
            } else {
                Some(&*(cls as *const Self))
            }
        }
    }

    // Same as `get`, but ...
    // fn lookup(name: &str) -> Option<&'static Self>;

    /// Obtains the list of registered class definitions.
    #[cfg(feature = "malloc")]
    pub fn classes() -> Malloc<[&'static Self]> {
        unsafe {
            let mut count: c_uint = 0;
            let classes = ffi::objc_copyClassList(&mut count);
            Malloc::from_array(classes as *mut _, count as usize)
        }
    }

    /// Returns the total number of registered classes.
    pub fn classes_count() -> usize {
        unsafe { ffi::objc_getClassList(ptr::null_mut(), 0) as usize }
    }

    /// Returns the name of the class.
    pub fn name(&self) -> &str {
        let name = unsafe { CStr::from_ptr(ffi::class_getName(self.as_ptr())) };
        str::from_utf8(name.to_bytes()).unwrap()
    }

    /// Returns the superclass of self, or [`None`] if self is a root class.
    pub fn superclass(&self) -> Option<&Class> {
        unsafe {
            let superclass = ffi::class_getSuperclass(self.as_ptr());
            if superclass.is_null() {
                None
            } else {
                Some(&*(superclass as *const Self))
            }
        }
    }

    /// Returns the metaclass of self.
    pub fn metaclass(&self) -> &Self {
        unsafe { &*(ffi::object_getClass(self.as_ptr() as *const _) as *const Self) }
    }

    // objc_getMetaClass -> Same as `Class::get(name).metaclass()`

    #[allow(unused)]
    fn is_metaclass(&self) -> bool {
        unsafe { Bool::from_raw(ffi::class_isMetaClass(self.as_ptr())).is_true() }
    }

    /// Returns the size of instances of self.
    pub fn instance_size(&self) -> usize {
        unsafe { ffi::class_getInstanceSize(self.as_ptr()) as usize }
    }

    /// Returns a specified instance method for self, or [`None`] if self and
    /// its superclasses do not contain an instance method with the specified
    /// selector.
    pub fn instance_method(&self, sel: Sel) -> Option<&Method> {
        unsafe {
            let method = ffi::class_getInstanceMethod(self.as_ptr(), sel.ptr);
            if method.is_null() {
                None
            } else {
                Some(&*(method as *const Method))
            }
        }
    }

    // fn class_method(&self, sel: Sel) -> Option<&Method>;

    /// Returns the ivar for a specified instance variable of self, or
    /// [`None`] if self has no ivar with the given name.
    pub fn instance_variable(&self, name: &str) -> Option<&Ivar> {
        let name = CString::new(name).unwrap();
        unsafe {
            let ivar = ffi::class_getInstanceVariable(self.as_ptr(), name.as_ptr());
            if ivar.is_null() {
                None
            } else {
                Some(&*(ivar as *const Ivar))
            }
        }
    }

    #[allow(unused)]
    fn instance_variable_layout(&self) -> Option<&[u8]> {
        let layout = unsafe { ffi::class_getIvarLayout(self.as_ptr()) };
        if layout.is_null() {
            None
        } else {
            Some(unsafe { CStr::from_ptr(layout as *const _) }.to_bytes())
        }
    }

    #[allow(unused)]
    fn class_variable(&self, name: &str) -> Option<&Ivar> {
        let name = CString::new(name).unwrap();
        let ivar = unsafe { ffi::class_getClassVariable(self.as_ptr(), name.as_ptr()) };
        // SAFETY: TODO
        unsafe { ivar.cast::<Ivar>().as_ref() }
    }

    /// Describes the instance methods implemented by self.
    #[cfg(feature = "malloc")]
    pub fn instance_methods(&self) -> Malloc<[&Method]> {
        unsafe {
            let mut count: c_uint = 0;
            let methods = ffi::class_copyMethodList(self.as_ptr(), &mut count);
            Malloc::from_array(methods as *mut _, count as usize)
        }
    }

    /// Checks whether this class conforms to the specified protocol.
    pub fn conforms_to(&self, proto: &Protocol) -> bool {
        unsafe {
            Bool::from_raw(ffi::class_conformsToProtocol(self.as_ptr(), proto.as_ptr())).is_true()
        }
    }

    /// Get a list of the protocols to which this class conforms.
    #[cfg(feature = "malloc")]
    pub fn adopted_protocols(&self) -> Malloc<[&Protocol]> {
        unsafe {
            let mut count: c_uint = 0;
            let protos = ffi::class_copyProtocolList(self.as_ptr(), &mut count);
            Malloc::from_array(protos as *mut _, count as usize)
        }
    }

    /// Describes the instance variables declared by self.
    #[cfg(feature = "malloc")]
    pub fn instance_variables(&self) -> Malloc<[&Ivar]> {
        unsafe {
            let mut count: c_uint = 0;
            let ivars = ffi::class_copyIvarList(self.as_ptr(), &mut count);
            Malloc::from_array(ivars as *mut _, count as usize)
        }
    }

    // fn property(&self, name: &str) -> Option<&Property>;
    // fn properties(&self) -> Malloc<[&Property]>;
    // unsafe fn replace_method(&self, name: Sel, imp: Imp, types: &str) -> Imp;
    // unsafe fn replace_property(&self, name: &str, attributes: &[ffi::objc_property_attribute_t]);
    // unsafe fn set_ivar_layout(&mut self, layout: &[u8]);
    // fn method_imp(&self, name: Sel) -> Imp; // + _stret
    // fn responds_to(&self, sel: Sel) -> bool;

    // fn get_version(&self) -> u32;
    // unsafe fn set_version(&mut self, version: u32);
}

// SAFETY: Class is immutable (and can be retrieved from any thread using the
// `class!` macro).
unsafe impl Sync for Class {}
unsafe impl Send for Class {}
impl UnwindSafe for Class {}
impl RefUnwindSafe for Class {}
// Note that Unpin is not applicable.

unsafe impl RefEncode for Class {
    const ENCODING_REF: Encoding<'static> = Encoding::Class;
}

impl PartialEq for Class {
    fn eq(&self, other: &Class) -> bool {
        self.as_ptr() == other.as_ptr()
    }
}

impl Eq for Class {}

impl fmt::Debug for Class {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl Protocol {
    pub(crate) fn as_ptr(&self) -> *const ffi::Protocol {
        self as *const Self as *const _
    }

    /// Returns the protocol definition of a specified protocol, or [`None`]
    /// if the protocol is not registered with the Objective-C runtime.
    pub fn get(name: &str) -> Option<&'static Protocol> {
        let name = CString::new(name).unwrap();
        unsafe {
            let proto = ffi::objc_getProtocol(name.as_ptr());
            if proto.is_null() {
                None
            } else {
                Some(&*(proto as *const Self))
            }
        }
    }

    /// Obtains the list of registered protocol definitions.
    #[cfg(feature = "malloc")]
    pub fn protocols() -> Malloc<[&'static Protocol]> {
        unsafe {
            let mut count: c_uint = 0;
            let protocols = ffi::objc_copyProtocolList(&mut count);
            Malloc::from_array(protocols as *mut _, count as usize)
        }
    }

    /// Get a list of the protocols to which this protocol conforms.
    #[cfg(feature = "malloc")]
    pub fn adopted_protocols(&self) -> Malloc<[&Protocol]> {
        unsafe {
            let mut count: c_uint = 0;
            let protocols = ffi::protocol_copyProtocolList(self.as_ptr(), &mut count);
            Malloc::from_array(protocols as *mut _, count as usize)
        }
    }

    /// Checks whether this protocol conforms to the specified protocol.
    pub fn conforms_to(&self, proto: &Protocol) -> bool {
        unsafe {
            Bool::from_raw(ffi::protocol_conformsToProtocol(
                self.as_ptr(),
                proto.as_ptr(),
            ))
            .is_true()
        }
    }

    /// Returns the name of self.
    pub fn name(&self) -> &str {
        let name = unsafe { CStr::from_ptr(ffi::protocol_getName(self.as_ptr())) };
        str::from_utf8(name.to_bytes()).unwrap()
    }
}

impl PartialEq for Protocol {
    fn eq(&self, other: &Protocol) -> bool {
        unsafe { Bool::from_raw(ffi::protocol_isEqual(self.as_ptr(), other.as_ptr())).is_true() }
    }
}

unsafe impl RefEncode for Protocol {
    // Protocol is an object internally
    const ENCODING_REF: Encoding<'static> = Encoding::Object;
}

impl Eq for Protocol {}

impl fmt::Debug for Protocol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

// SAFETY: Protocol is immutable (and can be retrieved from Class anyhow).
unsafe impl Sync for Protocol {}
unsafe impl Send for Protocol {}
impl UnwindSafe for Protocol {}
impl RefUnwindSafe for Protocol {}
// Note that Unpin is not applicable.

fn get_ivar_offset<T: Encode>(cls: &Class, name: &str) -> isize {
    match cls.instance_variable(name) {
        Some(ivar) => {
            assert!(T::ENCODING.equivalent_to_str(ivar.type_encoding()));
            ivar.offset()
        }
        None => panic!("Ivar {} not found on class {:?}", name, cls),
    }
}

impl Object {
    pub(crate) fn as_ptr(&self) -> *const ffi::objc_object {
        self as *const Self as *const _
    }

    /// Returns the class of this object.
    pub fn class(&self) -> &Class {
        unsafe { &*(ffi::object_getClass(self.as_ptr()) as *const Class) }
    }

    /// Returns a shared reference to the ivar with the given name.
    ///
    /// # Panics
    ///
    /// Panics if the object has no ivar with the given name, or the type
    /// encoding of the ivar differs from the type encoding of `T`.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the ivar is actually of type `T`.
    pub unsafe fn get_ivar<T: Encode>(&self, name: &str) -> &T {
        let offset = get_ivar_offset::<T>(self.class(), name);
        // `offset` is given in bytes, so we convert to `u8`
        let ptr = self as *const Self as *const u8;
        let ptr = unsafe { ptr.offset(offset) } as *const T;
        unsafe { &*ptr }
    }

    /// Returns a mutable reference to the ivar with the given name.
    ///
    /// # Panics
    ///
    /// Panics if the object has no ivar with the given name, or the type
    /// encoding of the ivar differs from the type encoding of `T`.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the ivar is actually of type `T`.
    pub unsafe fn get_mut_ivar<T: Encode>(&mut self, name: &str) -> &mut T {
        let offset = get_ivar_offset::<T>(self.class(), name);
        // `offset` is given in bytes, so we convert to `u8`
        let ptr = self as *mut Self as *mut u8;
        let ptr = unsafe { ptr.offset(offset) } as *mut T;
        unsafe { &mut *ptr }
    }

    /// Sets the value of the ivar with the given name.
    ///
    /// # Panics
    ///
    /// Panics if the object has no ivar with the given name, or the type
    /// encoding of the ivar differs from the type encoding of `T`.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the ivar is actually of type `T`.
    pub unsafe fn set_ivar<T: Encode>(&mut self, name: &str, value: T) {
        // SAFETY: Invariants upheld by caller
        unsafe { *self.get_mut_ivar::<T>(name) = value };
    }

    // objc_setAssociatedObject
    // objc_getAssociatedObject
    // objc_removeAssociatedObjects
}

/// ```
/// use objc2::runtime::Object;
/// fn needs_nothing<T: ?Sized>() {}
/// needs_nothing::<Object>();
/// ```
/// ```compile_fail
/// use objc2::runtime::Object;
/// fn needs_sync<T: ?Sized + Sync>() {}
/// needs_sync::<Object>();
/// ```
/// ```compile_fail
/// use objc2::runtime::Object;
/// fn needs_send<T: ?Sized + Send>() {}
/// needs_send::<Object>();
/// ```
#[cfg(doctest)]
pub struct ObjectNotSendNorSync;

unsafe impl RefEncode for Object {
    const ENCODING_REF: Encoding<'static> = Encoding::Object;
}

impl fmt::Debug for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<{:?}: {:p}>", self.class(), self.as_ptr())
    }
}

#[cfg(test)]
mod tests {
    use alloc::string::ToString;

    use super::{Bool, Class, Imp, Ivar, Method, Object, Protocol, Sel};
    use crate::test_utils;
    use crate::Encode;

    #[test]
    fn test_ivar() {
        let cls = test_utils::custom_class();
        let ivar = cls.instance_variable("_foo").unwrap();
        assert_eq!(ivar.name(), "_foo");
        assert!(<u32>::ENCODING.equivalent_to_str(ivar.type_encoding()));
        assert!(ivar.offset() > 0);

        #[cfg(feature = "malloc")]
        assert!(cls.instance_variables().len() > 0);
    }

    #[test]
    fn test_method() {
        let cls = test_utils::custom_class();
        let sel = Sel::register("foo");
        let method = cls.instance_method(sel).unwrap();
        assert_eq!(method.name().name(), "foo");
        assert_eq!(method.arguments_count(), 2);
        #[cfg(feature = "malloc")]
        {
            assert!(<u32>::ENCODING.equivalent_to_str(&method.return_type()));
            assert!(Sel::ENCODING.equivalent_to_str(&method.argument_type(1).unwrap()));

            let methods = cls.instance_methods();
            assert!(methods.len() > 0);
        }
    }

    #[test]
    fn test_class() {
        let cls = test_utils::custom_class();
        assert_eq!(cls.name(), "CustomObject");
        assert!(cls.instance_size() > 0);
        assert!(cls.superclass().is_none());

        assert_eq!(Class::get(cls.name()), Some(cls));

        let metaclass = cls.metaclass();
        // The metaclass of a root class is a subclass of the root class
        assert_eq!(metaclass.superclass().unwrap(), cls);

        let subclass = test_utils::custom_subclass();
        assert_eq!(subclass.superclass().unwrap(), cls);
    }

    #[test]
    fn test_classes_count() {
        assert!(Class::classes_count() > 0);
    }

    #[test]
    #[cfg(feature = "malloc")]
    fn test_classes() {
        let classes = Class::classes();
        assert!(classes.len() > 0);
    }

    #[test]
    fn test_protocol() {
        let proto = test_utils::custom_protocol();
        assert_eq!(proto.name(), "CustomProtocol");
        let class = test_utils::custom_class();
        assert!(class.conforms_to(proto));
        #[cfg(feature = "malloc")]
        assert!(class.adopted_protocols().len() > 0);
    }

    #[test]
    fn test_protocol_method() {
        let class = test_utils::custom_class();
        let result: i32 = unsafe { msg_send![class, addNumber: 1, toNumber: 2] };
        assert_eq!(result, 3);
    }

    #[test]
    fn test_subprotocols() {
        let sub_proto = test_utils::custom_subprotocol();
        let super_proto = test_utils::custom_protocol();
        assert!(sub_proto.conforms_to(super_proto));
        #[cfg(feature = "malloc")]
        assert_eq!(sub_proto.adopted_protocols()[0], super_proto);
    }

    #[test]
    fn test_protocols() {
        // Ensure that a protocol has been registered on linux
        let _ = test_utils::custom_protocol();

        #[cfg(feature = "malloc")]
        assert!(Protocol::protocols().len() > 0);
    }

    #[test]
    fn test_object() {
        let mut obj = test_utils::custom_object();
        assert_eq!(obj.class(), test_utils::custom_class());
        let result: u32 = unsafe {
            obj.set_ivar("_foo", 4u32);
            *obj.get_ivar("_foo")
        };
        assert_eq!(result, 4);
    }

    #[test]
    fn test_encode() {
        fn assert_enc<T: Encode>(expected: &str) {
            assert_eq!(&T::ENCODING.to_string(), expected);
        }
        assert_enc::<&Object>("@");
        assert_enc::<*mut Object>("@");
        assert_enc::<&Class>("#");
        assert_enc::<Sel>(":");
        assert_enc::<Imp>("^?");
        assert_enc::<Option<Imp>>("^?");
        assert_enc::<&Protocol>("@");
    }

    #[test]
    fn test_send_sync() {
        fn assert_send_sync<T: Send + Sync + ?Sized>() {}
        assert_send_sync::<Bool>();
        assert_send_sync::<Class>();
        assert_send_sync::<Ivar>();
        assert_send_sync::<Method>();
        assert_send_sync::<Protocol>();
        assert_send_sync::<Sel>();
    }
}
