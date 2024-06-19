#![cfg(feature = "all")]
use std::ptr;
use std::vec::Vec;

use objc2::mutability::InteriorMutable;
use objc2::rc::Retained;
use objc2::{declare_class, extern_methods, ClassType, DeclaredClass};
use objc2_foundation::{
    NSArray, NSCopying, NSMutableArray, NSMutableCopying, NSMutableDictionary, NSMutableSet,
    NSNumber, NSSet, NSValue,
};

use crate::rc_test_object::{RcTestObject, ThreadTestData};

#[test]
fn array_retains_stored() {
    let obj = RcTestObject::new();
    let mut expected = ThreadTestData::current();

    let input = [obj.clone(), obj.clone()];
    expected.retain += 2;
    expected.assert_current();

    let array = NSArray::from_id_slice(&input);
    expected.retain += 2;
    expected.assert_current();

    let _obj = array.first().unwrap();
    expected.assert_current();

    drop(array);
    expected.release += 2;
    expected.assert_current();

    let array = NSArray::from_vec(Vec::from(input));
    expected.retain += 2;
    expected.release += 2;
    expected.assert_current();

    let _obj = array.get(0).unwrap();
    let _obj = array.get(1).unwrap();
    assert!(array.get(2).is_none());
    expected.assert_current();

    drop(array);
    expected.release += 2;
    expected.assert_current();

    drop(obj);
    expected.release += 1;
    expected.drop += 1;
    expected.assert_current();
}

#[test]
fn set_retains_stored() {
    let obj = RcTestObject::new();
    let mut expected = ThreadTestData::current();

    let input = [obj.clone(), obj.clone()];
    expected.retain += 2;
    expected.assert_current();

    let set = NSSet::from_id_slice(&input);
    expected.retain += 1;
    expected.assert_current();

    let _obj = set.get_any().unwrap();
    expected.assert_current();

    drop(set);
    expected.release += 1;
    expected.assert_current();

    let set = NSSet::from_vec(Vec::from(input));
    expected.retain += 1;
    expected.release += 2;
    expected.assert_current();

    drop(set);
    expected.release += 1;
    expected.assert_current();

    drop(obj);
    expected.release += 1;
    expected.drop += 1;
    expected.assert_current();
}

#[test]
fn array_nscopying_uses_retain() {
    let obj = RcTestObject::new();
    let array = NSArray::from_id_slice(&[obj]);
    let mut expected = ThreadTestData::current();

    let _copy = array.copy();
    expected.assert_current();

    let _copy = array.mutableCopy();
    expected.retain += 1;
    expected.assert_current();
}

#[test]
fn set_nscopying_uses_retain() {
    let obj = RcTestObject::new();
    let set = NSSet::from_id_slice(&[obj]);
    let mut expected = ThreadTestData::current();

    let _copy = set.copy();
    expected.assert_current();

    let _copy = set.mutableCopy();
    expected.retain += 1;
    expected.assert_current();
}

#[test]
#[cfg_attr(
    target_vendor = "apple",
    ignore = "this works differently on different framework versions"
)]
fn array_iter_minimal_retains() {
    let objs = [RcTestObject::new()];
    let array = NSArray::from_id_slice(&objs);
    drop(objs);
    let mut expected = ThreadTestData::current();

    // Iter
    let mut iter = array.iter();
    expected.assert_current();

    assert!(iter.next().is_some());
    expected.assert_current();

    assert_eq!(iter.count(), 0);
    expected.assert_current();

    // IterRetained
    let mut iter = array.iter_retained();
    expected.assert_current();

    assert!(iter.next().is_some());
    expected.retain += 1;
    expected.release += 1;
    expected.assert_current();

    assert_eq!(iter.count(), 0);
    expected.assert_current();

    // IntoIter
    let mut iter = array.into_iter();
    expected.assert_current();

    assert!(iter.next().is_some());
    expected.retain += 1;
    expected.release += 1;
    expected.assert_current();

    assert_eq!(iter.count(), 0);
    expected.release += 1;
    expected.drop += 1;
    expected.assert_current();
}

#[test]
#[cfg_attr(
    target_vendor = "apple",
    ignore = "this works differently on different framework versions"
)]
fn set_iter_minimal_retains() {
    let objs = [RcTestObject::new()];
    let set = NSSet::from_id_slice(&objs);
    drop(objs);
    let mut expected = ThreadTestData::current();

    // Iter
    let mut iter = set.iter();
    expected.assert_current();

    assert!(iter.next().is_some());
    expected.assert_current();

    assert_eq!(iter.count(), 0);
    expected.assert_current();

    // IterRetained
    let mut iter = set.iter_retained();
    expected.assert_current();

    assert!(iter.next().is_some());
    expected.retain += 1;
    expected.release += 1;
    expected.assert_current();

    assert_eq!(iter.count(), 0);
    expected.assert_current();

    // IntoIter
    let mut iter = set.into_iter();
    expected.assert_current();

    assert!(iter.next().is_some());
    expected.retain += 1;
    expected.release += 1;
    expected.assert_current();

    assert_eq!(iter.count(), 0);
    expected.release += 1;
    expected.drop += 1;
    expected.assert_current();
}

#[test]
#[cfg_attr(
    feature = "gnustep-1-7",
    ignore = "thread safety issues regarding initialization"
)]
fn array_adding() {
    let mut array = NSMutableArray::new();
    let obj1 = RcTestObject::new();
    let obj2 = RcTestObject::new();
    let mut expected = ThreadTestData::current();

    array.push(obj1);
    expected.retain += 1;
    expected.release += 1;
    expected.assert_current();
    assert_eq!(array.len(), 1);
    assert_eq!(array.get(0), array.get(0));

    array.insert(0, obj2);
    expected.retain += 1;
    expected.release += 1;
    expected.assert_current();
    assert_eq!(array.len(), 2);
}

#[test]
#[cfg_attr(
    feature = "gnustep-1-7",
    ignore = "thread safety issues regarding initialization"
)]
fn array_replace() {
    let mut array = NSMutableArray::new();
    let obj1 = RcTestObject::new();
    let obj2 = RcTestObject::new();
    array.push(obj1);
    let mut expected = ThreadTestData::current();

    let old_obj = array.replace(0, obj2).unwrap();
    expected.retain += 2;
    expected.release += 2;
    expected.assert_current();
    assert_ne!(&*old_obj, array.get(0).unwrap());
}

#[test]
#[cfg_attr(
    feature = "gnustep-1-7",
    ignore = "thread safety issues regarding initialization"
)]
fn array_remove() {
    let mut array = NSMutableArray::new();
    for _ in 0..4 {
        array.push(RcTestObject::new());
    }
    let mut expected = ThreadTestData::current();

    let _obj = array.remove(1);
    expected.retain += 1;
    expected.release += 1;
    expected.assert_current();
    assert_eq!(array.len(), 3);

    let _obj = array.pop();
    expected.retain += 1;
    expected.release += 1;
    expected.assert_current();
    assert_eq!(array.len(), 2);

    array.removeAllObjects();
    expected.release += 2;
    expected.drop += 2;
    expected.assert_current();
    assert_eq!(array.len(), 0);
}

#[test]
fn set_insert_retain_release() {
    let mut set = <NSMutableSet<RcTestObject>>::new();
    let obj1 = RcTestObject::new();
    let obj2 = RcTestObject::new();
    let obj2_copy = obj2.retain();
    let mut expected = ThreadTestData::current();

    set.insert(&obj1);
    // Retain to store in set
    expected.retain += 1;
    expected.assert_current();
    assert_eq!(set.len(), 1);
    assert_eq!(set.get_any(), set.get_any());

    set.insert(&obj2);
    // Retain to store in set
    expected.retain += 1;
    expected.assert_current();
    assert_eq!(set.len(), 2);

    set.insert(&obj2_copy);
    // No retain, since the object is already in the set
    expected.retain += 0;
    expected.assert_current();
    assert_eq!(set.len(), 2);
}

#[test]
fn set_clear_release_dealloc() {
    let mut set = NSMutableSet::new();
    for _ in 0..4 {
        set.insert_id(RcTestObject::new());
    }
    let mut expected = ThreadTestData::current();

    set.removeAllObjects();
    expected.release += 4;
    expected.drop += 4;
    expected.assert_current();
    assert_eq!(set.len(), 0);
}

#[test]
fn value_does_not_retain() {
    let obj = RcTestObject::new();
    let expected = ThreadTestData::current();

    let val = NSValue::new::<*const RcTestObject>(&*obj);
    expected.assert_current();

    assert!(ptr::eq(unsafe { val.get::<*const RcTestObject>() }, &*obj));
    expected.assert_current();

    let _clone = val.clone();
    expected.assert_current();

    let _copy = val.copy();
    expected.assert_current();

    drop(val);
    expected.assert_current();
}

declare_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    struct NSCopyingRcTestObject;

    unsafe impl ClassType for NSCopyingRcTestObject {
        type Super = RcTestObject;
        type Mutability = InteriorMutable;
        const NAME: &'static str = "NSCopyingRcTestObject";
    }

    impl DeclaredClass for NSCopyingRcTestObject {}

    unsafe impl NSCopying for NSCopyingRcTestObject {}
);

extern_methods!(
    unsafe impl NSCopyingRcTestObject {
        #[method_id(new)]
        fn new() -> Retained<Self>;
    }
);

#[test]
fn dictionary_insert_key_copies() {
    let mut dict = <NSMutableDictionary<NSCopyingRcTestObject, _>>::new();
    let key1 = NSCopyingRcTestObject::new();
    let mut expected = ThreadTestData::current();

    let _ = dict.insert_id(&key1, NSNumber::new_i32(1));
    // Create copy
    expected.copy += 1;
    expected.alloc += 1;
    expected.init += 1;
    expected.assert_current();

    dict.removeAllObjects();
    // Release key
    expected.release += 1;
    expected.drop += 1;
    expected.assert_current();
}

#[test]
fn dictionary_get_key_copies() {
    let mut dict = <NSMutableDictionary<NSCopyingRcTestObject, _>>::new();
    let key1 = NSCopyingRcTestObject::new();
    let _ = dict.insert_id(&key1, NSNumber::new_i32(1));
    let expected = ThreadTestData::current();

    let _ = dict.get(&key1);
    // No change, getting doesn't do anything to the key!
    expected.assert_current();
}

#[test]
fn dictionary_insert_value_retain_release() {
    let mut dict = <NSMutableDictionary<NSNumber, _>>::new();
    dict.insert_id(&NSNumber::new_i32(1), RcTestObject::new());
    let to_insert = RcTestObject::new();
    let mut expected = ThreadTestData::current();

    let old = dict.insert(&NSNumber::new_i32(1), &to_insert);
    // Grab old value
    expected.retain += 1;

    // Dictionary takes new value and overwrites the old one
    expected.retain += 1;
    expected.release += 1;

    expected.assert_current();

    drop(old);
    expected.release += 1;
    expected.drop += 1;
    expected.assert_current();
}

#[test]
fn dictionary_remove_clear_release_dealloc() {
    let mut dict = <NSMutableDictionary<NSNumber, _>>::new();
    for i in 0..4 {
        dict.insert_id(&NSNumber::new_i32(i), RcTestObject::new());
    }
    let mut expected = ThreadTestData::current();

    let _obj = dict.remove(&NSNumber::new_i32(1));
    expected.retain += 1;
    expected.release += 1;
    expected.assert_current();
    assert_eq!(dict.len(), 3);

    let _obj = dict.remove(&NSNumber::new_i32(2));
    expected.retain += 1;
    expected.release += 1;
    expected.assert_current();
    assert_eq!(dict.len(), 2);

    dict.removeAllObjects();
    expected.release += 2;
    expected.drop += 2;
    expected.assert_current();
    assert_eq!(dict.len(), 0);
}
