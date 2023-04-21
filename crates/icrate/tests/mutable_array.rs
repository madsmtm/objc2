#![cfg(feature = "Foundation_NSMutableArray")]
use objc2::rc::{__RcTestObject, __ThreadTestData, autoreleasepool};

use icrate::Foundation::{self, NSMutableArray};

#[test]
fn test_adding() {
    let mut array = NSMutableArray::new();
    let obj1 = __RcTestObject::new();
    let obj2 = __RcTestObject::new();
    let mut expected = __ThreadTestData::current();

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
fn test_replace() {
    let mut array = NSMutableArray::new();
    let obj1 = __RcTestObject::new();
    let obj2 = __RcTestObject::new();
    array.push(obj1);
    let mut expected = __ThreadTestData::current();

    let old_obj = array.replace(0, obj2).unwrap();
    expected.retain += 2;
    expected.release += 2;
    expected.assert_current();
    assert_ne!(&*old_obj, array.get(0).unwrap());
}

#[test]
fn test_remove() {
    let mut array = NSMutableArray::new();
    for _ in 0..4 {
        array.push(__RcTestObject::new());
    }
    let mut expected = __ThreadTestData::current();

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
    expected.dealloc += 2;
    expected.assert_current();
    assert_eq!(array.len(), 0);
}

#[test]
#[cfg(feature = "Foundation_NSString")]
fn test_into_vec() {
    let array = NSMutableArray::from_id_slice(&[Foundation::NSString::new()]);

    let vec = NSMutableArray::into_vec(array);
    assert_eq!(vec.len(), 1);
}

#[test]
#[cfg(feature = "Foundation_NSString")]
fn test_sort() {
    use Foundation::NSString;

    let strings = vec![NSString::from_str("hello"), NSString::from_str("hi")];
    let mut strings = NSMutableArray::from_vec(strings);

    autoreleasepool(|pool| {
        strings.sort_by(|s1, s2| s1.as_str(pool).len().cmp(&s2.as_str(pool).len()));
        assert_eq!(strings[0].as_str(pool), "hi");
        assert_eq!(strings[1].as_str(pool), "hello");
    });
}
