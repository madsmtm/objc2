#![cfg(feature = "all")]
use std::ptr;

use objc2::rc::Retained;
use objc2::{declare_class, extern_methods};
use objc2_foundation::{
    CopyingHelper, NSArray, NSCopying, NSMutableArray, NSMutableCopying, NSMutableDictionary,
    NSMutableSet, NSNumber, NSSet, NSValue,
};

use crate::rc_test_object::{RcTestObject, ThreadTestData};

#[test]
fn array_retains_stored() {
    let obj = RcTestObject::new();
    let mut expected = ThreadTestData::current();

    let input = [obj.clone(), obj.clone()];
    expected.retain += 2;
    expected.assert_current();

    let array = NSArray::from_retained_slice(&input);
    expected.retain += 2;
    expected.assert_current();

    drop(input);
    expected.release += 2;
    expected.assert_current();

    let _obj = unsafe { array.firstObject_unchecked() }.unwrap();
    expected.assert_current();

    drop(array);
    expected.release += 2;
    expected.assert_current();

    let array = NSArray::from_slice(&[&*obj, &*obj]);
    expected.retain += 2;
    expected.assert_current();

    let _obj = unsafe { array.objectAtIndex_unchecked(0) };
    let _obj = unsafe { array.objectAtIndex_unchecked(1) };
    assert_eq!(array.len(), 2);
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

    let set = NSSet::from_retained_slice(&input);
    expected.retain += 1;
    expected.assert_current();

    drop(input);
    expected.release += 2;
    expected.assert_current();

    let _obj = unsafe { set.anyObject_unchecked().unwrap() };
    expected.assert_current();

    drop(set);
    expected.release += 1;
    expected.assert_current();

    let set = NSSet::from_slice(&[&*obj]);
    expected.retain += 1;
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
    let array = NSArray::from_retained_slice(&[obj]);
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
    let set = NSSet::from_retained_slice(&[obj]);
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
    let array = NSArray::from_retained_slice(&objs);
    drop(objs);
    let mut expected = ThreadTestData::current();

    // Iter
    let mut iter = array.iter();
    expected.assert_current();

    assert!(iter.next().is_some());
    expected.retain += 1;
    expected.release += 1;
    expected.assert_current();

    assert_eq!(iter.count(), 0);
    expected.assert_current();

    // IterUnchecked
    let mut iter = unsafe { array.iter_unchecked() };
    expected.assert_current();

    assert!(iter.next().is_some());
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
    let set = NSSet::from_retained_slice(&objs);
    drop(objs);
    let mut expected = ThreadTestData::current();

    // Iter
    let mut iter = set.iter();
    expected.assert_current();

    assert!(iter.next().is_some());
    expected.retain += 1;
    expected.release += 1;
    expected.assert_current();

    assert_eq!(iter.count(), 0);
    expected.assert_current();

    // IterUnchecked
    let mut iter = unsafe { set.iter_unchecked() };
    expected.assert_current();

    assert!(iter.next().is_some());
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
    let array = NSMutableArray::new();
    let obj1 = RcTestObject::new();
    let obj2 = RcTestObject::new();
    let mut expected = ThreadTestData::current();

    array.addObject(&*obj1);
    expected.retain += 1;
    expected.assert_current();
    assert_eq!(array.len(), 1);
    assert_eq!(unsafe { array.objectAtIndex_unchecked(0) }, &*obj1);

    array.insert(0, &obj2);
    expected.retain += 1;
    expected.assert_current();
    assert_eq!(array.len(), 2);
    assert_eq!(unsafe { array.objectAtIndex_unchecked(0) }, &*obj2);
}

#[test]
#[cfg_attr(
    feature = "gnustep-1-7",
    ignore = "thread safety issues regarding initialization"
)]
fn array_replace() {
    let array = NSMutableArray::new();
    let obj1 = RcTestObject::new();
    let obj2 = RcTestObject::new();
    array.addObject(&*obj1);
    let mut expected = ThreadTestData::current();

    array.replaceObjectAtIndex_withObject(0, &obj2);
    expected.retain += 1;
    expected.release += 1;
    expected.assert_current();
}

#[test]
#[cfg_attr(
    feature = "gnustep-1-7",
    ignore = "thread safety issues regarding initialization"
)]
fn array_remove() {
    let array = NSMutableArray::new();
    for _ in 0..4 {
        array.addObject(&*RcTestObject::new());
    }
    let mut expected = ThreadTestData::current();

    array.removeObjectAtIndex(1);
    expected.release += 1;
    expected.drop += 1;
    expected.assert_current();
    assert_eq!(array.len(), 3);

    array.removeLastObject();
    expected.release += 1;
    expected.drop += 1;
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
    let set = <NSMutableSet<RcTestObject>>::new();
    let obj1 = RcTestObject::new();
    let obj2 = RcTestObject::new();
    let mut expected = ThreadTestData::current();

    set.addObject(&obj1);
    // Retain to store in set
    expected.retain += 1;
    expected.assert_current();
    assert_eq!(set.len(), 1);
    assert_eq!(unsafe { set.anyObject_unchecked().unwrap() }, &*obj1);

    set.addObject(&obj2);
    // Retain to store in set
    expected.retain += 1;
    expected.assert_current();
    assert_eq!(set.len(), 2);

    set.addObject(&obj2);
    // No retain, since the object is already in the set
    expected.retain += 0;
    expected.assert_current();
    assert_eq!(set.len(), 2);
}

#[test]
fn set_clear_release_dealloc() {
    let set = NSMutableSet::new();
    for _ in 0..4 {
        set.addObject(&*RcTestObject::new());
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
    #[unsafe(super(RcTestObject))]
    #[name = "NSCopyingRcTestObject"]
    #[derive(Debug, PartialEq, Eq, Hash)]
    struct NSCopyingRcTestObject;

    unsafe impl NSCopying for NSCopyingRcTestObject {}
);

unsafe impl CopyingHelper for NSCopyingRcTestObject {
    type Result = Self;
}

extern_methods!(
    unsafe impl NSCopyingRcTestObject {
        #[method_id(new)]
        fn new() -> Retained<Self>;
    }
);

#[test]
fn dictionary_insert_key_copies() {
    let dict = NSMutableDictionary::new();
    let key1 = NSCopyingRcTestObject::new();
    let mut expected = ThreadTestData::current();

    dict.insert(&*key1, &*NSNumber::new_i32(1));
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
    let dict = NSMutableDictionary::new();
    let key1 = NSCopyingRcTestObject::new();
    dict.insert(&*key1, &*NSNumber::new_i32(1));
    let expected = ThreadTestData::current();

    let _ = dict.objectForKey(&key1);
    // No change, getting doesn't do anything to the key!
    expected.assert_current();
}

#[test]
fn dictionary_insert_value_retain_release() {
    let dict = NSMutableDictionary::new();
    dict.insert(&*NSNumber::new_i32(1), &*RcTestObject::new());
    let to_insert = RcTestObject::new();
    let mut expected = ThreadTestData::current();

    dict.insert(&*NSNumber::new_i32(1), &to_insert);
    // Dictionary takes new value and overwrites the old one
    expected.retain += 1;
    expected.release += 1;
    expected.drop += 1;
    expected.assert_current();
}

#[test]
fn dictionary_remove_clear_release_dealloc() {
    let dict = NSMutableDictionary::new();
    for i in 0..4 {
        dict.insert(&*NSNumber::new_i32(i), &*RcTestObject::new());
    }
    let mut expected = ThreadTestData::current();

    dict.removeObjectForKey(&NSNumber::new_i32(1));
    expected.release += 1;
    expected.drop += 1;
    expected.assert_current();
    assert_eq!(dict.len(), 3);

    dict.removeObjectForKey(&NSNumber::new_i32(2));
    expected.release += 1;
    expected.drop += 1;
    expected.assert_current();
    assert_eq!(dict.len(), 2);

    dict.removeAllObjects();
    expected.release += 2;
    expected.drop += 2;
    expected.assert_current();
    assert_eq!(dict.len(), 0);
}
