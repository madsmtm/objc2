#![cfg(feature = "NSDictionary")]
#![cfg(feature = "NSValue")]
#![cfg(feature = "NSObject")]
use alloc::vec;

use objc2::msg_send;
use objc2::rc::Retained;

use crate::Foundation::{NSMutableDictionary, NSNumber, NSObject};

fn sample_dict() -> Retained<NSMutableDictionary<NSNumber, NSObject>> {
    NSMutableDictionary::from_retained_slice(
        &[
            &*NSNumber::new_i32(1),
            &*NSNumber::new_i32(2),
            &*NSNumber::new_i32(3),
        ],
        &[NSObject::new(), NSObject::new(), NSObject::new()],
    )
}

fn sample_dict_mut() -> Retained<NSMutableDictionary<NSNumber, NSMutableDictionary>> {
    NSMutableDictionary::from_vec(
        &[
            &*NSNumber::new_i32(1),
            &*NSNumber::new_i32(2),
            &*NSNumber::new_i32(3),
        ],
        vec![
            NSMutableDictionary::new(),
            NSMutableDictionary::new(),
            NSMutableDictionary::new(),
        ],
    )
}

#[test]
#[cfg(feature = "NSString")]
fn dict_from_mutable() {
    use crate::{NSMutableString, NSString};

    let _: Retained<NSMutableDictionary<NSString, NSString>> = NSMutableDictionary::from_slice(
        &[&*NSMutableString::from_str("a")],
        &[&**NSMutableString::from_str("b")],
    );
}

#[test]
fn test_new() {
    let dict = NSMutableDictionary::<NSObject, NSObject>::new();
    assert!(dict.is_empty());
}

#[test]
fn test_get_mut() {
    let mut dict = sample_dict_mut();

    assert!(dict.get_mut(&NSNumber::new_i32(1)).is_some());
    assert!(dict.get_mut(&NSNumber::new_i32(2)).is_some());
    assert!(dict.get_mut(&NSNumber::new_i32(4)).is_none());
}

#[test]
fn test_values_mut() {
    let mut dict = sample_dict_mut();
    let vec = dict.values_vec_mut();
    assert_eq!(vec.len(), 3);
}

#[test]
fn test_insert() {
    let mut dict = <NSMutableDictionary<NSNumber, _>>::new();
    assert!(dict
        .insert_id(&NSNumber::new_i32(1), NSObject::new())
        .is_none());
    assert!(dict
        .insert_id(&NSNumber::new_i32(2), NSObject::new())
        .is_none());
    assert!(dict
        .insert_id(&NSNumber::new_i32(3), NSObject::new())
        .is_none());
    assert!(dict
        .insert_id(&NSNumber::new_i32(1), NSObject::new())
        .is_some());
    assert_eq!(dict.len(), 3);
}

#[test]
fn test_remove() {
    let mut dict = sample_dict();
    assert_eq!(dict.len(), 3);
    assert!(dict.remove(&NSNumber::new_i32(1)).is_some());
    assert!(dict.remove(&NSNumber::new_i32(2)).is_some());
    assert!(dict.remove(&NSNumber::new_i32(1)).is_none());
    assert!(dict.remove(&NSNumber::new_i32(4)).is_none());
    assert_eq!(dict.len(), 1);
}

#[test]
fn test_clear() {
    let mut dict = sample_dict();
    assert_eq!(dict.len(), 3);

    dict.removeAllObjects();
    assert!(dict.is_empty());
}

#[test]
#[cfg(feature = "NSArray")]
fn test_to_array() {
    let dict = sample_dict();
    let array = dict.to_array();
    assert_eq!(array.len(), 3);
}

#[test]
#[should_panic = "mutation detected during enumeration"]
#[cfg_attr(
    not(debug_assertions),
    ignore = "enumeration mutation only detected with debug assertions on"
)]
fn test_iter_mutation_detection() {
    let dict = sample_dict();

    let mut iter = dict.keys();
    let _ = iter.next();

    let key: &NSNumber = &NSNumber::new_usize(1);
    let obj: &NSObject = &NSObject::new();
    let _: () = unsafe { msg_send![&dict, setObject: obj, forKey: key] };

    let _ = iter.next();
}
