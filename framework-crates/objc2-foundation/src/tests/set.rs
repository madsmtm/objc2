#![cfg(feature = "NSSet")]
#![cfg(feature = "NSString")]
#![cfg(feature = "NSValue")]
use alloc::vec::Vec;
use alloc::{format, vec};

use crate::Foundation::{ns_string, NSNumber, NSObject, NSSet, NSString};

#[test]
fn test_new() {
    let set = NSSet::<NSObject>::new();
    assert!(set.is_empty());
}

#[test]
fn test_from_vec() {
    let set = NSSet::<NSObject>::from_vec(Vec::new());
    assert!(set.is_empty());

    let strs = ["one", "two", "three"].map(NSString::from_str);
    let set = NSSet::from_vec(strs.to_vec());
    assert!(strs.into_iter().all(|s| set.contains(&s)));

    let nums = [1, 2, 3].map(NSNumber::new_i32);
    let set = NSSet::from_vec(nums.to_vec());
    assert!(nums.into_iter().all(|n| set.contains(&n)));
}

#[test]
fn test_from_slice() {
    let set = NSSet::<NSString>::from_slice(&[]);
    assert!(set.is_empty());

    let strs = [ns_string!("one"), ns_string!("two"), ns_string!("three")];
    let set = NSSet::from_slice(&strs);
    assert!(strs.into_iter().all(|s| set.contains(s)));

    let nums = [1, 2, 3].map(NSNumber::new_i32);
    let set = NSSet::from_id_slice(&nums);
    assert!(nums.into_iter().all(|n| set.contains(&n)));
}

#[test]
fn test_len() {
    let set = NSSet::<NSObject>::new();
    assert!(set.is_empty());

    let set = NSSet::from_slice(&[ns_string!("one"), ns_string!("two"), ns_string!("two")]);
    assert_eq!(set.len(), 2);

    let set = NSSet::from_vec(vec![
        NSNumber::new_i32(1),
        NSNumber::new_i32(2),
        NSNumber::new_i32(3),
    ]);
    assert_eq!(set.len(), 3);
}

#[test]
fn test_get() {
    let set = NSSet::<NSString>::new();
    assert!(set.get(ns_string!("one")).is_none());

    let set = NSSet::from_slice(&[ns_string!("one"), ns_string!("two"), ns_string!("two")]);
    assert!(set.get(ns_string!("two")).is_some());
    assert!(set.get(ns_string!("three")).is_none());
}

#[test]
fn test_get_return_lifetime() {
    let set = NSSet::from_slice(&[ns_string!("one"), ns_string!("two"), ns_string!("two")]);

    let res = {
        let value = NSString::from_str("one");
        set.get(&value)
    };

    assert_eq!(res, Some(ns_string!("one")));
}

#[test]
fn test_get_any() {
    let set = NSSet::<NSObject>::new();
    assert!(set.get_any().is_none());

    let strs = [ns_string!("one"), ns_string!("two"), ns_string!("three")];
    let set = NSSet::from_slice(&strs);
    let any = set.get_any().unwrap();
    assert!(any == strs[0] || any == strs[1] || any == strs[2]);
}

#[test]
fn test_contains() {
    let set = NSSet::<NSString>::new();
    assert!(!set.contains(ns_string!("one")));

    let set = NSSet::from_slice(&[ns_string!("one"), ns_string!("two"), ns_string!("two")]);
    assert!(set.contains(ns_string!("one")));
    assert!(!set.contains(ns_string!("three")));
}

#[test]
fn test_is_subset() {
    let set1 = NSSet::from_slice(&[ns_string!("one"), ns_string!("two")]);
    let set2 = NSSet::from_slice(&[ns_string!("one"), ns_string!("two"), ns_string!("three")]);

    assert!(set1.is_subset(&set2));
    assert!(!set2.is_subset(&set1));
}

#[test]
fn test_is_superset() {
    let set1 = NSSet::from_slice(&[ns_string!("one"), ns_string!("two")]);
    let set2 = NSSet::from_slice(&[ns_string!("one"), ns_string!("two"), ns_string!("three")]);

    assert!(!set1.is_superset(&set2));
    assert!(set2.is_superset(&set1));
}

#[test]
fn test_is_disjoint() {
    let set1 = NSSet::from_slice(&[ns_string!("one"), ns_string!("two")]);
    let set2 = NSSet::from_slice(&[ns_string!("one"), ns_string!("two"), ns_string!("three")]);
    let set3 = NSSet::from_slice(&[ns_string!("four"), ns_string!("five"), ns_string!("six")]);

    assert!(!set1.is_disjoint(&set2));
    assert!(set1.is_disjoint(&set3));
    assert!(set2.is_disjoint(&set3));
}

#[test]
#[cfg(feature = "NSArray")]
fn test_to_array() {
    let nums = [1, 2, 3];
    let set = NSSet::from_id_slice(&nums.map(NSNumber::new_i32));

    assert_eq!(set.to_array().len(), 3);
    assert!(set.to_array().iter().all(|i| nums.contains(&i.as_i32())));
}

#[test]
fn test_iter() {
    let nums = [1, 2, 3];
    let set = NSSet::from_id_slice(&nums.map(NSNumber::new_i32));

    assert_eq!(set.iter().count(), 3);
    assert!(set.iter().all(|i| nums.contains(&i.as_i32())));
}

#[test]
fn test_into_iter() {
    let nums = [1, 2, 3];
    let set = NSSet::from_id_slice(&nums.map(NSNumber::new_i32));

    assert!(set.into_iter().all(|i| nums.contains(&i.as_i32())));
}

#[test]
fn test_into_vec() {
    let strs = vec![ns_string!("one"), ns_string!("two"), ns_string!("three")];
    let set = NSSet::from_slice(&strs);

    assert_eq!(set.len(), 3);
    assert_eq!(set.to_vec().len(), 3);
}

#[test]
fn test_equality() {
    let set1 = NSSet::<NSObject>::new();
    let set2 = NSSet::<NSObject>::new();
    assert_eq!(set1, set2);
}

#[test]
#[cfg(feature = "NSObject")]
fn test_copy() {
    use crate::NSCopying;

    let set1 = NSSet::from_slice(&[ns_string!("one"), ns_string!("two"), ns_string!("three")]);
    let set2 = set1.copy();
    assert_eq!(set1, set2);
}

#[test]
fn test_debug() {
    let set = NSSet::<NSObject>::new();
    assert_eq!(format!("{set:?}"), "{}");

    let set = NSSet::from_slice(&[ns_string!("one"), ns_string!("two")]);
    assert!(matches!(
        &*format!("{set:?}"),
        "{\"one\", \"two\"}" | "{\"two\", \"one\"}"
    ));
}

/// This currently works, but we should figure out a way to disallow it!
#[test]
#[cfg(all(feature = "NSArray", feature = "NSCalendar"))]
#[allow(deprecated)]
fn invalid_generic() {
    let something_interior_mutable = unsafe { crate::NSCalendar::currentCalendar() };
    let set = NSSet::from_id_slice(&[crate::NSArray::from_id_slice(&[something_interior_mutable])]);
    let _ = set.get_any().unwrap().get(0).unwrap();
    // something_interior_mutable.setAbc(...)
}

#[test]
fn new_from_nsobject() {
    let _ = NSSet::from_vec(vec![NSObject::new()]);
}
