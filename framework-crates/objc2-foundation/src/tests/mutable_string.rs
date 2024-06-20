#![cfg(feature = "NSString")]
use alloc::format;
use alloc::string::{String, ToString};
use core::any::TypeId;
use core::fmt::Write;
use objc2::runtime::NSObject;

use objc2::mutability::CounterpartOrSelf;

use crate::Foundation::{NSMutableString, NSString};

#[test]
fn display_debug() {
    let s = NSMutableString::from_str("test\"123");
    assert_eq!(format!("{s}"), "test\"123");
    assert_eq!(format!("{s:?}"), r#""test\"123""#);
}

#[test]
fn test_from_nsstring() {
    let s = NSString::from_str("abc");
    let s = NSMutableString::stringWithString(&s);
    assert_eq!(&s.to_string(), "abc");
}

#[test]
#[allow(clippy::deref_addrof)]
fn test_append() {
    let s = NSMutableString::from_str("abc");
    s.appendString(&NSString::from_str("def"));
    *&mut &*s += &NSString::from_str("ghi");
    assert_eq!(&s.to_string(), "abcdefghi");
}

#[test]
fn test_set() {
    let s = NSMutableString::from_str("abc");
    s.setString(&NSString::from_str("def"));
    assert_eq!(&s.to_string(), "def");
}

#[test]
#[allow(clippy::deref_addrof)]
fn test_with_capacity() {
    let s = NSMutableString::stringWithCapacity(3);
    *&mut &*s += &NSString::from_str("abc");
    *&mut &*s += &NSString::from_str("def");
    assert_eq!(&s.to_string(), "abcdef");
}

#[test]
#[cfg(feature = "NSObject")]
fn test_copy() {
    use crate::Foundation::{NSCopying, NSMutableCopying, NSObjectProtocol};
    use objc2::rc::Retained;

    let s1 = NSMutableString::from_str("abc");
    let s2 = s1.copy();
    assert_ne!(Retained::as_ptr(&s1), Retained::as_ptr(&s2).cast());
    assert!(s2.is_kind_of::<NSString>());

    let s3 = s1.mutableCopy();
    assert_ne!(Retained::as_ptr(&s1), Retained::as_ptr(&s3));
    assert!(s3.is_kind_of::<NSMutableString>());
}

#[test]
fn counterpart() {
    assert_eq!(
        TypeId::of::<<NSString as CounterpartOrSelf>::Immutable>(),
        TypeId::of::<NSString>(),
    );
    assert_eq!(
        TypeId::of::<<NSString as CounterpartOrSelf>::Mutable>(),
        TypeId::of::<NSMutableString>(),
    );

    assert_eq!(
        TypeId::of::<<NSMutableString as CounterpartOrSelf>::Immutable>(),
        TypeId::of::<NSString>(),
    );
    assert_eq!(
        TypeId::of::<<NSMutableString as CounterpartOrSelf>::Mutable>(),
        TypeId::of::<NSMutableString>(),
    );
}

#[test]
#[cfg(all(feature = "NSObject", feature = "NSZone"))]
fn test_copy_with_zone() {
    use crate::Foundation::{NSCopying, NSMutableCopying, NSObjectProtocol};
    use objc2::rc::Retained;

    let s1 = NSString::from_str("abc");
    let s2 = unsafe { s1.copyWithZone(core::ptr::null_mut()) };
    assert_eq!(Retained::as_ptr(&s1), Retained::as_ptr(&s2));
    assert!(s2.is_kind_of::<NSString>());

    let s3 = unsafe { s1.mutableCopyWithZone(core::ptr::null_mut()) };
    assert_ne!(
        Retained::as_ptr(&s1).cast::<NSMutableString>(),
        Retained::as_ptr(&s3)
    );
    assert!(s3.is_kind_of::<NSMutableString>());
}

/// Test that writing a NSMutableString to itself works (i.e. ensure that
/// `NSString`'s `Debug` implementation is correct).
#[test]
fn write_same_string() {
    let string = NSMutableString::from_str("foo");
    let mut expected = String::from("foo");

    for i in 0..3 {
        write!(&mut &*string, "{string}").unwrap();
        write!(&mut &*string, "{string:?}").unwrap();
        // Test `NSObject`'s Debug impl as well.
        let object: &NSObject = &string;
        write!(&mut &*string, "{object:?}").unwrap();

        let expected_clone = expected.clone();
        write!(&mut expected, "{expected_clone}").unwrap();
        let expected_clone = expected.clone();
        write!(&mut expected, "{expected_clone:?}").unwrap();
        let expected_clone = expected.clone();
        write!(&mut expected, "{expected_clone}").unwrap();

        assert_eq!(string.to_string(), expected, "failed at iteration {i}");
    }
}

/// Test that appending a NSMutableString to itself works (i.e. ensure that
/// `appendString` is sound).
#[test]
fn append_same_string() {
    let string = NSMutableString::from_str("foo");
    let mut expected = String::from("foo");
    for i in 0..3 {
        string.appendString(&string);
        expected += &*expected.clone();

        assert_eq!(string.to_string(), expected, "failed at iteration {i}");
    }
}
