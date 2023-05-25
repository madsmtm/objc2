#![cfg(feature = "Foundation_NSArray")]
#![cfg(feature = "Foundation_NSNumber")]
use icrate::Foundation::{NSArray, NSNumber, NSObject};
use objc2::rc::Id;
use objc2::rc::{__RcTestObject, __ThreadTestData};

fn sample_array(len: usize) -> Id<NSArray<NSObject>> {
    let mut vec = Vec::with_capacity(len);
    for _ in 0..len {
        vec.push(NSObject::new());
    }
    NSArray::from_vec(vec)
}

fn sample_number_array(len: u8) -> Id<NSArray<NSNumber>> {
    let mut vec = Vec::with_capacity(len as usize);
    for i in 0..len {
        vec.push(NSNumber::new_u8(i));
    }
    NSArray::from_vec(vec)
}

#[test]
fn test_two_empty() {
    let _empty_array1 = NSArray::<NSObject>::new();
    let _empty_array2 = NSArray::<NSObject>::new();
}

#[test]
fn test_creation() {
    let _ = <NSArray<NSNumber>>::from_vec(vec![]);
    let _ = NSArray::from_vec(vec![NSNumber::new_u8(4), NSNumber::new_u8(2)]);

    let _ = <NSArray<NSNumber>>::from_id_slice(&[]);
    let _ = NSArray::from_id_slice(&[NSNumber::new_u8(4), NSNumber::new_u8(2)]);

    let _ = <NSArray<NSNumber>>::from_slice(&[]);
    let _ = NSArray::from_slice(&[&*NSNumber::new_u8(4), &*NSNumber::new_u8(2)]);
}

#[test]
fn test_len() {
    let empty_array = NSArray::<NSObject>::new();
    assert_eq!(empty_array.len(), 0);

    let array = sample_array(4);
    assert_eq!(array.len(), 4);
}

#[test]
fn test_equality() {
    let array1 = sample_array(3);
    let array2 = sample_array(3);
    assert_ne!(array1, array2);

    let array1 = sample_number_array(3);
    let array2 = sample_number_array(3);
    assert_eq!(array1, array2);

    let array1 = sample_number_array(3);
    let array2 = sample_number_array(4);
    assert_ne!(array1, array2);
}

#[test]
fn test_debug() {
    let obj = sample_number_array(0);
    assert_eq!(format!("{obj:?}"), "[]");
    let obj = sample_number_array(3);
    assert_eq!(format!("{obj:?}"), "[0, 1, 2]");
}

#[test]
fn test_get() {
    let array = sample_array(4);
    assert_ne!(array.get(0), array.get(3));
    assert_eq!(array.first(), array.get(0));
    assert_eq!(array.last(), array.get(3));

    let empty_array = <NSArray<NSObject>>::new();
    assert!(empty_array.first().is_none());
    assert!(empty_array.last().is_none());
}

#[test]
fn test_retains_stored() {
    let obj = __RcTestObject::new();
    let mut expected = __ThreadTestData::current();

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
    expected.dealloc += 1;
    expected.assert_current();
}

#[test]
#[cfg(feature = "Foundation_NSMutableArray")]
fn test_nscopying_uses_retain() {
    use icrate::Foundation::{NSCopying, NSMutableCopying};

    let obj = __RcTestObject::new();
    let array = NSArray::from_id_slice(&[obj]);
    let mut expected = __ThreadTestData::current();

    let _copy = array.copy();
    expected.assert_current();

    let _copy = array.mutableCopy();
    expected.retain += 1;
    expected.assert_current();
}

#[test]
#[cfg_attr(
    feature = "apple",
    ignore = "this works differently on different framework versions"
)]
fn test_iter_minimal_retains() {
    let objs = [__RcTestObject::new()];
    let array = NSArray::from_id_slice(&objs);
    drop(objs);
    let mut expected = __ThreadTestData::current();

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
    expected.dealloc += 1;
    expected.assert_current();
}

#[test]
fn test_iter() {
    let array = sample_number_array(4);

    let vec1 = array.to_vec_retained();
    let vec2: Vec<_> = array.iter_retained().collect();
    assert_eq!(vec1, vec2);

    let mut iterations = 0;
    for _ in &array {
        iterations += 1;
    }
    for _ in array.iter() {
        iterations += 1;
    }
    for _ in array.iter_retained() {
        iterations += 1;
    }
    for _ in array {
        iterations += 1;
    }
    assert_eq!(iterations, 4 * 4);
}

#[test]
fn test_iter_fused() {
    // Not actually documented, nor is FusedIterator implemented for the
    // array iterators; but it it still important to test that iterating past
    // the end still works.

    let array = sample_number_array(2);

    let mut iter = array.iter();
    assert_eq!(iter.next(), Some(&*NSNumber::new_u8(0)));
    assert_eq!(iter.next(), Some(&*NSNumber::new_u8(1)));
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), None);
}

#[test]
fn test_two_iters() {
    let array = sample_number_array(4);

    let iter1 = array.iter();
    let iter2 = array.iter_retained();
    for (_, _) in iter1.zip(iter2) {}
}

#[test]
fn test_objects_in_range() {
    let array = sample_array(4);

    let middle_objs = array.objects_in_range(1..3).unwrap();
    assert_eq!(middle_objs.len(), 2);
    assert_eq!(middle_objs[0], array.get(1).unwrap());
    assert_eq!(middle_objs[1], array.get(2).unwrap());

    let empty_objs = array.objects_in_range(1..1).unwrap();
    assert!(empty_objs.is_empty());

    let all_objs = array.objects_in_range(0..4).unwrap();
    assert_eq!(all_objs.len(), 4);
}

#[test]
fn test_generic_ownership_traits() {
    fn assert_partialeq<T: PartialEq>() {}

    assert_partialeq::<NSArray<NSObject>>();
}
