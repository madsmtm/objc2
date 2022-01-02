use alloc::vec::Vec;
use core::cmp::Ordering;
use core::ffi::c_void;
use core::marker::PhantomData;
use core::ops::{Index, IndexMut, Range};
use core::ptr::NonNull;

use objc2::msg_send;
use objc2::rc::{DefaultId, Id, Owned, Ownership, Shared, SliceId};
use objc2::runtime::Object;

use super::{
    INSCopying, INSFastEnumeration, INSMutableCopying, INSObject, NSComparisonResult, NSEnumerator,
    NSRange,
};

unsafe fn from_refs<A: INSArray + ?Sized>(refs: &[&A::Item]) -> Id<A, A::Ownership> {
    let cls = A::class();
    let obj: *mut A = unsafe { msg_send![cls, alloc] };
    let obj: *mut A = unsafe {
        msg_send![
            obj,
            initWithObjects: refs.as_ptr(),
            count: refs.len(),
        ]
    };
    let obj = unsafe { NonNull::new_unchecked(obj) };
    unsafe { Id::new(obj) }
}

pub unsafe trait INSArray: INSObject {
    type Ownership: Ownership;
    type Item: INSObject;
    type ItemOwnership: Ownership;

    unsafe_def_fn!(fn new -> Self::Ownership);

    #[doc(alias = "count")]
    fn len(&self) -> usize {
        unsafe { msg_send![self, count] }
    }

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[doc(alias = "objectAtIndex:")]
    fn get(&self, index: usize) -> Option<&Self::Item> {
        // TODO: Replace this check with catching the thrown NSRangeException
        if index < self.len() {
            // SAFETY: The index is checked to be in bounds.
            Some(unsafe { msg_send![self, objectAtIndex: index] })
        } else {
            None
        }
    }

    #[doc(alias = "objectAtIndex:")]
    fn get_mut(&mut self, index: usize) -> Option<&mut Self::Item>
    where
        Self: INSArray<ItemOwnership = Owned>,
    {
        // TODO: Replace this check with catching the thrown NSRangeException
        if index < self.len() {
            // SAFETY: The index is checked to be in bounds.
            Some(unsafe { msg_send![self, objectAtIndex: index] })
        } else {
            None
        }
    }

    #[doc(alias = "objectAtIndex:")]
    fn get_retained(&self, index: usize) -> Id<Self::Item, Shared>
    where
        Self: INSArray<ItemOwnership = Shared>,
    {
        let obj = self.get(index).unwrap();
        // SAFETY: The object is originally shared (see `where` bound).
        unsafe { Id::retain(obj.into()) }
    }

    #[doc(alias = "firstObject")]
    fn first(&self) -> Option<&Self::Item> {
        unsafe { msg_send![self, firstObject] }
    }

    #[doc(alias = "firstObject")]
    fn first_mut(&mut self) -> Option<&mut Self::Item>
    where
        Self: INSArray<ItemOwnership = Owned>,
    {
        unsafe { msg_send![self, firstObject] }
    }

    #[doc(alias = "lastObject")]
    fn last(&self) -> Option<&Self::Item> {
        unsafe { msg_send![self, lastObject] }
    }

    #[doc(alias = "lastObject")]
    fn last_mut(&mut self) -> Option<&mut Self::Item>
    where
        Self: INSArray<ItemOwnership = Owned>,
    {
        unsafe { msg_send![self, lastObject] }
    }

    #[doc(alias = "objectEnumerator")]
    fn iter(&self) -> NSEnumerator<'_, Self::Item> {
        unsafe {
            let result: *mut Object = msg_send![self, objectEnumerator];
            NSEnumerator::from_ptr(result)
        }
    }

    fn from_vec(vec: Vec<Id<Self::Item, Self::ItemOwnership>>) -> Id<Self, Self::Ownership> {
        unsafe { from_refs(vec.as_slice_ref()) }
    }

    fn objects_in_range(&self, range: Range<usize>) -> Vec<&Self::Item> {
        let range = NSRange::from(range);
        let mut vec = Vec::with_capacity(range.length);
        unsafe {
            let _: () = msg_send![self, getObjects: vec.as_ptr(), range: range];
            vec.set_len(range.length);
        }
        vec
    }

    fn to_vec(&self) -> Vec<&Self::Item> {
        self.objects_in_range(0..self.len())
    }

    // TODO: Take Id<Self, Self::ItemOwnership> ?
    fn into_vec(array: Id<Self, Owned>) -> Vec<Id<Self::Item, Self::ItemOwnership>> {
        array
            .to_vec()
            .into_iter()
            .map(|obj| unsafe { Id::retain(obj.into()) })
            .collect()
    }

    fn from_slice(slice: &[Id<Self::Item, Shared>]) -> Id<Self, Self::Ownership>
    where
        Self: INSArray<ItemOwnership = Shared>,
    {
        unsafe { from_refs(slice.as_slice_ref()) }
    }

    fn to_shared_vec(&self) -> Vec<Id<Self::Item, Shared>>
    where
        Self: INSArray<ItemOwnership = Shared>,
    {
        self.to_vec()
            .into_iter()
            .map(|obj| unsafe { Id::retain(obj.into()) })
            .collect()
    }
}

object!(
    /// TODO
    ///
    /// You can have a `Id<NSArray<T, Owned>, Owned>`, which allows mutable access
    /// to the elements (without modifying the array itself), and
    /// `Id<NSArray<T, Shared>, Shared>` which allows sharing the array.
    ///
    /// `Id<NSArray<T, Owned>, Shared>` is possible, but pretty useless.
    /// TODO: Can we make it impossible? Should we?
    ///
    /// What about `Id<NSArray<T, Shared>, Owned>`?
    unsafe pub struct NSArray<T, O: Ownership> {
        item: PhantomData<Id<T, O>>,
    }
);

// SAFETY: Same as Id<T, O> (which is what NSArray effectively stores).
//
// The `PhantomData` can't get these impls to display in the docs.
//
// TODO: Properly verify this
unsafe impl<T: Sync + Send> Sync for NSArray<T, Shared> {}
unsafe impl<T: Sync + Send> Send for NSArray<T, Shared> {}
unsafe impl<T: Sync> Sync for NSArray<T, Owned> {}
unsafe impl<T: Send> Send for NSArray<T, Owned> {}

/// ```compile_fail
/// use objc2::rc::Shared;
/// use objc2::runtime::Object;
/// use objc2_foundation::NSArray;
/// fn needs_send_sync<T: Send + Sync>() {}
/// needs_send_sync::<NSArray<Object, Shared>>();
/// ```
#[cfg(doctest)]
pub struct NSArrayWithObjectNotSendSync;

unsafe impl<T: INSObject, O: Ownership> INSArray for NSArray<T, O> {
    /// The `NSArray` itself (length and number of items) is always immutable,
    /// but we would like to know when we're the only owner of the array, to
    /// allow mutation of the array's items.
    ///
    /// We only implement `INSCopying` when `O = Shared`, so this is safe.
    type Ownership = O;

    type Item = T;
    type ItemOwnership = O;
}

// Copying only possible when ItemOwnership = Shared

unsafe impl<T: INSObject> INSCopying for NSArray<T, Shared> {
    type Ownership = Shared;
    type Output = NSArray<T, Shared>;
}

unsafe impl<T: INSObject> INSMutableCopying for NSArray<T, Shared> {
    type Output = NSMutableArray<T, Shared>;
}

unsafe impl<T: INSObject, O: Ownership> INSFastEnumeration for NSArray<T, O> {
    type Item = T;
}

impl<T: INSObject, O: Ownership> Index<usize> for NSArray<T, O> {
    type Output = T;

    fn index(&self, index: usize) -> &T {
        self.get(index).unwrap()
    }
}

impl<T: INSObject, O: Ownership> DefaultId for NSArray<T, O> {
    type Ownership = O;

    #[inline]
    fn default_id() -> Id<Self, Self::Ownership> {
        Self::new()
    }
}

pub unsafe trait INSMutableArray: INSArray {
    #[doc(alias = "addObject:")]
    fn push(&mut self, obj: Id<Self::Item, Self::ItemOwnership>) {
        // SAFETY: The object is not nil
        unsafe { msg_send![self, addObject: &*obj] }
    }

    #[doc(alias = "insertObject:atIndex:")]
    fn insert(&mut self, index: usize, obj: Id<Self::Item, Self::ItemOwnership>) {
        // TODO: Replace this check with catching the thrown NSRangeException
        let len = self.len();
        if index < len {
            // SAFETY: The object is not nil and the index is checked to be in
            // bounds.
            unsafe { msg_send![self, insertObject: &*obj, atIndex: index] }
        } else {
            panic!(
                "insertion index (is {}) should be <= len (is {})",
                index, len
            );
        }
    }

    #[doc(alias = "replaceObjectAtIndex:withObject:")]
    fn replace(
        &mut self,
        index: usize,
        obj: Id<Self::Item, Self::ItemOwnership>,
    ) -> Id<Self::Item, Self::ItemOwnership> {
        let old_obj = unsafe {
            let obj = self.get(index).unwrap();
            Id::retain(obj.into())
        };
        unsafe {
            let _: () = msg_send![
                self,
                replaceObjectAtIndex: index,
                withObject: &*obj,
            ];
        }
        old_obj
    }

    #[doc(alias = "removeObjectAtIndex:")]
    fn remove(&mut self, index: usize) -> Id<Self::Item, Self::ItemOwnership> {
        let obj = if let Some(obj) = self.get(index) {
            unsafe { Id::retain(obj.into()) }
        } else {
            panic!("removal index should be < len");
        };
        unsafe {
            let _: () = msg_send![self, removeObjectAtIndex: index];
        }
        obj
    }

    #[doc(alias = "removeLastObject")]
    fn pop(&mut self) -> Option<Id<Self::Item, Self::ItemOwnership>> {
        self.last().map(|obj| {
            let obj = unsafe { Id::retain(obj.into()) };
            unsafe {
                let _: () = msg_send![self, removeLastObject];
            }
            obj
        })
    }

    #[doc(alias = "removeAllObjects")]
    fn clear(&mut self) {
        unsafe { msg_send![self, removeAllObjects] }
    }

    #[doc(alias = "sortUsingFunction:context:")]
    fn sort_by<F>(&mut self, compare: F)
    where
        F: FnMut(&Self::Item, &Self::Item) -> Ordering,
    {
        extern "C" fn compare_with_closure<T, F: FnMut(&T, &T) -> Ordering>(
            obj1: &T,
            obj2: &T,
            context: *mut c_void,
        ) -> NSComparisonResult {
            // Bring back a reference to the closure.
            // Guaranteed to be unique, we gave `sortUsingFunction` unique is
            // ownership, and that method only runs one function at a time.
            let closure: &mut F = unsafe { &mut *(context as *mut F) };

            NSComparisonResult::from((*closure)(obj1, obj2))
        }

        let f: extern "C" fn(_, _, _) -> _ = compare_with_closure::<Self::Item, F>;

        // Grab a type-erased pointer to the closure (a pointer to stack).
        let mut closure = compare;
        let context = &mut closure as *mut F as *mut c_void;

        unsafe {
            let _: () = msg_send![self, sortUsingFunction: f, context: context];
        }
        // Keep the closure alive until the function has run.
        drop(closure);
    }
}

object!(
    unsafe pub struct NSMutableArray<T, O: Ownership> {
        item: PhantomData<Id<T, O>>,
    }
);

// SAFETY: Same as NSArray.
//
// TODO: Properly verify this
unsafe impl<T: Sync + Send> Sync for NSMutableArray<T, Shared> {}
unsafe impl<T: Sync + Send> Send for NSMutableArray<T, Shared> {}
unsafe impl<T: Sync> Sync for NSMutableArray<T, Owned> {}
unsafe impl<T: Send> Send for NSMutableArray<T, Owned> {}

unsafe impl<T: INSObject, O: Ownership> INSArray for NSMutableArray<T, O> {
    type Ownership = Owned;
    type Item = T;
    type ItemOwnership = O;
}

unsafe impl<T: INSObject, O: Ownership> INSMutableArray for NSMutableArray<T, O> {}

// Copying only possible when ItemOwnership = Shared

unsafe impl<T: INSObject> INSCopying for NSMutableArray<T, Shared> {
    type Ownership = Shared;
    type Output = NSArray<T, Shared>;
}

unsafe impl<T: INSObject> INSMutableCopying for NSMutableArray<T, Shared> {
    type Output = NSMutableArray<T, Shared>;
}

unsafe impl<T: INSObject, O: Ownership> INSFastEnumeration for NSMutableArray<T, O> {
    type Item = T;
}

impl<T: INSObject, O: Ownership> Index<usize> for NSMutableArray<T, O> {
    type Output = T;

    fn index(&self, index: usize) -> &T {
        self.get(index).unwrap()
    }
}

impl<T: INSObject> IndexMut<usize> for NSMutableArray<T, Owned> {
    fn index_mut(&mut self, index: usize) -> &mut T {
        self.get_mut(index).unwrap()
    }
}

impl<T: INSObject, O: Ownership> DefaultId for NSMutableArray<T, O> {
    type Ownership = Owned;

    #[inline]
    fn default_id() -> Id<Self, Self::Ownership> {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use alloc::format;
    use alloc::vec;
    use alloc::vec::Vec;

    use objc2::msg_send;
    use objc2::rc::{autoreleasepool, Id, Owned, Shared};

    use super::{INSArray, INSMutableArray, NSArray, NSMutableArray};
    use crate::{INSObject, INSString, INSValue, NSObject, NSString, NSValue};

    fn sample_array(len: usize) -> Id<NSArray<NSObject, Owned>, Owned> {
        let mut vec = Vec::with_capacity(len);
        for _ in 0..len {
            vec.push(NSObject::new());
        }
        NSArray::from_vec(vec)
    }

    fn sample_number_array(len: u8) -> Id<NSArray<NSValue<u8>, Shared>, Shared> {
        let mut vec = Vec::with_capacity(len as usize);
        for i in 0..len {
            vec.push(NSValue::new(i));
        }
        NSArray::from_vec(vec)
    }

    fn retain_count<T: INSObject>(obj: &T) -> usize {
        unsafe { msg_send![obj, retainCount] }
    }

    #[test]
    fn test_len() {
        let empty_array = NSArray::<NSObject, Owned>::new();
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
    #[ignore = "Output is different depending on OS version and runtime"]
    fn test_debug() {
        let obj = sample_number_array(3);
        let expected = r#"(
    "<00>",
    "<01>",
    "<02>"
)"#;
        assert_eq!(format!("{:?}", obj), format!("{:?}", expected));
    }

    #[test]
    fn test_get() {
        let array = sample_array(4);
        assert_ne!(array.get(0), array.get(3));
        assert_eq!(array.first(), array.get(0));
        assert_eq!(array.last(), array.get(3));

        let empty_array = <NSArray<NSObject, Owned>>::new();
        assert!(empty_array.first().is_none());
        assert!(empty_array.last().is_none());
    }

    #[test]
    fn test_get_does_not_autorelease() {
        let obj: Id<_, Shared> = NSObject::new().into();

        assert_eq!(retain_count(&*obj), 1);

        let array = NSArray::from_slice(&[obj.clone()]);

        assert_eq!(retain_count(&*obj), 2);

        autoreleasepool(|_pool| {
            let obj2 = array.first().unwrap();
            assert_eq!(retain_count(obj2), 2);
        });

        assert_eq!(retain_count(&*obj), 2);

        drop(array);

        assert_eq!(retain_count(&*obj), 1);
    }

    #[test]
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

        let middle_objs = array.objects_in_range(1..3);
        assert_eq!(middle_objs.len(), 2);
        assert_eq!(middle_objs[0], array.get(1).unwrap());
        assert_eq!(middle_objs[1], array.get(2).unwrap());

        let empty_objs = array.objects_in_range(1..1);
        assert!(empty_objs.is_empty());

        let all_objs = array.objects_in_range(0..4);
        assert_eq!(all_objs.len(), 4);
    }

    #[test]
    fn test_into_vec() {
        let array = sample_array(4);

        let vec = INSArray::into_vec(array);
        assert_eq!(vec.len(), 4);
    }

    #[test]
    fn test_adding() {
        let mut array = NSMutableArray::new();
        let obj = NSObject::new();
        array.push(obj);

        assert_eq!(array.len(), 1);
        assert_eq!(array.get(0), array.get(0));

        let obj = NSObject::new();
        array.insert(0, obj);
        assert_eq!(array.len(), 2);
    }

    #[test]
    fn test_replace() {
        let mut array = NSMutableArray::new();
        let obj = NSObject::new();
        array.push(obj);

        let obj = NSObject::new();
        let old_obj = array.replace(0, obj);
        assert_ne!(&*old_obj, array.get(0).unwrap());
    }

    #[test]
    fn test_remove() {
        let mut array = NSMutableArray::new();
        for _ in 0..4 {
            array.push(NSObject::new());
        }

        let _ = array.remove(1);
        assert_eq!(array.len(), 3);

        let _ = array.pop();
        assert_eq!(array.len(), 2);

        array.clear();
        assert_eq!(array.len(), 0);
    }

    #[test]
    fn test_sort() {
        let strings = vec![NSString::from_str("hello"), NSString::from_str("hi")];
        let mut strings = NSMutableArray::from_vec(strings);

        autoreleasepool(|pool| {
            strings.sort_by(|s1, s2| s1.as_str(pool).len().cmp(&s2.as_str(pool).len()));
            assert_eq!(strings[0].as_str(pool), "hi");
            assert_eq!(strings[1].as_str(pool), "hello");
        });
    }

    #[test]
    fn test_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}

        assert_send_sync::<NSArray<NSString, Shared>>();
        assert_send_sync::<NSMutableArray<NSString, Shared>>();
        assert_send_sync::<Id<NSArray<NSString, Shared>, Shared>>();
        assert_send_sync::<Id<NSMutableArray<NSString, Shared>, Owned>>();
    }
}
