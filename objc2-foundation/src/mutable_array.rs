use alloc::vec::Vec;
use core::cmp::Ordering;
use core::ffi::c_void;
use core::ops::{Index, IndexMut};

use objc2::rc::{DefaultId, Id, Owned, Ownership, Shared, SliceId};
use objc2::Message;
use objc2::{msg_send, msg_send_id};

use crate::array::from_refs;
use crate::{
    NSArray, NSComparisonResult, NSCopying, NSFastEnumeration, NSMutableCopying, NSObject,
    __inner_extern_class,
};

__inner_extern_class! {
    // TODO: Ensure that this deref to NSArray is safe!
    // This "inherits" NSArray, and has the same `Send`/`Sync` impls as that.
    #[derive(Debug, PartialEq, Eq, Hash)]
    unsafe pub struct NSMutableArray<T, O: Ownership>: NSArray<T, O>, NSObject {}
}

impl<T: Message, O: Ownership> NSMutableArray<T, O> {
    pub fn new() -> Id<Self, Owned> {
        unsafe { msg_send_id![Self::class(), new].unwrap() }
    }

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
        // TODO: "C-unwind"
        extern "C" fn compare_with_closure<U, F: FnMut(&U, &U) -> Ordering>(
            obj1: &U,
            obj2: &U,
            context: *mut c_void,
        ) -> NSComparisonResult {
            // Bring back a reference to the closure.
            // Guaranteed to be unique, we gave `sortUsingFunction` unique is
            // ownership, and that method only runs one function at a time.
            let closure: &mut F = unsafe { context.cast::<F>().as_mut().unwrap_unchecked() };

            NSComparisonResult::from((*closure)(obj1, obj2))
        }

        // We can't name the actual lifetimes in use here, so use `_`.
        // See also https://github.com/rust-lang/rust/issues/56105
        let f: extern "C" fn(_, _, *mut c_void) -> NSComparisonResult =
            compare_with_closure::<T, F>;

        // Grab a type-erased pointer to the closure (a pointer to stack).
        let mut closure = compare;
        let context: *mut F = &mut closure;
        let context: *mut c_void = context.cast();

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
    use alloc::vec;

    use objc2::rc::autoreleasepool;

    use super::*;
    use crate::NSString;

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
}
