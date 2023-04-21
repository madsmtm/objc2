#![cfg(feature = "Foundation_NSMutableSet")]
#![cfg(feature = "Foundation_NSString")]
use objc2::rc::{__RcTestObject, __ThreadTestData};

use icrate::ns_string;
use icrate::Foundation::{self, NSMutableSet, NSSet, NSString};

#[test]
fn test_insert() {
    let mut set = NSMutableSet::new();
    assert!(set.is_empty());

    assert!(set.insert(NSString::from_str("one")));
    assert!(!set.insert(NSString::from_str("one")));
    assert!(set.insert(NSString::from_str("two")));
}

#[test]
fn test_remove() {
    let strs = ["one", "two", "three"].map(NSString::from_str);
    let mut set = NSMutableSet::from_id_slice(&strs);

    assert!(set.remove(ns_string!("one")));
    assert!(!set.remove(ns_string!("one")));
}

#[test]
fn test_clear() {
    let strs = ["one", "two", "three"].map(NSString::from_str);
    let mut set = NSMutableSet::from_id_slice(&strs);
    assert_eq!(set.len(), 3);

    set.removeAllObjects();
    assert!(set.is_empty());
}

#[test]
#[cfg(feature = "Foundation_NSMutableString")]
fn test_into_vec() {
    let strs = vec![
        Foundation::NSMutableString::from_str("one"),
        Foundation::NSMutableString::from_str("two"),
        Foundation::NSMutableString::from_str("three"),
    ];
    let set = NSMutableSet::from_vec(strs);

    let mut vec = NSMutableSet::into_vec(set);
    for str in vec.iter_mut() {
        str.appendString(ns_string!(" times zero is zero"));
    }

    assert_eq!(vec.len(), 3);
    let suffix = ns_string!("zero");
    assert!(vec.iter().all(|str| str.hasSuffix(suffix)));
}

#[test]
fn test_extend() {
    let mut set = NSMutableSet::new();
    assert!(set.is_empty());

    set.extend(["one", "two", "three"].map(NSString::from_str));
    assert_eq!(set.len(), 3);
}

#[test]
fn test_mutable_copy() {
    use Foundation::NSMutableCopying;

    let set1 = NSSet::from_id_slice(&["one", "two", "three"].map(NSString::from_str));
    let mut set2 = set1.mutableCopy();
    set2.insert(NSString::from_str("four"));

    assert!(set1.is_subset(&set2));
    assert_ne!(set1.mutableCopy(), set2);
}

#[test]
fn test_insert_retain_release() {
    let mut set = NSMutableSet::new();
    let obj1 = __RcTestObject::new();
    let obj2 = __RcTestObject::new();
    let mut expected = __ThreadTestData::current();

    set.insert(obj1);
    expected.retain += 1;
    expected.release += 1;
    expected.assert_current();
    assert_eq!(set.len(), 1);
    assert_eq!(set.get_any(), set.get_any());

    set.insert(obj2);
    expected.retain += 1;
    expected.release += 1;
    expected.assert_current();
    assert_eq!(set.len(), 2);
}

#[test]
fn test_clear_release_dealloc() {
    let mut set = NSMutableSet::new();
    for _ in 0..4 {
        set.insert(__RcTestObject::new());
    }
    let mut expected = __ThreadTestData::current();

    set.removeAllObjects();
    expected.release += 4;
    expected.dealloc += 4;
    expected.assert_current();
    assert_eq!(set.len(), 0);
}
