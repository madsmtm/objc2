#![cfg(feature = "NSSet")]
#![cfg(feature = "NSString")]
use alloc::vec;

use crate::Foundation::{ns_string, NSMutableSet, NSMutableString};

#[test]
fn test_insert() {
    let mut set = NSMutableSet::new();
    assert!(set.is_empty());

    assert!(set.insert(ns_string!("one")));
    assert!(!set.insert(ns_string!("one")));
    assert!(set.insert(ns_string!("two")));
}

#[test]
fn test_remove() {
    let strs = [ns_string!("one"), ns_string!("two"), ns_string!("three")];
    let mut set = NSMutableSet::from_slice(&strs);

    assert!(set.remove(ns_string!("one")));
    assert!(!set.remove(ns_string!("one")));
}

#[test]
fn test_clear() {
    let strs = [ns_string!("one"), ns_string!("two"), ns_string!("three")];
    let mut set = NSMutableSet::from_slice(&strs);
    assert_eq!(set.len(), 3);

    set.removeAllObjects();
    assert!(set.is_empty());
}

#[test]
fn test_into_vec() {
    let strs = vec![
        NSMutableString::from_str("one"),
        NSMutableString::from_str("two"),
        NSMutableString::from_str("three"),
    ];
    let set = NSMutableSet::from_vec(strs);

    let mut vec = NSMutableSet::into_vec(set);
    for str in vec.iter_mut() {
        str.appendString(ns_string!(" times zero is zero"));
    }

    assert_eq!(vec.len(), 3);
    assert!(vec.iter().all(|str| str.hasSuffix(ns_string!("zero"))));
}

#[test]
fn test_extend() {
    let mut set = NSMutableSet::new();
    assert!(set.is_empty());

    set.extend([ns_string!("one"), ns_string!("two"), ns_string!("three")]);
    assert_eq!(set.len(), 3);
}

#[test]
#[cfg(feature = "NSObject")]
fn test_mutable_copy() {
    use crate::{NSMutableCopying, NSSet};

    let set1 = NSSet::from_slice(&[ns_string!("one"), ns_string!("two"), ns_string!("three")]);
    let mut set2 = set1.mutableCopy();
    set2.insert(ns_string!("four"));

    assert!(set1.is_subset(&set2));
    assert_ne!(set1.mutableCopy(), set2);
}
