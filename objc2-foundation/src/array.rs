use alloc::vec::Vec;
use core::cmp::Ordering;
use core::ffi::c_void;
use core::marker::PhantomData;
use core::ops::{Index, IndexMut, Range};

use objc2::msg_send;
use objc2::rc::{DefaultId, Id, Owned, Ownership, Shared, SliceId};
use objc2::runtime::{Class, Object};
use objc2::Message;

use super::{
    NSComparisonResult, NSCopying, NSEnumerator, NSFastEnumeration, NSMutableCopying, NSObject,
    NSRange,
};

object! {
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
    unsafe pub struct NSArray<T, O: Ownership>: NSObject {
        item: PhantomData<Id<T, O>>,
    }
}

// SAFETY: Same as Id<T, O> (which is what NSArray effectively stores).
//
// The `PhantomData` can't get these impls to display in the docs.
//
// TODO: Properly verify this
unsafe impl<T: Sync + Send> Sync for NSArray<T, Shared> {}
unsafe impl<T: Sync + Send> Send for NSArray<T, Shared> {}
unsafe impl<T: Sync> Sync for NSArray<T, Owned> {}
unsafe impl<T: Send> Send for NSArray<T, Owned> {}

object! {
    // TODO: Ensure that this deref to NSArray is safe!
    // This "inherits" NSArray, and has the same `Send`/`Sync` impls as that.
    unsafe pub struct NSMutableArray<T, O: Ownership>: NSArray<T, O>, NSObject {}
}

unsafe fn from_refs<T: Message + ?Sized>(cls: &Class, refs: &[&T]) -> *mut Object {
    let obj: *mut Object = unsafe { msg_send![cls, alloc] };
    unsafe {
        msg_send![
            obj,
            initWithObjects: refs.as_ptr(),
            count: refs.len(),
        ]
    }
}

impl<T: Message> NSArray<T, Shared> {
    unsafe_def_fn! {
        pub fn new -> Shared;
    }
}

impl<T: Message, O: Ownership> NSArray<T, O> {
    #[doc(alias = "count")]
    pub fn len(&self) -> usize {
        unsafe { msg_send![self, count] }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[doc(alias = "objectAtIndex:")]
    pub fn get(&self, index: usize) -> Option<&T> {
        // TODO: Replace this check with catching the thrown NSRangeException
        if index < self.len() {
            // SAFETY: The index is checked to be in bounds.
            Some(unsafe { msg_send![self, objectAtIndex: index] })
        } else {
            None
        }
    }

    #[doc(alias = "firstObject")]
    pub fn first(&self) -> Option<&T> {
        unsafe { msg_send![self, firstObject] }
    }

    #[doc(alias = "lastObject")]
    pub fn last(&self) -> Option<&T> {
        unsafe { msg_send![self, lastObject] }
    }

    #[doc(alias = "objectEnumerator")]
    pub fn iter(&self) -> NSEnumerator<'_, T> {
        unsafe {
            let result: *mut Object = msg_send![self, objectEnumerator];
            NSEnumerator::from_ptr(result)
        }
    }

    // The `NSArray` itself (length and number of items) is always immutable,
    // but we would like to know when we're the only owner of the array, to
    // allow mutation of the array's items.
    pub fn from_vec(vec: Vec<Id<T, O>>) -> Id<Self, O> {
        unsafe { Id::new(from_refs(Self::class(), vec.as_slice_ref()).cast()).unwrap() }
    }

    pub fn objects_in_range(&self, range: Range<usize>) -> Vec<&T> {
        let range = NSRange::from(range);
        let mut vec = Vec::with_capacity(range.length);
        unsafe {
            let _: () = msg_send![self, getObjects: vec.as_ptr(), range: range];
            vec.set_len(range.length);
        }
        vec
    }

    pub fn to_vec(&self) -> Vec<&T> {
        self.objects_in_range(0..self.len())
    }

    // TODO: Take Id<Self, Self::ItemOwnership> ?
    pub fn into_vec(array: Id<Self, Owned>) -> Vec<Id<T, O>> {
        array
            .to_vec()
            .into_iter()
            .map(|obj| unsafe { Id::retain(obj as *const T as *mut T).unwrap_unchecked() })
            .collect()
    }
}

impl<T: Message> NSArray<T, Shared> {
    pub fn from_slice(slice: &[Id<T, Shared>]) -> Id<Self, Shared> {
        unsafe { Id::new(from_refs(Self::class(), slice.as_slice_ref()).cast()).unwrap() }
    }

    #[doc(alias = "objectAtIndex:")]
    pub fn get_retained(&self, index: usize) -> Id<T, Shared> {
        let obj = self.get(index).unwrap();
        // SAFETY: The object is originally shared (see `where` bound).
        unsafe { Id::retain_autoreleased(obj as *const T as *mut T).unwrap_unchecked() }
    }

    pub fn to_shared_vec(&self) -> Vec<Id<T, Shared>> {
        self.to_vec()
            .into_iter()
            .map(|obj| unsafe { Id::retain(obj as *const T as *mut T).unwrap_unchecked() })
            .collect()
    }
}

impl<T: Message> NSArray<T, Owned> {
    #[doc(alias = "objectAtIndex:")]
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        // TODO: Replace this check with catching the thrown NSRangeException
        if index < self.len() {
            // SAFETY: The index is checked to be in bounds.
            Some(unsafe { msg_send![self, objectAtIndex: index] })
        } else {
            None
        }
    }

    #[doc(alias = "firstObject")]
    pub fn first_mut(&mut self) -> Option<&mut T> {
        unsafe { msg_send![self, firstObject] }
    }

    #[doc(alias = "lastObject")]
    pub fn last_mut(&mut self) -> Option<&mut T> {
        unsafe { msg_send![self, lastObject] }
    }
}

// Copying only possible when ItemOwnership = Shared

unsafe impl<T: Message> NSCopying for NSArray<T, Shared> {
    type Ownership = Shared;
    type Output = NSArray<T, Shared>;
}

unsafe impl<T: Message> NSMutableCopying for NSArray<T, Shared> {
    type Output = NSMutableArray<T, Shared>;
}

impl<T: Message> alloc::borrow::ToOwned for NSArray<T, Shared> {
    type Owned = Id<NSArray<T, Shared>, Shared>;
    fn to_owned(&self) -> Self::Owned {
        self.copy()
    }
}

unsafe impl<T: Message, O: Ownership> NSFastEnumeration for NSArray<T, O> {
    type Item = T;
}

impl<T: Message, O: Ownership> Index<usize> for NSArray<T, O> {
    type Output = T;

    fn index(&self, index: usize) -> &T {
        self.get(index).unwrap()
    }
}

impl<T: Message> DefaultId for NSArray<T, Shared> {
    type Ownership = Shared;

    #[inline]
    fn default_id() -> Id<Self, Self::Ownership> {
        Self::new()
    }
}

impl<T: Message, O: Ownership> NSMutableArray<T, O> {
    unsafe_def_fn!(pub fn new -> Owned);

    pub fn from_vec(vec: Vec<Id<T, O>>) -> Id<Self, Owned> {
        unsafe { Id::new(from_refs(Self::class(), vec.as_slice_ref()).cast()).unwrap() }
    }
}

impl<T: Message> NSMutableArray<T, Shared> {
    pub fn from_slice(slice: &[Id<T, Shared>]) -> Id<Self, Owned> {
        unsafe { Id::new(from_refs(Self::class(), slice.as_slice_ref()).cast()).unwrap() }
    }
}

impl<T: Message, O: Ownership> NSMutableArray<T, O> {
    #[doc(alias = "addObject:")]
    pub fn push(&mut self, obj: Id<T, O>) {
        // SAFETY: The object is not nil
        unsafe { msg_send![self, addObject: &*obj] }
    }

    #[doc(alias = "insertObject:atIndex:")]
    pub fn insert(&mut self, index: usize, obj: Id<T, O>) {
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
    pub fn replace(&mut self, index: usize, obj: Id<T, O>) -> Id<T, O> {
        let old_obj = unsafe {
            let obj = self.get(index).unwrap();
            Id::retain_autoreleased(obj as *const T as *mut T).unwrap_unchecked()
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
    pub fn remove(&mut self, index: usize) -> Id<T, O> {
        let obj = if let Some(obj) = self.get(index) {
            unsafe { Id::retain_autoreleased(obj as *const T as *mut T).unwrap_unchecked() }
        } else {
            panic!("removal index should be < len");
        };
        unsafe {
            let _: () = msg_send![self, removeObjectAtIndex: index];
        }
        obj
    }

    fn remove_last(&mut self) {
        unsafe { msg_send![self, removeLastObject] }
    }

    #[doc(alias = "removeLastObject")]
    pub fn pop(&mut self) -> Option<Id<T, O>> {
        self.last()
            .map(|obj| unsafe { Id::retain(obj as *const T as *mut T).unwrap_unchecked() })
            .map(|obj| {
                self.remove_last();
                obj
            })
    }

    #[doc(alias = "removeAllObjects")]
    pub fn clear(&mut self) {
        unsafe { msg_send![self, removeAllObjects] }
    }

    #[doc(alias = "sortUsingFunction:context:")]
    pub fn sort_by<F: FnMut(&T, &T) -> Ordering>(&mut self, compare: F) {
        extern "C" fn compare_with_closure<U, F: FnMut(&U, &U) -> Ordering>(
            obj1: &U,
            obj2: &U,
            context: *mut c_void,
        ) -> NSComparisonResult {
            // Bring back a reference to the closure.
            // Guaranteed to be unique, we gave `sortUsingFunction` unique is
            // ownership, and that method only runs one function at a time.
            let closure: &mut F = unsafe { (context as *mut F).as_mut().unwrap_unchecked() };

            NSComparisonResult::from((*closure)(obj1, obj2))
        }

        // We can't name the actual lifetimes in use here, so use `_`.
        // See also https://github.com/rust-lang/rust/issues/56105
        let f: extern "C" fn(_, _, *mut c_void) -> NSComparisonResult =
            compare_with_closure::<T, F>;

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

// Copying only possible when ItemOwnership = Shared

unsafe impl<T: Message> NSCopying for NSMutableArray<T, Shared> {
    type Ownership = Shared;
    type Output = NSArray<T, Shared>;
}

unsafe impl<T: Message> NSMutableCopying for NSMutableArray<T, Shared> {
    type Output = NSMutableArray<T, Shared>;
}

impl<T: Message> alloc::borrow::ToOwned for NSMutableArray<T, Shared> {
    type Owned = Id<NSMutableArray<T, Shared>, Owned>;
    fn to_owned(&self) -> Self::Owned {
        self.mutable_copy()
    }
}

unsafe impl<T: Message, O: Ownership> NSFastEnumeration for NSMutableArray<T, O> {
    type Item = T;
}

impl<T: Message, O: Ownership> Index<usize> for NSMutableArray<T, O> {
    type Output = T;

    fn index(&self, index: usize) -> &T {
        self.get(index).unwrap()
    }
}

impl<T: Message> IndexMut<usize> for NSMutableArray<T, Owned> {
    fn index_mut(&mut self, index: usize) -> &mut T {
        self.get_mut(index).unwrap()
    }
}

impl<T: Message, O: Ownership> DefaultId for NSMutableArray<T, O> {
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

    use super::{NSArray, NSMutableArray};
    use crate::{NSObject, NSString, NSValue};

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

    fn retain_count(obj: &NSObject) -> usize {
        unsafe { msg_send![obj, retainCount] }
    }

    #[test]
    fn test_two_empty() {
        let _empty_array1 = NSArray::<NSObject, _>::new();
        let _empty_array2 = NSArray::<NSObject, _>::new();
    }

    #[test]
    fn test_len() {
        let empty_array = NSArray::<NSObject, _>::new();
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

        let empty_array = <NSArray<NSObject, Shared>>::new();
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

        let vec = NSArray::into_vec(array);
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
