#![cfg_attr(not(feature = "afl"), no_main)]
use std::mem::replace;

use arbitrary::Arbitrary;
use icrate::Foundation::{NSMutableArray, NSNull, NSObject, NSObjectProtocol};
use objc2::rc::{autoreleasepool, Id};

/// The operations that the fuzzer can do on an array and the array's iterator.
///
/// This contains the operations specified in `NSArray` and `NSMutableArray`'s
/// subclassing information:
/// - https://developer.apple.com/documentation/foundation/nsarray?language=objc#1651563
/// - https://developer.apple.com/documentation/foundation/nsmutablearray?language=objc#1770394
#[derive(Arbitrary, Debug)]
enum Operation {
    /// count
    Count,
    /// objectAtIndex:
    Get(usize),

    /// insertObject:atIndex:
    Insert(usize),
    /// removeObjectAtIndex:
    Remove(usize),
    /// addObject:
    Add,
    /// removeLastObject
    Pop,
    /// replaceObjectAtIndex:withObject:
    Replace(usize),
    /// removeAllObjects
    Clear,

    CreateIter,
    NextElemIter,
}

#[derive(Debug)]
struct VecIter {
    index: usize,
    invalid: bool,
}

impl VecIter {
    fn new() -> Self {
        Self {
            index: 0,
            invalid: false,
        }
    }

    fn invalidate(&mut self) {
        self.invalid = true;
    }

    fn next<'a, T>(&mut self, vec: &'a [T]) -> Option<&'a T> {
        let item = vec.get(self.index)?;
        self.index += 1;
        Some(item)
    }
}

fn retain_count(obj: &NSObject) -> usize {
    unsafe { objc2::msg_send![obj, retainCount] }
}

fn run(ops: Vec<Operation>) {
    let arr: Id<NSMutableArray<NSObject>> = NSMutableArray::new();
    let mut vec: Vec<Id<NSObject>> = Vec::new();

    let mut arr_iter = arr.iter();
    let mut vec_iter = VecIter::new();

    for op in ops {
        autoreleasepool(|_| match op {
            Operation::Count => {
                assert_eq!(arr.len(), vec.len());
            }
            Operation::Get(idx) => {
                assert_eq!(arr.get(idx), vec.get(idx).map(|s| &**s));
            }
            Operation::Insert(idx) => {
                if idx < vec.len() {
                    let obj = NSObject::new();
                    arr.insert(idx, obj.clone());
                    vec.insert(idx, obj);
                    vec_iter.invalidate();
                }
            }
            Operation::Remove(idx) => {
                let arr_elem = arr.remove(idx);
                let vec_elem = if idx < vec.len() {
                    vec_iter.invalidate();
                    Some(vec.remove(idx))
                } else {
                    None
                };
                assert_eq!(arr_elem, vec_elem);
            }
            Operation::Add => {
                let obj = NSObject::new();
                arr.push(obj.clone());
                vec.push(obj);
                vec_iter.invalidate();
            }
            Operation::Pop => {
                if 0 < vec.len() {
                    vec_iter.invalidate();
                }
                assert_eq!(arr.pop(), vec.pop());
            }
            Operation::Replace(idx) => {
                let obj = NSObject::new();
                let arr_old = arr.replace(idx, obj.clone());
                let vec_old = if let Some(item) = vec.get_mut(idx) {
                    vec_iter.invalidate();
                    Ok(replace(item, obj))
                } else {
                    Err(obj)
                };
                assert_eq!(arr_old, vec_old);
            }
            Operation::Clear => {
                arr.removeAllObjects();
                vec.clear();
                vec_iter.invalidate();
            }
            Operation::CreateIter => {
                arr_iter = arr.iter();
                vec_iter = VecIter::new();
            }
            Operation::NextElemIter => {
                let arr_item = autoreleasepool(|_| arr_iter.next());
                let vec_item = vec_iter.next(&vec).map(|s| &**s);
                match (arr_item, vec_item) {
                    (Some(arr_s), _) if arr_s.is_kind_of::<NSNull>() => {
                        assert!(vec_iter.invalid, "mutation while iterator was valid");
                    }
                    (Some(arr_s), Some(vec_s)) => {
                        assert_eq!(arr_s, vec_s);
                        assert_eq!(retain_count(arr_s), 2);
                    }
                    (None, Some(vec_s)) => {
                        assert!(vec_iter.invalid, "vec contained item, array did not");
                        assert_eq!(retain_count(vec_s), 2);
                    }
                    (Some(_), None) => {
                        panic!("array contained item, vec did not");
                    }
                    (None, None) => {}
                }
            }
        });
    }
}

#[cfg(not(feature = "afl"))]
libfuzzer_sys::fuzz_target!(|ops: Vec<Operation>| run(ops));

#[cfg(feature = "afl")]
fn main() {
    afl::fuzz!(|ops: Vec<Operation>| {
        run(ops);
    });
}
