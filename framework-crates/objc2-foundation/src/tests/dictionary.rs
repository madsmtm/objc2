#![cfg(feature = "NSDictionary")]
#![cfg(feature = "NSString")]
#![cfg(feature = "NSObject")]
use alloc::string::ToString;
use alloc::{format, vec};
use core::ptr;

use objc2::{
    ffi, msg_send,
    rc::Retained,
    runtime::{AnyClass, AnyObject, Bool, ClassBuilder, Sel},
    sel,
};

use crate::{ns_string, NSDictionary, NSObject, NSString, NSUInteger};

fn sample_dict(key: &str) -> Retained<NSDictionary<NSString, NSObject>> {
    let string = NSString::from_str(key);
    let obj = NSObject::new();
    NSDictionary::from_vec(&[&*string], vec![obj])
}

#[test]
fn test_len() {
    let dict = sample_dict("abcd");
    assert_eq!(dict.len(), 1);
}

#[test]
fn test_get() {
    let dict = sample_dict("abcd");

    assert!(dict.get(ns_string!("abcd")).is_some());
    assert!(dict.get(ns_string!("abcde")).is_none());
}

#[test]
fn test_keys() {
    let dict = sample_dict("abcd");
    let keys = dict.keys_vec();

    assert_eq!(keys.len(), 1);
    assert_eq!(keys[0].to_string(), "abcd");
}

#[test]
fn test_values() {
    let dict = sample_dict("abcd");
    let vals = dict.values_vec();

    assert_eq!(vals.len(), 1);
}

#[test]
fn test_keys_and_objects() {
    let dict = sample_dict("abcd");
    let (keys, objs) = dict.to_vecs();

    assert_eq!(keys.len(), 1);
    assert_eq!(objs.len(), 1);
    assert_eq!(keys[0].to_string(), "abcd");
    assert_eq!(objs[0], dict.get(keys[0]).unwrap());
}

#[test]
#[cfg(feature = "NSEnumerator")]
fn test_iter_keys() {
    let dict = sample_dict("abcd");
    assert_eq!(dict.keys().count(), 1);
    assert_eq!(dict.keys().next().unwrap().to_string(), "abcd");
}

#[test]
#[cfg(feature = "NSEnumerator")]
fn test_iter_values() {
    let dict = sample_dict("abcd");
    assert_eq!(dict.values().count(), 1);
}

#[test]
#[cfg(feature = "NSArray")]
fn test_arrays() {
    let dict = sample_dict("abcd");

    let keys = unsafe { dict.allKeys() };
    assert_eq!(keys.len(), 1);
    assert_eq!(keys[0].to_string(), "abcd");

    // let objs = dict.to_array();
    // assert_eq!(objs.len(), 1);
}

#[test]
fn test_debug() {
    let key = ns_string!("a");
    let val = ns_string!("b");
    let dict = NSDictionary::from_slice(&[key], &[val]);
    assert_eq!(format!("{dict:?}"), r#"{"a": "b"}"#);
}

#[test]
fn new_different_lengths() {
    let dict = NSDictionary::from_retained_slice(
        &[ns_string!("a"), ns_string!("b"), ns_string!("c")],
        &[NSObject::new(), NSObject::new()],
    );
    assert_eq!(dict.len(), 2);
}

fn base_class_builder(name: &str) -> Option<ClassBuilder> {
    extern "C" fn initialize(_cls: &AnyClass, _sel: Sel) {}
    let mut builder = ClassBuilder::root(name, initialize as extern "C" fn(_, _))?;

    extern "C" fn new(cls: &AnyClass, _sel: Sel) -> *mut AnyObject {
        unsafe { ffi::class_createInstance((cls as *const AnyClass).cast(), 0).cast() }
    }
    unsafe {
        builder.add_class_method(sel!(new), new as extern "C" fn(_, _) -> _);
    }

    extern "C" fn does_not_recognize_selector(obj: &AnyObject, _sel: Sel, sel: Sel) {
        panic!("does not recognize: -[{} {sel}]", obj.class());
    }
    unsafe {
        builder.add_method(
            sel!(doesNotRecognizeSelector:),
            does_not_recognize_selector as extern "C" fn(_, _, _),
        );
    }

    extern "C" fn forward_invocation(obj: &AnyObject, _sel: Sel, invocation: &AnyObject) {
        let sel: Sel = unsafe { msg_send![invocation, selector] };
        panic!("does not recognize: -[{} {sel}]", obj.class());
    }
    unsafe {
        builder.add_method(
            sel!(forwardInvocation:),
            forward_invocation as extern "C" fn(_, _, _),
        );
    }

    extern "C" fn release(obj: *mut AnyObject, _sel: Sel) {
        #[allow(deprecated)]
        unsafe {
            ffi::object_dispose(obj.cast());
        }
    }
    unsafe {
        builder.add_method(sel!(release), release as extern "C" fn(_, _));
    }

    Some(builder)
}

fn test_from_base_class(cls: &AnyClass) {
    // Fake type-alias to an object that implements `NSCopying`
    type Base = NSString;

    let obj1: Retained<Base> = unsafe {
        Retained::from_raw(ffi::class_createInstance((cls as *const AnyClass).cast(), 0).cast())
            .unwrap()
    };
    let obj2: Retained<Base> = unsafe {
        Retained::from_raw(ffi::class_createInstance((cls as *const AnyClass).cast(), 0).cast())
            .unwrap()
    };

    let _dict =
        NSDictionary::from_retained_slice(&[&*obj1, &*obj2], &[NSObject::new(), NSObject::new()]);
}

#[test]
#[should_panic = "does not recognize: -[NoCopy copyWithZone:]"]
#[cfg_attr(
    feature = "gnustep-1-7",
    ignore = "GNUStep stack overflows here for some reason?"
)]
fn no_copy() {
    let mut builder = base_class_builder("NoCopy").unwrap();

    extern "C" fn hash(obj: &AnyObject, _sel: Sel) -> NSUInteger {
        obj as *const AnyObject as NSUInteger
    }
    unsafe {
        builder.add_method(sel!(hash), hash as extern "C" fn(_, _) -> _);
    }

    extern "C" fn is_equal(obj: &AnyObject, _sel: Sel, other: &AnyObject) -> Bool {
        ptr::eq(obj, other).into()
    }
    unsafe {
        builder.add_method(sel!(isEqual:), is_equal as extern "C" fn(_, _, _) -> _);
    }

    let cls = builder.register();
    test_from_base_class(cls);
}

#[test]
#[should_panic = "does not recognize: -[NoIsEqualHash hash]"]
#[cfg(feature = "NSZone")]
#[cfg_attr(
    feature = "gnustep-1-7",
    ignore = "GNUStep stack overflows here for some reason?"
)]
fn no_is_equal_hash() {
    let mut builder = base_class_builder("NoIsEqualHash").unwrap();

    extern "C" fn copy_with_zone(
        obj: &AnyObject,
        _sel: Sel,
        _zone: *mut crate::NSZone,
    ) -> *mut AnyObject {
        unsafe { ffi::class_createInstance((obj.class() as *const AnyClass).cast(), 0).cast() }
    }
    unsafe {
        builder.add_method(
            sel!(copyWithZone:),
            copy_with_zone as extern "C" fn(_, _, _) -> _,
        );
    }

    let cls = builder.register();
    test_from_base_class(cls);
}

#[test]
#[cfg(feature = "NSZone")]
#[cfg_attr(
    feature = "gnustep-1-7",
    ignore = "GNUStep stack overflows here for some reason?"
)]
fn root_object() {
    let mut builder = base_class_builder("RootObject").unwrap();

    extern "C" fn copy_with_zone(
        obj: &AnyObject,
        _sel: Sel,
        _zone: *mut crate::NSZone,
    ) -> *mut AnyObject {
        unsafe { msg_send![obj.class(), new] }
    }
    unsafe {
        builder.add_method(
            sel!(copyWithZone:),
            copy_with_zone as extern "C" fn(_, _, _) -> _,
        );
    }

    extern "C" fn hash(obj: &AnyObject, _sel: Sel) -> NSUInteger {
        obj as *const AnyObject as NSUInteger
    }
    unsafe {
        builder.add_method(sel!(hash), hash as extern "C" fn(_, _) -> _);
    }

    extern "C" fn is_equal(obj: &AnyObject, _sel: Sel, other: &AnyObject) -> Bool {
        ptr::eq(obj, other).into()
    }
    unsafe {
        builder.add_method(sel!(isEqual:), is_equal as extern "C" fn(_, _, _) -> _);
    }

    let cls = builder.register();
    test_from_base_class(cls);
}
