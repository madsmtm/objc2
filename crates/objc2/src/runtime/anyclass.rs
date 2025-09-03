use core::ffi::c_uint;
use core::ffi::CStr;
use core::fmt;
use core::hash;
use core::panic::{RefUnwindSafe, UnwindSafe};
use core::ptr;

use super::verify_method_signature;
use super::AnyProtocol;
use super::Inner;
use super::Ivar;
use super::Method;
use super::Sel;
use super::VerificationError;
use super::{AnyObject, MallocSlice};
use crate::encode::EncodeArguments;
use crate::encode::EncodeReturn;
use crate::encode::{Encoding, RefEncode};
use crate::ffi;
use crate::Message;

/// An opaque type that represents an Objective-C class.
///
/// This is an opaque type meant to be used behind a shared reference
/// `&AnyClass`, which is semantically equivalent to `Class _Nonnull`.
///
/// A nullable class can be used as `Option<&AnyClass>`.
///
/// See [Apple's documentation](https://developer.apple.com/documentation/objectivec/class?language=objc).
#[repr(C)]
#[doc(alias = "Class")]
#[doc(alias = "objc_class")]
pub struct AnyClass {
    // `isa` field is deprecated and not available on GNUStep, so we don't
    // expose it here. Use `class_getSuperclass` instead.
    inner: AnyObject,
}

/// Use [`AnyClass`] instead.
#[deprecated = "renamed to `runtime::AnyClass`"]
pub type Class = AnyClass;

// SAFETY: AnyClass is immutable (and can be retrieved from any thread using
// the `class!` macro).
unsafe impl Sync for AnyClass {}
unsafe impl Send for AnyClass {}
impl UnwindSafe for AnyClass {}
impl RefUnwindSafe for AnyClass {}
// Note that Unpin is not applicable.

impl AnyClass {
    /// Returns the class definition of a specified class, or [`None`] if the
    /// class is not registered with the Objective-C runtime.
    #[inline]
    #[doc(alias = "objc_getClass")]
    pub fn get(name: &CStr) -> Option<&'static Self> {
        let cls = unsafe { ffi::objc_getClass(name.as_ptr()) };
        unsafe { cls.as_ref() }
    }

    // Same as `get`, but ...
    // fn lookup(name: &CStr) -> Option<&'static Self>;

    /// Obtains the list of registered class definitions.
    #[doc(alias = "objc_copyClassList")]
    pub fn classes() -> MallocSlice!(&'static Self) {
        unsafe {
            let mut count: c_uint = 0;
            let classes: *mut &Self = ffi::objc_copyClassList(&mut count).cast();
            MallocSlice::from_array(classes, count as usize)
        }
    }

    /// Returns the total number of registered classes.
    #[inline]
    #[doc(alias = "objc_getClassList")]
    pub fn classes_count() -> usize {
        unsafe { ffi::objc_getClassList(ptr::null_mut(), 0) as usize }
    }

    /// # Safety
    ///
    /// 1. The class pointer must be valid.
    /// 2. The string is unbounded, so the caller must bound it.
    pub(crate) unsafe fn name_raw<'a>(ptr: *const Self) -> &'a CStr {
        // SAFETY: Caller ensures that the pointer is valid
        let name = unsafe { ffi::class_getName(ptr) };
        if name.is_null() {
            panic!("class name was NULL");
        }
        // SAFETY: We've checked that the pointer is not NULL, and
        // `class_getName` is guaranteed to return a valid C-string.
        //
        // That the result is properly bounded is checked by the caller.
        unsafe { CStr::from_ptr(name) }
    }

    /// Returns the name of the class.
    #[inline]
    #[doc(alias = "class_getName")]
    pub fn name(&self) -> &CStr {
        // SAFETY: The pointer is valid, and the return is properly bounded
        unsafe { Self::name_raw(self) }
    }

    /// # Safety
    ///
    /// 1. The class pointer must be valid.
    /// 2. The caller must bound the lifetime of the returned class.
    #[inline]
    pub(crate) unsafe fn superclass_raw<'a>(ptr: *const Self) -> Option<&'a AnyClass> {
        // SAFETY: Caller ensures that the pointer is valid
        let superclass = unsafe { ffi::class_getSuperclass(ptr) };
        // SAFETY: The result is properly bounded by the caller.
        unsafe { superclass.as_ref() }
    }

    /// Returns the superclass of self, or [`None`] if self is a root class.
    #[inline]
    #[doc(alias = "class_getSuperclass")]
    pub fn superclass(&self) -> Option<&AnyClass> {
        // SAFETY: The pointer is valid, and the return is properly bounded
        unsafe { Self::superclass_raw(self) }
    }

    /// Returns the metaclass of self.
    ///
    ///
    /// # Example
    ///
    /// Get the metaclass of an object.
    ///
    /// ```
    /// use objc2::runtime::NSObject;
    /// use objc2::ClassType;
    ///
    /// let cls = NSObject::class();
    /// let metacls = cls.metaclass();
    ///
    /// assert_eq!(metacls.name(), c"NSObject");
    /// ```
    #[inline]
    #[doc(alias = "object_getClass")]
    #[doc(alias = "objc_getMetaClass")] // Same as `AnyClass::get(name).metaclass()`
    pub fn metaclass(&self) -> &Self {
        let ptr: *const Self = self;
        let ptr = unsafe { ffi::object_getClass(ptr.cast()) };
        unsafe { ptr.as_ref().unwrap_unchecked() }
    }

    /// Whether the class is a metaclass.
    ///
    ///
    /// # Example
    ///
    /// ```
    /// use objc2::runtime::NSObject;
    /// use objc2::ClassType;
    ///
    /// let cls = NSObject::class();
    /// let metacls = cls.metaclass();
    ///
    /// assert!(!cls.is_metaclass());
    /// assert!(metacls.is_metaclass());
    /// ```
    #[inline]
    #[doc(alias = "class_isMetaClass")]
    pub fn is_metaclass(&self) -> bool {
        unsafe { ffi::class_isMetaClass(self).as_bool() }
    }

    /// Returns the size of instances of self.
    #[inline]
    #[doc(alias = "class_getInstanceSize")]
    pub fn instance_size(&self) -> usize {
        unsafe { ffi::class_getInstanceSize(self) }
    }

    /// Returns a specified instance method for self, or [`None`] if self and
    /// its superclasses do not contain an instance method with the specified
    /// selector.
    #[inline]
    #[doc(alias = "class_getInstanceMethod")]
    pub fn instance_method(&self, sel: Sel) -> Option<&Method> {
        unsafe {
            let method = ffi::class_getInstanceMethod(self, sel);
            method.as_ref()
        }
    }

    /// Returns a specified class method for self, or [`None`] if self and
    /// its superclasses do not contain a class method with the specified
    /// selector.
    ///
    /// Same as `cls.metaclass().class_method()`.
    #[inline]
    #[doc(alias = "class_getClassMethod")]
    pub fn class_method(&self, sel: Sel) -> Option<&Method> {
        unsafe {
            let method = ffi::class_getClassMethod(self, sel);
            method.as_ref()
        }
    }

    /// Returns the ivar for a specified instance variable of self, or
    /// [`None`] if self has no ivar with the given name.
    ///
    /// If the instance variable was not found on the specified class, the
    /// superclasses are searched.
    ///
    /// Attempting to access or modify instance variables of a class that you
    /// do no control may invoke undefined behaviour.
    #[inline]
    #[doc(alias = "class_getInstanceVariable")]
    pub fn instance_variable(&self, name: &CStr) -> Option<&Ivar> {
        unsafe {
            let ivar = ffi::class_getInstanceVariable(self, name.as_ptr());
            ivar.as_ref()
        }
    }

    #[allow(unused)]
    #[inline]
    #[doc(alias = "class_getClassVariable")]
    fn class_variable(&self, name: &CStr) -> Option<&Ivar> {
        let ivar = unsafe { ffi::class_getClassVariable(self, name.as_ptr()) };
        // SAFETY: TODO
        unsafe { ivar.as_ref() }
    }

    /// Describes the instance methods implemented by self.
    #[doc(alias = "class_copyMethodList")]
    pub fn instance_methods(&self) -> MallocSlice!(&Method) {
        unsafe {
            let mut count: c_uint = 0;
            let methods: *mut &Method = ffi::class_copyMethodList(self, &mut count).cast();
            MallocSlice::from_array(methods, count as usize)
        }
    }

    /// Checks whether this class conforms to the specified protocol.
    #[inline]
    #[doc(alias = "class_conformsToProtocol")]
    pub fn conforms_to(&self, proto: &AnyProtocol) -> bool {
        unsafe { ffi::class_conformsToProtocol(self, proto).as_bool() }
    }

    /// Get a list of the protocols to which this class conforms.
    #[doc(alias = "class_copyProtocolList")]
    pub fn adopted_protocols(&self) -> MallocSlice!(&AnyProtocol) {
        unsafe {
            let mut count: c_uint = 0;
            let protos: *mut &AnyProtocol = ffi::class_copyProtocolList(self, &mut count).cast();
            MallocSlice::from_array(protos, count as usize)
        }
    }

    /// Get a list of instance variables on the class.
    #[doc(alias = "class_copyIvarList")]
    pub fn instance_variables(&self) -> MallocSlice!(&Ivar) {
        unsafe {
            let mut count: c_uint = 0;
            let ivars: *mut &Ivar = ffi::class_copyIvarList(self, &mut count).cast();
            MallocSlice::from_array(ivars, count as usize)
        }
    }

    /// Check whether instances of this class respond to the given selector.
    ///
    /// This doesn't call `respondsToSelector:`, but works entirely within the
    /// runtime, which means it'll always be safe to call, but may not return
    /// exactly what you'd expect if `respondsToSelector:` has been
    /// overwritten.
    ///
    /// That said, it will always return `true` if an instance of the class
    /// responds to the selector, but may return `false` if they don't
    /// directly (e.g. does so by using forwarding instead).
    #[inline]
    #[doc(alias = "class_respondsToSelector")]
    pub fn responds_to(&self, sel: Sel) -> bool {
        // This may call `resolveInstanceMethod:` and `resolveClassMethod:`
        // SAFETY: The selector is guaranteed non-null.
        unsafe { ffi::class_respondsToSelector(self, sel).as_bool() }
    }

    // <https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/ObjCRuntimeGuide/Articles/ocrtPropertyIntrospection.html>
    // fn property(&self, name: &CStr) -> Option<&Property>;
    // fn properties(&self) -> MallocSlice!(&Property);
    // unsafe fn replace_method(&self, name: Sel, imp: Imp, types: &CStr) -> Imp;
    // unsafe fn replace_property(&self, name: &CStr, attributes: &[ffi::objc_property_attribute_t]);
    // fn method_imp(&self, name: Sel) -> Imp; // + _stret

    // fn get_version(&self) -> u32;
    // unsafe fn set_version(&mut self, version: u32);

    /// Verify argument and return types for a given selector.
    ///
    /// This will look up the encoding of the method for the given selector
    /// and return a [`VerificationError`] if any encodings differ for the
    /// arguments `A` and return type `R`.
    ///
    ///
    /// # Example
    ///
    /// ```
    /// use objc2::{class, sel};
    /// use objc2::runtime::{AnyClass, Bool};
    /// let cls = class!(NSObject);
    /// let sel = sel!(isKindOfClass:);
    /// // Verify that `isKindOfClass:`:
    /// // - Exists on the class
    /// // - Takes a class as a parameter
    /// // - Returns a BOOL
    /// let result = cls.verify_sel::<(&AnyClass,), Bool>(sel);
    /// assert!(result.is_ok());
    /// ```
    #[allow(clippy::missing_errors_doc)] // Written differently in the docs
    pub fn verify_sel<A, R>(&self, sel: Sel) -> Result<(), VerificationError>
    where
        A: EncodeArguments,
        R: EncodeReturn,
    {
        let method = self.instance_method(sel).ok_or(Inner::MethodNotFound)?;
        verify_method_signature(method, A::ENCODINGS, &R::ENCODING_RETURN)
    }
}

standard_pointer_impls!(AnyClass);

unsafe impl RefEncode for AnyClass {
    const ENCODING_REF: Encoding = Encoding::Class;
}

// SAFETY: Classes act as objects, and can be sent messages.
unsafe impl Message for AnyClass {}

impl fmt::Debug for AnyClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AnyClass")
            .field("name", &self.name())
            .finish_non_exhaustive()
    }
}

impl fmt::Display for AnyClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Classes are usually UTF-8, so it's probably fine to do a lossy
        // conversion here.
        fmt::Display::fmt(&self.name().to_string_lossy(), f)
    }
}

impl AsRef<Self> for AnyClass {
    fn as_ref(&self) -> &Self {
        self
    }
}

// This is the same as what Swift allows (`AnyClass` coerces to `AnyObject`).
impl AsRef<AnyObject> for AnyClass {
    fn as_ref(&self) -> &AnyObject {
        &self.inner
    }
}

#[cfg(test)]
mod tests {
    use super::super::test_utils;
    use super::*;
    use crate::encode::Encode;
    use crate::runtime::{ClassBuilder, NSObject, NSObjectProtocol};
    use crate::{msg_send, sel, ClassType, ProtocolType};
    use alloc::ffi::CString;

    // TODO: Remove once c"" strings are in MSRV
    fn c(s: &str) -> CString {
        CString::new(s).unwrap()
    }

    #[test]
    fn test_ivar() {
        let cls = test_utils::custom_class();
        let ivar = cls.instance_variable(&c("_foo")).unwrap();
        assert_eq!(ivar.name(), &*c("_foo"));
        assert!(<u32>::ENCODING.equivalent_to_str(ivar.type_encoding().to_str().unwrap()));
        assert!(ivar.offset() > 0);
        assert!(cls.instance_variables().len() > 0);
    }

    #[test]
    fn test_instance_method() {
        let cls = test_utils::custom_class();
        let sel = Sel::register(&c("foo"));
        let method = cls.instance_method(sel).unwrap();
        assert_eq!(method.name().name(), &*c("foo"));
        assert_eq!(method.arguments_count(), 2);

        assert!(<u32>::ENCODING.equivalent_to_str(method.return_type().to_str().unwrap()));
        assert!(Sel::ENCODING.equivalent_to_str(method.argument_type(1).unwrap().to_str().unwrap()));

        assert!(cls.instance_methods().contains(&method));
    }

    #[test]
    fn test_class_method() {
        let cls = test_utils::custom_class();
        let method = cls.class_method(sel!(classFoo)).unwrap();
        assert_eq!(method.name().name(), &*c("classFoo"));
        assert_eq!(method.arguments_count(), 2);

        assert!(<u32>::ENCODING.equivalent_to_str(method.return_type().to_str().unwrap()));
        assert!(Sel::ENCODING.equivalent_to_str(method.argument_type(1).unwrap().to_str().unwrap()));

        assert!(cls.metaclass().instance_methods().contains(&method));
    }

    #[test]
    #[cfg_attr(feature = "gnustep-1-7", ignore = "flaky")]
    fn test_class() {
        let cls = test_utils::custom_class();
        assert_eq!(cls.name(), &*c("CustomObject"));
        assert!(cls.instance_size() > 0);
        assert!(cls.superclass().is_none());

        assert!(cls.responds_to(sel!(foo)));
        assert!(cls.responds_to(sel!(setBar:)));
        assert!(cls.responds_to(sel!(test::test::)));
        assert!(!cls.responds_to(sel!(abc)));
        assert!(!cls.responds_to(sel!(addNumber:toNumber:)));

        assert_eq!(AnyClass::get(cls.name()), Some(cls));

        let metaclass = cls.metaclass();
        // The metaclass of a root class is a subclass of the root class
        assert_eq!(metaclass.superclass().unwrap(), cls);
        assert!(metaclass.responds_to(sel!(addNumber:toNumber:)));
        assert!(metaclass.responds_to(sel!(test::test::)));
        // TODO: This is unexpected!
        assert!(metaclass.responds_to(sel!(foo)));

        let subclass = test_utils::custom_subclass();
        assert_eq!(subclass.superclass().unwrap(), cls);
    }

    #[test]
    fn test_classes_count() {
        assert!(AnyClass::classes_count() > 0);
    }

    #[test]
    fn test_classes() {
        let classes = AnyClass::classes();
        assert!(classes.len() > 0);
    }

    #[test]
    fn class_is_object() {
        let cls = NSObject::class();
        let retained = cls.retain();
        assert_eq!(&*retained, cls);

        let obj: &AnyObject = cls.as_ref();
        let superclass = obj.class();
        assert!(superclass.conforms_to(<dyn NSObjectProtocol>::protocol().unwrap()));

        // Classes are NSObject subclasses in the current runtime.
        let ns_obj = retained.downcast::<NSObject>().unwrap();
        // Test that we can call NSObject methods on classes.
        assert_eq!(ns_obj, ns_obj);
        let _ = ns_obj.retainCount();
    }

    #[test]
    fn class_has_infinite_retain_count() {
        let obj: &AnyObject = NSObject::class().as_ref();
        let obj = obj.downcast_ref::<NSObject>().unwrap();

        let large_retain = if cfg!(feature = "gnustep-1-7") {
            u32::MAX as usize
        } else {
            usize::MAX
        };

        assert_eq!(obj.retainCount(), large_retain);
        let obj2 = obj.retain();
        assert_eq!(obj.retainCount(), large_retain);
        drop(obj2);
        assert_eq!(obj.retainCount(), large_retain);
    }

    #[test]
    fn test_protocol_method() {
        let class = test_utils::custom_class();
        let result: i32 = unsafe { msg_send![class, addNumber: 1, toNumber: 2] };
        assert_eq!(result, 3);
    }

    #[test]
    fn class_self() {
        let cls = NSObject::class();
        let result: &'static AnyClass = unsafe { msg_send![cls, self] };
        assert_eq!(cls, result);
    }

    #[test]
    fn test_object() {
        let obj = test_utils::custom_object();
        let cls = test_utils::custom_class();
        assert_eq!(obj.class(), cls);

        let ivar = cls.instance_variable(&c("_foo")).unwrap();

        unsafe { *ivar.load_ptr::<u32>(&obj) = 4 };
        let result = unsafe { *ivar.load::<u32>(&obj) };
        assert_eq!(result, 4);
    }

    #[test]
    fn test_ivar_unknown() {
        let cls = test_utils::custom_class();
        assert_eq!(cls.instance_variable(&c("unknown")), None);
    }

    #[test]
    fn test_no_ivars() {
        let cls = ClassBuilder::new(&c("NoIvarObject"), NSObject::class())
            .unwrap()
            .register();
        assert_eq!(cls.instance_variables().len(), 0);
    }
}
