#![cfg(feature = "Foundation_NSMutableString")]
use alloc::format;
use alloc::string::ToString;
use core::any::TypeId;
use std::ptr;

use objc2::mutability::CounterpartOrSelf;
use objc2::rc::Id;

use crate::Foundation::{
    self, NSCopying, NSMutableCopying, NSMutableString, NSObjectProtocol, NSString,
};

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
fn test_append() {
    let mut s = NSMutableString::from_str("abc");
    s.appendString(&NSString::from_str("def"));
    *s += &NSString::from_str("ghi");
    assert_eq!(&s.to_string(), "abcdefghi");
}

#[test]
fn test_set() {
    let mut s = NSMutableString::from_str("abc");
    s.setString(&NSString::from_str("def"));
    assert_eq!(&s.to_string(), "def");
}

#[test]
fn test_with_capacity() {
    let mut s = NSMutableString::stringWithCapacity(3);
    *s += &NSString::from_str("abc");
    *s += &NSString::from_str("def");
    assert_eq!(&s.to_string(), "abcdef");
}

#[test]
fn test_copy() {
    use Foundation::{NSCopying, NSMutableCopying};

    let s1 = NSMutableString::from_str("abc");
    let s2 = s1.copy();
    assert_ne!(Id::as_ptr(&s1), Id::as_ptr(&s2).cast());
    assert!(s2.is_kind_of::<NSString>());

    let s3 = s1.mutableCopy();
    assert_ne!(Id::as_ptr(&s1), Id::as_ptr(&s3));
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
fn test_copy_with_zone() {
    let s1 = NSString::from_str("abc");
    let s2 = unsafe { s1.copyWithZone(ptr::null_mut()) };
    assert_eq!(Id::as_ptr(&s1), Id::as_ptr(&s2));
    assert!(s2.is_kind_of::<NSString>());

    let s3 = unsafe { s1.mutableCopyWithZone(ptr::null_mut()) };
    assert_ne!(Id::as_ptr(&s1).cast::<NSMutableString>(), Id::as_ptr(&s3));
    assert!(s3.is_kind_of::<Foundation::NSMutableString>());
}
