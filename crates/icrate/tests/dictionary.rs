#![cfg(feature = "Foundation_NSDictionary")]
#![cfg(feature = "Foundation_NSString")]
use objc2::rc::{autoreleasepool, Id};

use icrate::Foundation::{NSDictionary, NSObject, NSString};

fn sample_dict(key: &str) -> Id<NSDictionary<NSString, NSObject>> {
    let string = NSString::from_str(key);
    let obj = NSObject::new();
    NSDictionary::from_keys_and_objects(&[&*string], vec![obj])
}

#[test]
fn test_len() {
    let dict = sample_dict("abcd");
    assert_eq!(dict.len(), 1);
}

#[test]
fn test_get() {
    let dict = sample_dict("abcd");

    let string = NSString::from_str("abcd");
    assert!(dict.get(&string).is_some());

    let string = NSString::from_str("abcde");
    assert!(dict.get(&string).is_none());
}

#[test]
fn test_keys() {
    let dict = sample_dict("abcd");
    let keys = dict.keys_vec();

    assert_eq!(keys.len(), 1);
    autoreleasepool(|pool| {
        assert_eq!(keys[0].as_str(pool), "abcd");
    });
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
    autoreleasepool(|pool| {
        assert_eq!(keys[0].as_str(pool), "abcd");
    });
    assert_eq!(objs[0], dict.get(keys[0]).unwrap());
}

#[test]
#[cfg(feature = "Foundation_NSEnumerator")]
fn test_iter_keys() {
    let dict = sample_dict("abcd");
    assert_eq!(dict.keys().count(), 1);
    autoreleasepool(|pool| {
        assert_eq!(dict.keys().next().unwrap().as_str(pool), "abcd");
    });
}

#[test]
#[cfg(feature = "Foundation_NSEnumerator")]
fn test_iter_values() {
    let dict = sample_dict("abcd");
    assert_eq!(dict.values().count(), 1);
}

#[test]
#[cfg(feature = "Foundation_NSArray")]
fn test_arrays() {
    let dict = sample_dict("abcd");

    let keys = unsafe { dict.allKeys() };
    assert_eq!(keys.len(), 1);
    autoreleasepool(|pool| {
        assert_eq!(keys[0].as_str(pool), "abcd");
    });

    // let objs = NSDictionary::into_values_array(dict);
    // assert_eq!(objs.len(), 1);
}

#[test]
fn test_debug() {
    let key = NSString::from_str("a");
    let val = NSString::from_str("b");
    let dict = NSDictionary::from_keys_and_objects(&[&*key], vec![val]);
    assert_eq!(format!("{dict:?}"), r#"{"a": "b"}"#);
}
