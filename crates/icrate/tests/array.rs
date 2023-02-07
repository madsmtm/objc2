#![cfg(feature = "Foundation_NSArray")]
#![cfg(feature = "Foundation_NSNumber")]
use icrate::Foundation::{NSArray, NSNumber, NSObject};
use objc2::rc::{Id, Owned, Ownership, Shared};
use objc2::rc::{__RcTestObject, __ThreadTestData};

fn sample_array(len: usize) -> Id<NSArray<NSObject, Owned>, Owned> {
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
    let obj = Id::into_shared(__RcTestObject::new());
    let mut expected = __ThreadTestData::current();

    let input = [obj.clone(), obj.clone()];
    expected.retain += 2;
    expected.assert_current();

    let array = NSArray::from_slice(&input);
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

    let obj = Id::into_shared(__RcTestObject::new());
    let array = NSArray::from_slice(&[obj]);
    let mut expected = __ThreadTestData::current();

    let _copy = array.copy();
    expected.assert_current();

    let _copy = array.mutable_copy();
    expected.retain += 1;
    expected.assert_current();
}

#[test]
#[cfg_attr(
    feature = "apple",
    ignore = "this works differently on different framework versions"
)]
#[cfg(feature = "Foundation_NSEnumerator")]
fn test_iter_no_retain() {
    use icrate::Foundation::NSFastEnumeration2;

    let obj = Id::into_shared(__RcTestObject::new());
    let array = NSArray::from_slice(&[obj]);
    let mut expected = __ThreadTestData::current();

    let iter = array.iter();
    expected.retain += 0;
    expected.assert_current();

    assert_eq!(iter.count(), 1);
    expected.autorelease += 0;
    expected.assert_current();

    let iter = array.iter_fast();
    assert_eq!(iter.count(), 1);
    expected.assert_current();
}

#[test]
#[cfg(feature = "Foundation_NSEnumerator")]
fn test_iter() {
    let array = sample_array(4);

    assert_eq!(array.iter().count(), 4);
    assert!(array
        .iter()
        .enumerate()
        .all(|(i, obj)| Some(obj) == array.get(i)));
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
fn test_into_vec() {
    let array = sample_array(4);

    let vec = NSArray::into_vec(array);
    assert_eq!(vec.len(), 4);
}

#[test]
#[cfg(feature = "Foundation_NSString")]
fn test_generic_ownership_traits() {
    use icrate::Foundation::NSString;

    fn assert_partialeq<T: PartialEq>() {}

    assert_partialeq::<NSArray<NSString, Shared>>();
    assert_partialeq::<NSArray<NSString, Owned>>();

    fn test_ownership_implies_partialeq<O: Ownership>() {
        assert_partialeq::<NSArray<NSString, O>>();
    }

    test_ownership_implies_partialeq::<Shared>();
    test_ownership_implies_partialeq::<Owned>();
}
