//! A Rust interface for the functionality of the Objective-C runtime.
//!
//! For more information on foreign functions, see Apple's documentation:
//! <https://developer.apple.com/library/mac/documentation/Cocoa/Reference/ObjCRuntimeRef/index.html>

use core::ffi::c_void;
use core::fmt;
use core::ptr;
use core::str;
use malloc_buf::Malloc;
use std::ffi::{CStr, CString};
use std::os::raw::c_uint;

#[allow(deprecated)]
pub use objc2_sys::object_dispose;
pub use objc2_sys::{
    class_addIvar, class_addMethod, class_addProtocol, class_conformsToProtocol,
    class_copyIvarList, class_copyMethodList, class_copyProtocolList, class_createInstance,
    class_getInstanceMethod, class_getInstanceSize, class_getInstanceVariable, class_getName,
    class_getSuperclass, ivar_getName, ivar_getOffset, ivar_getTypeEncoding,
    method_copyArgumentType, method_copyReturnType, method_exchangeImplementations,
    method_getImplementation, method_getName, method_getNumberOfArguments,
    method_setImplementation, objc_allocateClassPair, objc_allocateProtocol, objc_autorelease,
    objc_autoreleasePoolPop, objc_autoreleasePoolPush, objc_copyClassList, objc_copyProtocolList,
    objc_copyWeak, objc_destroyWeak, objc_disposeClassPair, objc_getClass, objc_getClassList,
    objc_getProtocol, objc_initWeak, objc_loadWeakRetained, objc_registerClassPair,
    objc_registerProtocol, objc_release, objc_retain, object_getClass,
    protocol_addMethodDescription, protocol_addProtocol, protocol_conformsToProtocol,
    protocol_copyProtocolList, protocol_getName, protocol_isEqual, sel_getName, sel_registerName,
};
pub use objc2_sys::{BOOL, NO, YES};

pub use super::bool::Bool;
use crate::Encode;

/// A type that represents a method selector.
#[repr(transparent)]
pub struct Sel {
    ptr: *const objc2_sys::objc_selector,
}

/// A type that represents an instance variable.
#[repr(C)]
pub struct Ivar(objc2_sys::objc_ivar);

/// A type that represents a method in a class definition.
#[repr(C)]
pub struct Method(objc2_sys::objc_method);

/// A type that represents an Objective-C class.
#[repr(C)]
pub struct Class(objc2_sys::objc_class);

/// A type that represents an Objective-C protocol.
#[repr(C)]
pub struct Protocol(objc2_sys::Protocol);

/// A type that represents an instance of a class.
#[repr(C)]
pub struct Object(objc2_sys::objc_object);

/// A pointer to the start of a method implementation.
pub type Imp = unsafe extern "C" fn();

impl Sel {
    /// Registers a method with the Objective-C runtime system,
    /// maps the method name to a selector, and returns the selector value.
    pub fn register(name: &str) -> Self {
        let name = CString::new(name).unwrap();
        Self {
            ptr: unsafe { sel_registerName(name.as_ptr()) },
        }
    }

    /// Returns the name of the method specified by self.
    pub fn name(&self) -> &str {
        let name = unsafe { CStr::from_ptr(sel_getName(self.ptr)) };
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
            ptr: ptr as *const objc2_sys::objc_selector,
        }
    }

    /// Returns a pointer to the raw selector.
    #[inline]
    pub fn as_ptr(&self) -> *const c_void {
        self.ptr as *const c_void
    }
}

impl PartialEq for Sel {
    fn eq(&self, other: &Sel) -> bool {
        self.ptr == other.ptr
    }
}

impl Eq for Sel {}

// Sel is safe to share across threads because it is immutable
unsafe impl Sync for Sel {}
unsafe impl Send for Sel {}

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
    pub(crate) fn as_ptr(&self) -> *const objc2_sys::objc_ivar {
        self as *const Self as *const _
    }

    /// Returns the name of self.
    pub fn name(&self) -> &str {
        let name = unsafe { CStr::from_ptr(ivar_getName(self.as_ptr())) };
        str::from_utf8(name.to_bytes()).unwrap()
    }

    /// Returns the offset of self.
    pub fn offset(&self) -> isize {
        let offset = unsafe { ivar_getOffset(self.as_ptr()) };
        offset as isize
    }

    /// Returns the `Encoding` of self.
    pub fn type_encoding(&self) -> &str {
        let encoding = unsafe { CStr::from_ptr(ivar_getTypeEncoding(self.as_ptr())) };
        str::from_utf8(encoding.to_bytes()).unwrap()
    }
}

impl Method {
    pub(crate) fn as_ptr(&self) -> *const objc2_sys::objc_method {
        self as *const Self as *const _
    }

    /// Returns the name of self.
    pub fn name(&self) -> Sel {
        Sel {
            ptr: unsafe { method_getName(self.as_ptr()) },
        }
    }

    /// Returns the `Encoding` of self's return type.
    pub fn return_type(&self) -> Malloc<str> {
        unsafe {
            let encoding = method_copyReturnType(self.as_ptr());
            Malloc::from_c_str(encoding).unwrap()
        }
    }

    /// Returns the `Encoding` of a single parameter type of self, or
    /// [`None`] if self has no parameter at the given index.
    pub fn argument_type(&self, index: usize) -> Option<Malloc<str>> {
        unsafe {
            let encoding = method_copyArgumentType(self.as_ptr(), index as c_uint);
            if encoding.is_null() {
                None
            } else {
                Some(Malloc::from_c_str(encoding).unwrap())
            }
        }
    }

    /// Returns the number of arguments accepted by self.
    pub fn arguments_count(&self) -> usize {
        unsafe { method_getNumberOfArguments(self.as_ptr()) as usize }
    }

    /// Returns the implementation of self.
    pub fn implementation(&self) -> Imp {
        unsafe { method_getImplementation(self.as_ptr()).expect("Null IMP") }
    }
}

impl Class {
    pub(crate) fn as_ptr(&self) -> *const objc2_sys::objc_class {
        self as *const Self as *const _
    }

    /// Returns the class definition of a specified class, or [`None`] if the
    /// class is not registered with the Objective-C runtime.
    pub fn get(name: &str) -> Option<&'static Self> {
        let name = CString::new(name).unwrap();
        unsafe {
            let cls = objc_getClass(name.as_ptr());
            if cls.is_null() {
                None
            } else {
                Some(&*(cls as *const Self))
            }
        }
    }

    /// Obtains the list of registered class definitions.
    pub fn classes() -> Malloc<[&'static Self]> {
        unsafe {
            let mut count: c_uint = 0;
            let classes = objc_copyClassList(&mut count);
            Malloc::from_array(classes as *mut _, count as usize)
        }
    }

    /// Returns the total number of registered classes.
    pub fn classes_count() -> usize {
        unsafe { objc_getClassList(ptr::null_mut(), 0) as usize }
    }

    /// Returns the name of the class.
    pub fn name(&self) -> &str {
        let name = unsafe { CStr::from_ptr(class_getName(self.as_ptr())) };
        str::from_utf8(name.to_bytes()).unwrap()
    }

    /// Returns the superclass of self, or [`None`] if self is a root class.
    pub fn superclass(&self) -> Option<&Class> {
        unsafe {
            let superclass = class_getSuperclass(self.as_ptr());
            if superclass.is_null() {
                None
            } else {
                Some(&*(superclass as *const Self))
            }
        }
    }

    /// Returns the metaclass of self.
    pub fn metaclass(&self) -> &Self {
        unsafe { &*(object_getClass(self.as_ptr() as *const _) as *const Self) }
    }

    /// Returns the size of instances of self.
    pub fn instance_size(&self) -> usize {
        unsafe { class_getInstanceSize(self.as_ptr()) as usize }
    }

    /// Returns a specified instance method for self, or [`None`] if self and
    /// its superclasses do not contain an instance method with the specified
    /// selector.
    pub fn instance_method(&self, sel: Sel) -> Option<&Method> {
        unsafe {
            let method = class_getInstanceMethod(self.as_ptr(), sel.ptr);
            if method.is_null() {
                None
            } else {
                Some(&*(method as *const Method))
            }
        }
    }

    /// Returns the ivar for a specified instance variable of self, or
    /// [`None`] if self has no ivar with the given name.
    pub fn instance_variable(&self, name: &str) -> Option<&Ivar> {
        let name = CString::new(name).unwrap();
        unsafe {
            let ivar = class_getInstanceVariable(self.as_ptr(), name.as_ptr());
            if ivar.is_null() {
                None
            } else {
                Some(&*(ivar as *const Ivar))
            }
        }
    }

    /// Describes the instance methods implemented by self.
    pub fn instance_methods(&self) -> Malloc<[&Method]> {
        unsafe {
            let mut count: c_uint = 0;
            let methods = class_copyMethodList(self.as_ptr(), &mut count);
            Malloc::from_array(methods as *mut _, count as usize)
        }
    }

    /// Checks whether this class conforms to the specified protocol.
    pub fn conforms_to(&self, proto: &Protocol) -> bool {
        unsafe { Bool::from_raw(class_conformsToProtocol(self.as_ptr(), proto.as_ptr())).into() }
    }

    /// Get a list of the protocols to which this class conforms.
    pub fn adopted_protocols(&self) -> Malloc<[&Protocol]> {
        unsafe {
            let mut count: c_uint = 0;
            let protos = class_copyProtocolList(self.as_ptr(), &mut count);
            Malloc::from_array(protos as *mut _, count as usize)
        }
    }

    /// Describes the instance variables declared by self.
    pub fn instance_variables(&self) -> Malloc<[&Ivar]> {
        unsafe {
            let mut count: c_uint = 0;
            let ivars = class_copyIvarList(self.as_ptr(), &mut count);
            Malloc::from_array(ivars as *mut _, count as usize)
        }
    }
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
    pub(crate) fn as_ptr(&self) -> *const objc2_sys::Protocol {
        self as *const Self as *const _
    }

    /// Returns the protocol definition of a specified protocol, or [`None`]
    /// if the protocol is not registered with the Objective-C runtime.
    pub fn get(name: &str) -> Option<&'static Protocol> {
        let name = CString::new(name).unwrap();
        unsafe {
            let proto = objc_getProtocol(name.as_ptr());
            if proto.is_null() {
                None
            } else {
                Some(&*(proto as *const Self))
            }
        }
    }

    /// Obtains the list of registered protocol definitions.
    pub fn protocols() -> Malloc<[&'static Protocol]> {
        unsafe {
            let mut count: c_uint = 0;
            let protocols = objc_copyProtocolList(&mut count);
            Malloc::from_array(protocols as *mut _, count as usize)
        }
    }

    /// Get a list of the protocols to which this protocol conforms.
    pub fn adopted_protocols(&self) -> Malloc<[&Protocol]> {
        unsafe {
            let mut count: c_uint = 0;
            let protocols = protocol_copyProtocolList(self.as_ptr(), &mut count);
            Malloc::from_array(protocols as *mut _, count as usize)
        }
    }

    /// Checks whether this protocol conforms to the specified protocol.
    pub fn conforms_to(&self, proto: &Protocol) -> bool {
        unsafe { Bool::from_raw(protocol_conformsToProtocol(self.as_ptr(), proto.as_ptr())).into() }
    }

    /// Returns the name of self.
    pub fn name(&self) -> &str {
        let name = unsafe { CStr::from_ptr(protocol_getName(self.as_ptr())) };
        str::from_utf8(name.to_bytes()).unwrap()
    }
}

impl PartialEq for Protocol {
    fn eq(&self, other: &Protocol) -> bool {
        unsafe { Bool::from_raw(protocol_isEqual(self.as_ptr(), other.as_ptr())).is_true() }
    }
}

impl Eq for Protocol {}

impl fmt::Debug for Protocol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

fn get_ivar_offset<T: Encode>(cls: &Class, name: &str) -> isize {
    match cls.instance_variable(name) {
        Some(ivar) => {
            assert!(ivar.type_encoding() == &T::ENCODING);
            ivar.offset()
        }
        None => panic!("Ivar {} not found on class {:?}", name, cls),
    }
}

impl Object {
    pub(crate) fn as_ptr(&self) -> *const objc2_sys::objc_object {
        self as *const Self as *const _
    }

    /// Returns the class of this object.
    pub fn class(&self) -> &Class {
        unsafe { &*(object_getClass(self.as_ptr()) as *const Class) }
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
        let ptr = (self as *const Self as *const u8).offset(offset) as *const T;
        &*ptr
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
        let ptr = (self as *mut Self as *mut u8).offset(offset) as *mut T;
        &mut *ptr
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
        *self.get_mut_ivar::<T>(name) = value;
    }
}

impl fmt::Debug for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<{:?}: {:p}>", self.class(), self.as_ptr())
    }
}

#[cfg(test)]
mod tests {
    use super::{Class, Protocol, Sel};
    use crate::test_utils;
    use crate::Encode;

    #[test]
    fn test_ivar() {
        let cls = test_utils::custom_class();
        let ivar = cls.instance_variable("_foo").unwrap();
        assert!(ivar.name() == "_foo");
        assert!(ivar.type_encoding() == &<u32>::ENCODING);
        assert!(ivar.offset() > 0);

        let ivars = cls.instance_variables();
        assert!(ivars.len() > 0);
    }

    #[test]
    fn test_method() {
        let cls = test_utils::custom_class();
        let sel = Sel::register("foo");
        let method = cls.instance_method(sel).unwrap();
        assert!(method.name().name() == "foo");
        assert!(method.arguments_count() == 2);
        assert!(*method.return_type() == <u32>::ENCODING);
        assert!(*method.argument_type(1).unwrap() == Sel::ENCODING);

        let methods = cls.instance_methods();
        assert!(methods.len() > 0);
    }

    #[test]
    fn test_class() {
        let cls = test_utils::custom_class();
        assert!(cls.name() == "CustomObject");
        assert!(cls.instance_size() > 0);
        assert!(cls.superclass().is_none());

        assert!(Class::get(cls.name()) == Some(cls));

        let metaclass = cls.metaclass();
        // The metaclass of a root class is a subclass of the root class
        assert!(metaclass.superclass().unwrap() == cls);

        let subclass = test_utils::custom_subclass();
        assert!(subclass.superclass().unwrap() == cls);
    }

    #[test]
    fn test_classes() {
        assert!(Class::classes_count() > 0);
        let classes = Class::classes();
        assert!(classes.len() > 0);
    }

    #[test]
    fn test_protocol() {
        let proto = test_utils::custom_protocol();
        assert!(proto.name() == "CustomProtocol");
        let class = test_utils::custom_class();
        assert!(class.conforms_to(proto));
        let class_protocols = class.adopted_protocols();
        assert!(class_protocols.len() > 0);
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
        let adopted_protocols = sub_proto.adopted_protocols();
        assert_eq!(adopted_protocols[0], super_proto);
    }

    #[test]
    fn test_protocols() {
        // Ensure that a protocol has been registered on linux
        let _ = test_utils::custom_protocol();

        let protocols = Protocol::protocols();
        assert!(protocols.len() > 0);
    }

    #[test]
    fn test_object() {
        let mut obj = test_utils::custom_object();
        assert!(obj.class() == test_utils::custom_class());
        let result: u32 = unsafe {
            obj.set_ivar("_foo", 4u32);
            *obj.get_ivar("_foo")
        };
        assert!(result == 4);
    }
}
