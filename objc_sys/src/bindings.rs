use core::ffi::c_void;
use std::os::raw::{c_char, c_int, c_uint};

use crate::{
    objc_AssociationPolicy, objc_class, objc_exception_matcher, objc_exception_preprocessor,
    objc_ivar, objc_method, objc_method_description, objc_object, objc_property_attribute_t,
    objc_property_t, objc_protocol, objc_selector, objc_uncaught_exception_handler, BOOL, IMP,
};

extern "C" {
    pub fn class_addIvar(
        cls: *mut objc_class,
        name: *const c_char,
        size: usize,
        alignment: u8,
        types: *const c_char,
    ) -> BOOL;
    pub fn class_addMethod(
        cls: *mut objc_class,
        name: *const objc_selector,
        imp: IMP,
        types: *const c_char,
    ) -> BOOL;
    pub fn class_addProperty(
        cls: *mut objc_class,
        name: *const c_char,
        attributes: *const objc_property_attribute_t,
        attributeCount: c_uint,
    ) -> BOOL;
    pub fn class_addProtocol(cls: *mut objc_class, protocol: *mut objc_protocol) -> BOOL;
    pub fn class_conformsToProtocol(cls: *mut objc_class, protocol: *mut objc_protocol) -> BOOL;
    pub fn class_copyIvarList(cls: *mut objc_class, outCount: *mut c_uint)
        -> *mut *const objc_ivar;
    pub fn class_copyMethodList(
        cls: *mut objc_class,
        outCount: *mut c_uint,
    ) -> *mut *mut objc_method;
    pub fn class_copyPropertyList(
        cls: *mut objc_class,
        outCount: *mut c_uint,
    ) -> *mut objc_property_t;
    pub fn class_copyProtocolList(
        cls: *mut objc_class,
        outCount: *mut c_uint,
    ) -> *mut *mut objc_protocol;
    pub fn class_createInstance(cls: *mut objc_class, extraBytes: usize) -> *mut objc_object;
    pub fn class_getClassMethod(
        cls: *mut objc_class,
        name: *const objc_selector,
    ) -> *mut objc_method;
    pub fn class_getClassVariable(cls: *mut objc_class, name: *const c_char) -> *const objc_ivar;
    pub fn class_getImageName(cls: *mut objc_class) -> *const c_char;
    pub fn class_getInstanceMethod(
        cls: *mut objc_class,
        name: *const objc_selector,
    ) -> *mut objc_method;
    pub fn class_getInstanceSize(cls: *mut objc_class) -> usize;
    pub fn class_getInstanceVariable(cls: *mut objc_class, name: *const c_char)
        -> *const objc_ivar;
    pub fn class_getIvarLayout(cls: *mut objc_class) -> *const u8;
    pub fn class_getName(cls: *mut objc_class) -> *const c_char;
    pub fn class_getProperty(cls: *mut objc_class, name: *const c_char) -> objc_property_t;
    pub fn class_getSuperclass(cls: *mut objc_class) -> *mut objc_class;
    pub fn class_getVersion(cls: *mut objc_class) -> c_int;
    pub fn class_getWeakIvarLayout(cls: *mut objc_class) -> *const u8;
    pub fn class_isMetaClass(cls: *mut objc_class) -> BOOL;
    pub fn class_replaceMethod(
        cls: *mut objc_class,
        name: *const objc_selector,
        imp: IMP,
        types: *const c_char,
    ) -> IMP;
    pub fn class_replaceProperty(
        cls: *mut objc_class,
        name: *const c_char,
        attributes: *const objc_property_attribute_t,
        attributeCount: c_uint,
    );
    pub fn class_respondsToSelector(cls: *mut objc_class, sel: *const objc_selector) -> BOOL;
    pub fn class_setIvarLayout(cls: *mut objc_class, layout: *const u8);
    pub fn class_setVersion(cls: *mut objc_class, version: c_int);
    pub fn class_setWeakIvarLayout(cls: *mut objc_class, layout: *const u8);
    pub fn imp_getBlock(anImp: IMP) -> *mut objc_object;
    pub fn imp_implementationWithBlock(block: *mut objc_object) -> IMP;
    pub fn imp_removeBlock(anImp: IMP) -> BOOL;
    pub fn ivar_getName(v: *const objc_ivar) -> *const c_char;
    pub fn ivar_getOffset(v: *const objc_ivar) -> isize;
    pub fn ivar_getTypeEncoding(v: *const objc_ivar) -> *const c_char;
    pub fn method_copyArgumentType(m: *mut objc_method, index: c_uint) -> *mut c_char;
    pub fn method_copyReturnType(m: *mut objc_method) -> *mut c_char;
    pub fn method_exchangeImplementations(m1: *mut objc_method, m2: *mut objc_method);
    pub fn method_getArgumentType(
        m: *mut objc_method,
        index: c_uint,
        dst: *mut c_char,
        dst_len: usize,
    );
    pub fn method_getDescription(m: *mut objc_method) -> *mut objc_method_description;
    pub fn method_getImplementation(m: *mut objc_method) -> IMP;
    pub fn method_getName(m: *mut objc_method) -> *const objc_selector;
    pub fn method_getNumberOfArguments(m: *mut objc_method) -> c_uint;
    pub fn method_getReturnType(m: *mut objc_method, dst: *mut c_char, dst_len: usize);
    pub fn method_getTypeEncoding(m: *mut objc_method) -> *const c_char;
    pub fn method_setImplementation(m: *mut objc_method, imp: IMP) -> IMP;
    pub fn objc_allocateClassPair(
        superclass: *mut objc_class,
        name: *const c_char,
        extraBytes: usize,
    ) -> *mut objc_class;
    pub fn objc_allocateProtocol(name: *const c_char) -> *mut objc_protocol;
    pub fn objc_begin_catch(exc_buf: *mut c_void) -> *mut objc_object;
    pub fn objc_copyClassList(outCount: *mut c_uint) -> *mut *mut objc_class;
    pub fn objc_copyClassNamesForImage(
        image: *const c_char,
        outCount: *mut c_uint,
    ) -> *mut *const c_char;
    pub fn objc_copyImageNames(outCount: *mut c_uint) -> *mut *const c_char;
    pub fn objc_copyProtocolList(outCount: *mut c_uint) -> *mut *mut objc_protocol;
    pub fn objc_disposeClassPair(cls: *mut objc_class);
    pub fn objc_duplicateClass(
        original: *mut objc_class,
        name: *const c_char,
        extraBytes: usize,
    ) -> *mut objc_class;
    pub fn objc_end_catch();
    pub fn objc_enumerationMutation(obj: *mut objc_object);
    pub fn objc_exception_rethrow();
    pub fn objc_exception_throw(exception: *mut objc_object);
    pub fn objc_getAssociatedObject(
        object: *mut objc_object,
        key: *const c_void,
    ) -> *mut objc_object;
    pub fn objc_getClass(name: *const c_char) -> *mut objc_class;
    pub fn objc_getClassList(buffer: *mut *mut objc_class, bufferCount: c_int) -> c_int;
    pub fn objc_getMetaClass(name: *const c_char) -> *mut objc_class;
    pub fn objc_getProtocol(name: *const c_char) -> *mut objc_protocol;
    pub fn objc_getRequiredClass(name: *const c_char) -> *mut objc_class;
    pub fn objc_lookUpClass(name: *const c_char) -> *mut objc_class;
    pub fn objc_registerClassPair(cls: *mut objc_class);
    pub fn objc_registerProtocol(proto: *mut objc_protocol);
    pub fn objc_removeAssociatedObjects(object: *mut objc_object);
    pub fn objc_setAssociatedObject(
        object: *mut objc_object,
        key: *const c_void,
        value: *mut objc_object,
        policy: objc_AssociationPolicy,
    );
    pub fn objc_setEnumerationMutationHandler(
        handler: Option<unsafe extern "C" fn(arg1: *mut objc_object)>,
    );
    pub fn objc_setExceptionMatcher(fn_: objc_exception_matcher) -> objc_exception_matcher;
    pub fn objc_setExceptionPreprocessor(
        fn_: objc_exception_preprocessor,
    ) -> objc_exception_preprocessor;
    pub fn objc_setForwardHandler(fwd: *mut c_void, fwd_stret: *mut c_void);
    pub fn objc_setUncaughtExceptionHandler(
        fn_: objc_uncaught_exception_handler,
    ) -> objc_uncaught_exception_handler;
    pub fn objc_sync_enter(obj: *mut objc_object) -> c_int;
    pub fn objc_sync_exit(obj: *mut objc_object) -> c_int;
    pub fn object_getClass(obj: *mut objc_object) -> *mut objc_class;
    pub fn object_getClassName(obj: *mut objc_object) -> *const c_char;
    pub fn object_getIndexedIvars(obj: *mut objc_object) -> *mut c_void;
    pub fn object_getIvar(obj: *mut objc_object, ivar: *const objc_ivar) -> *mut objc_object;
    pub fn object_setClass(obj: *mut objc_object, cls: *mut objc_class) -> *mut objc_class;
    pub fn object_setIvar(obj: *mut objc_object, ivar: *const objc_ivar, value: *mut objc_object);
    pub fn property_copyAttributeList(
        property: objc_property_t,
        outCount: *mut c_uint,
    ) -> *mut objc_property_attribute_t;
    pub fn property_copyAttributeValue(
        property: objc_property_t,
        attributeName: *const c_char,
    ) -> *mut c_char;
    pub fn property_getAttributes(property: objc_property_t) -> *const c_char;
    pub fn property_getName(property: objc_property_t) -> *const c_char;
    pub fn protocol_addMethodDescription(
        proto: *mut objc_protocol,
        name: *const objc_selector,
        types: *const c_char,
        isRequiredMethod: BOOL,
        isInstanceMethod: BOOL,
    );
    pub fn protocol_addProperty(
        proto: *mut objc_protocol,
        name: *const c_char,
        attributes: *const objc_property_attribute_t,
        attributeCount: c_uint,
        isRequiredProperty: BOOL,
        isInstanceProperty: BOOL,
    );
    pub fn protocol_addProtocol(proto: *mut objc_protocol, addition: *mut objc_protocol);
    pub fn protocol_conformsToProtocol(
        proto: *mut objc_protocol,
        other: *mut objc_protocol,
    ) -> BOOL;
    pub fn protocol_copyMethodDescriptionList(
        proto: *mut objc_protocol,
        isRequiredMethod: BOOL,
        isInstanceMethod: BOOL,
        outCount: *mut c_uint,
    ) -> *mut objc_method_description;
    pub fn protocol_copyPropertyList(
        proto: *mut objc_protocol,
        outCount: *mut c_uint,
    ) -> *mut objc_property_t;
    pub fn protocol_copyProtocolList(
        proto: *mut objc_protocol,
        outCount: *mut c_uint,
    ) -> *mut *mut objc_protocol;
    pub fn protocol_getMethodDescription(
        proto: *mut objc_protocol,
        aSel: *const objc_selector,
        isRequiredMethod: BOOL,
        isInstanceMethod: BOOL,
    ) -> objc_method_description;
    pub fn protocol_getName(proto: *mut objc_protocol) -> *const c_char;
    pub fn protocol_getProperty(
        proto: *mut objc_protocol,
        name: *const c_char,
        isRequiredProperty: BOOL,
        isInstanceProperty: BOOL,
    ) -> objc_property_t;
    pub fn protocol_isEqual(proto: *mut objc_protocol, other: *mut objc_protocol) -> BOOL;
    pub fn sel_getName(sel: *const objc_selector) -> *const c_char;
    pub fn sel_getUid(str_: *const c_char) -> *const objc_selector;
    pub fn sel_isEqual(lhs: *const objc_selector, rhs: *const objc_selector) -> BOOL;
    pub fn sel_isMapped(sel: *const objc_selector) -> BOOL;
    pub fn sel_registerName(str_: *const c_char) -> *const objc_selector;
}
