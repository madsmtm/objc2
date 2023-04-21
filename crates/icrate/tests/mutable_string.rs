#![cfg(feature = "Foundation_NSMutableString")]
use objc2::rc::Id;

use icrate::Foundation::{self, NSMutableString, NSObjectProtocol, NSString};

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
