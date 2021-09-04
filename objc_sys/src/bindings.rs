use core::ffi::c_void;
use std::os::raw::{c_char, c_int, c_uint};

use crate::*;

extern "C" {
    pub fn class_addIvar(
        cls: Class,
        name: *const c_char,
        size: usize,
        alignment: u8,
        types: *const c_char,
    ) -> BOOL;
    pub fn class_addMethod(cls: Class, name: SEL, imp: IMP, types: *const c_char) -> BOOL;
    pub fn class_addProperty(
        cls: Class,
        name: *const c_char,
        attributes: *const objc_property_attribute_t,
        attributeCount: c_uint,
    ) -> BOOL;
    pub fn class_addProtocol(cls: Class, protocol: *mut Protocol) -> BOOL;
    pub fn class_conformsToProtocol(cls: Class, protocol: *mut Protocol) -> BOOL;
    pub fn class_copyIvarList(cls: Class, outCount: *mut c_uint) -> *mut Ivar;
    pub fn class_copyMethodList(cls: Class, outCount: *mut c_uint) -> *mut Method;
    pub fn class_copyPropertyList(cls: Class, outCount: *mut c_uint) -> *mut objc_property_t;
    pub fn class_copyProtocolList(cls: Class, outCount: *mut c_uint) -> *mut *mut Protocol;
    pub fn class_createInstance(cls: Class, extraBytes: usize) -> id;
    pub fn class_getClassMethod(cls: Class, name: SEL) -> Method;
    pub fn class_getClassVariable(cls: Class, name: *const c_char) -> Ivar;
    pub fn class_getImageName(cls: Class) -> *const c_char;
    pub fn class_getInstanceMethod(cls: Class, name: SEL) -> Method;
    pub fn class_getInstanceSize(cls: Class) -> usize;
    pub fn class_getInstanceVariable(cls: Class, name: *const c_char) -> Ivar;
    pub fn class_getIvarLayout(cls: Class) -> *const u8;
    pub fn class_getName(cls: Class) -> *const c_char;
    pub fn class_getProperty(cls: Class, name: *const c_char) -> objc_property_t;
    pub fn class_getSuperclass(cls: Class) -> Class;
    pub fn class_getVersion(cls: Class) -> c_int;
    pub fn class_getWeakIvarLayout(cls: Class) -> *const u8;
    pub fn class_isMetaClass(cls: Class) -> BOOL;
    pub fn class_replaceMethod(cls: Class, name: SEL, imp: IMP, types: *const c_char) -> IMP;
    pub fn class_replaceProperty(
        cls: Class,
        name: *const c_char,
        attributes: *const objc_property_attribute_t,
        attributeCount: c_uint,
    );
    pub fn class_respondsToSelector(cls: Class, sel: SEL) -> BOOL;
    pub fn class_setIvarLayout(cls: Class, layout: *const u8);
    pub fn class_setVersion(cls: Class, version: c_int);
    pub fn class_setWeakIvarLayout(cls: Class, layout: *const u8);
    pub fn imp_getBlock(anImp: IMP) -> id;
    pub fn imp_implementationWithBlock(block: id) -> IMP;
    pub fn imp_removeBlock(anImp: IMP) -> BOOL;
    pub fn ivar_getName(v: Ivar) -> *const c_char;
    pub fn ivar_getOffset(v: Ivar) -> isize;
    pub fn ivar_getTypeEncoding(v: Ivar) -> *const c_char;
    pub fn method_copyArgumentType(m: Method, index: c_uint) -> *mut c_char;
    pub fn method_copyReturnType(m: Method) -> *mut c_char;
    pub fn method_exchangeImplementations(m1: Method, m2: Method);
    pub fn method_getArgumentType(m: Method, index: c_uint, dst: *mut c_char, dst_len: usize);
    pub fn method_getDescription(m: Method) -> *mut objc_method_description;
    pub fn method_getImplementation(m: Method) -> IMP;
    pub fn method_getName(m: Method) -> SEL;
    pub fn method_getNumberOfArguments(m: Method) -> c_uint;
    pub fn method_getReturnType(m: Method, dst: *mut c_char, dst_len: usize);
    pub fn method_getTypeEncoding(m: Method) -> *const c_char;
    pub fn method_setImplementation(m: Method, imp: IMP) -> IMP;
    pub fn objc_allocateClassPair(
        superclass: Class,
        name: *const c_char,
        extraBytes: usize,
    ) -> Class;
    pub fn objc_allocateProtocol(name: *const c_char) -> *mut Protocol;
    pub fn objc_begin_catch(exc_buf: *mut c_void) -> id;
    pub fn objc_copyClassList(outCount: *mut c_uint) -> *mut Class;
    pub fn objc_copyClassNamesForImage(
        image: *const c_char,
        outCount: *mut c_uint,
    ) -> *mut *const c_char;
    pub fn objc_copyImageNames(outCount: *mut c_uint) -> *mut *const c_char;
    pub fn objc_copyProtocolList(outCount: *mut c_uint) -> *mut *mut Protocol;
    pub fn objc_disposeClassPair(cls: Class);
    pub fn objc_duplicateClass(original: Class, name: *const c_char, extraBytes: usize) -> Class;
    pub fn objc_end_catch();
    pub fn objc_enumerationMutation(obj: id);
    pub fn objc_exception_rethrow();
    pub fn objc_exception_throw(exception: id);
    pub fn objc_getAssociatedObject(object: id, key: *const c_void) -> id;
    pub fn objc_getClass(name: *const c_char) -> Class;
    pub fn objc_getClassList(buffer: *mut Class, bufferCount: c_int) -> c_int;
    pub fn objc_getMetaClass(name: *const c_char) -> Class;
    pub fn objc_getProtocol(name: *const c_char) -> *mut Protocol;
    pub fn objc_getRequiredClass(name: *const c_char) -> Class;
    pub fn objc_lookUpClass(name: *const c_char) -> Class;
    pub fn objc_registerClassPair(cls: Class);
    pub fn objc_registerProtocol(proto: *mut Protocol);
    pub fn objc_removeAssociatedObjects(object: id);
    pub fn objc_setAssociatedObject(
        object: id,
        key: *const c_void,
        value: id,
        policy: objc_AssociationPolicy,
    );
    pub fn objc_setEnumerationMutationHandler(handler: Option<unsafe extern "C" fn(arg1: id)>);
    pub fn objc_setExceptionMatcher(fn_: objc_exception_matcher) -> objc_exception_matcher;
    pub fn objc_setExceptionPreprocessor(
        fn_: objc_exception_preprocessor,
    ) -> objc_exception_preprocessor;
    pub fn objc_setForwardHandler(fwd: *mut c_void, fwd_stret: *mut c_void);
    pub fn objc_setUncaughtExceptionHandler(
        fn_: objc_uncaught_exception_handler,
    ) -> objc_uncaught_exception_handler;
    pub fn objc_sync_enter(obj: id) -> c_int;
    pub fn objc_sync_exit(obj: id) -> c_int;
    pub fn object_getClass(obj: id) -> Class;
    pub fn object_getClassName(obj: id) -> *const c_char;
    pub fn object_getIndexedIvars(obj: id) -> *mut c_void;
    pub fn object_getIvar(obj: id, ivar: Ivar) -> id;
    pub fn object_setClass(obj: id, cls: Class) -> Class;
    pub fn object_setIvar(obj: id, ivar: Ivar, value: id);
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
        proto: *mut Protocol,
        name: SEL,
        types: *const c_char,
        isRequiredMethod: BOOL,
        isInstanceMethod: BOOL,
    );
    pub fn protocol_addProperty(
        proto: *mut Protocol,
        name: *const c_char,
        attributes: *const objc_property_attribute_t,
        attributeCount: c_uint,
        isRequiredProperty: BOOL,
        isInstanceProperty: BOOL,
    );
    pub fn protocol_addProtocol(proto: *mut Protocol, addition: *mut Protocol);
    pub fn protocol_conformsToProtocol(proto: *mut Protocol, other: *mut Protocol) -> BOOL;
    pub fn protocol_copyMethodDescriptionList(
        proto: *mut Protocol,
        isRequiredMethod: BOOL,
        isInstanceMethod: BOOL,
        outCount: *mut c_uint,
    ) -> *mut objc_method_description;
    pub fn protocol_copyPropertyList(
        proto: *mut Protocol,
        outCount: *mut c_uint,
    ) -> *mut objc_property_t;
    pub fn protocol_copyProtocolList(
        proto: *mut Protocol,
        outCount: *mut c_uint,
    ) -> *mut *mut Protocol;
    pub fn protocol_getMethodDescription(
        proto: *mut Protocol,
        aSel: SEL,
        isRequiredMethod: BOOL,
        isInstanceMethod: BOOL,
    ) -> objc_method_description;
    pub fn protocol_getName(proto: *mut Protocol) -> *const c_char;
    pub fn protocol_getProperty(
        proto: *mut Protocol,
        name: *const c_char,
        isRequiredProperty: BOOL,
        isInstanceProperty: BOOL,
    ) -> objc_property_t;
    pub fn protocol_isEqual(proto: *mut Protocol, other: *mut Protocol) -> BOOL;
    pub fn sel_getName(sel: SEL) -> *const c_char;
    pub fn sel_getUid(str_: *const c_char) -> SEL;
    pub fn sel_isEqual(lhs: SEL, rhs: SEL) -> BOOL;
    pub fn sel_isMapped(sel: SEL) -> BOOL;
    pub fn sel_registerName(str_: *const c_char) -> SEL;
}
