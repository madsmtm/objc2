use std::kinds::marker::{ContravariantLifetime, NoCopy};
use std::mem;

use runtime::Object;
use {class, Id, IdVector, IntoIdVector, Owned, Ownership, Shared, ShareId};
use super::{INSCopying, INSMutableCopying, INSObject};

pub struct NSRange {
	pub location: uint,
	pub length: uint,
}

pub struct NSEnumerator<'a, T> {
	id: Id<Object>,
	marker: ContravariantLifetime<'a>,
}

impl<'a, T> NSEnumerator<'a, T> {
	pub unsafe fn from_ptr(ptr: *mut Object) -> NSEnumerator<'a, T> {
		NSEnumerator { id: Id::from_ptr(ptr), marker: ContravariantLifetime }
	}
}

impl<'a, T> Iterator<&'a T> for NSEnumerator<'a, T> {
	fn next(&mut self) -> Option<&'a T> {
		unsafe {
			let obj = msg_send![self.id nextObject] as *mut T;
			if obj.is_null() {
				None
			} else {
				Some(&*obj)
			}
		}
	}
}

pub trait INSArray<T: INSObject, O: Ownership> : INSObject {
	fn count(&self) -> uint {
		let result = unsafe {
			msg_send![self count]
		};
		result as uint
	}

	fn object_at(&self, index: uint) -> &T {
		unsafe {
			let obj = msg_send![self objectAtIndex:index] as *const T;
			&*obj
		}
	}

	fn first_object(&self) -> Option<&T> {
		unsafe {
			let obj = msg_send![self firstObject] as *const T;
			if obj.is_null() {
				None
			} else {
				Some(&*obj)
			}
		}
	}

	fn last_object(&self) -> Option<&T> {
		unsafe {
			let obj = msg_send![self lastObject] as *const T;
			if obj.is_null() {
				None
			} else {
				Some(&*obj)
			}
		}
	}

	fn object_enumerator(&self) -> NSEnumerator<T> {
		unsafe {
			let result = msg_send![self objectEnumerator];
			NSEnumerator::from_ptr(result)
		}
	}

	unsafe fn from_refs(refs: &[&T]) -> Id<Self> {
		let cls = class::<Self>();
		let obj = msg_send![cls alloc];
		let obj = msg_send![obj initWithObjects:refs.as_ptr() count:refs.len()];
		Id::from_retained_ptr(obj as *mut Self)
	}

	fn from_vec(vec: Vec<Id<T, O>>) -> Id<Self> {
		let refs = vec.as_refs_slice();
		unsafe {
			INSArray::from_refs(refs)
		}
	}

	fn objects_in_range(&self, start: uint, len: uint) -> Vec<&T> {
		let vec: Vec<*mut T> = Vec::from_elem(len, RawPtr::null());
		let range = NSRange { location: start, length: len };
		unsafe {
			msg_send![self getObjects:vec.as_ptr() range:range];
			mem::transmute(vec)
		}
	}

	fn to_vec(&self) -> Vec<&T> {
		self.objects_in_range(0, self.count())
	}

	fn into_vec(array: Id<Self>) -> Vec<Id<T, O>> {
		let vec = array.to_vec();
		unsafe {
			vec.into_id_vec()
		}
	}
}

pub trait INSOwnedArray<T: INSObject> : INSArray<T, Owned> {
	fn mut_object_at(&mut self, index: uint) -> &mut T {
		unsafe {
			let result = msg_send![self objectAtIndex:index] as *mut T;
			&mut *result
		}
	}
}

pub trait INSSharedArray<T: INSObject> : INSArray<T, Shared> {
	fn shared_object_at(&self, index: uint) -> ShareId<T> {
		let obj = self.object_at(index);
		unsafe {
			Id::from_ptr(obj as *const _ as *mut T)
		}
	}

	fn from_slice(slice: &[ShareId<T>]) -> Id<Self> {
		let refs = slice.as_refs_slice();
		unsafe {
			INSArray::from_refs(refs)
		}
	}

	fn to_shared_vec(&self) -> Vec<ShareId<T>> {
		let vec = self.to_vec();
		unsafe {
			vec.into_id_vec()
		}
	}
}

pub struct NSArray<T, O = Owned> {
	nocopy: NoCopy,
}

object_impl!(NSArray<T, O>)

impl<T: INSObject, O: Ownership> INSArray<T, O> for NSArray<T, O> { }

impl<T: INSObject> INSOwnedArray<T> for NSArray<T, Owned> { }

impl<T: INSObject> INSSharedArray<T> for NSArray<T, Shared> { }

impl<T> INSCopying<NSSharedArray<T>> for NSArray<T, Shared> { }

impl<T> INSMutableCopying<NSMutableSharedArray<T>> for NSArray<T, Shared> { }

impl<T: INSObject, O: Ownership> Collection for NSArray<T, O> {
	fn len(&self) -> uint {
		self.count()
	}
}

impl<T: INSObject, O: Ownership> Index<uint, T> for NSArray<T, O> {
	fn index(&self, index: &uint) -> &T {
		self.object_at(*index)
	}
}

pub type NSSharedArray<T> = NSArray<T, Shared>;

pub trait INSMutableArray<T: INSObject, O: Ownership> : INSArray<T, O> {
	fn add_object(&mut self, obj: Id<T, O>) {
		unsafe {
			msg_send![self addObject:obj];
		}
	}

	fn insert_object_at(&mut self, index: uint, obj: Id<T, O>) {
		unsafe {
			msg_send![self insertObject:obj atIndex:index];
		}
	}

	fn replace_object_at(&mut self, index: uint, obj: Id<T, O>) -> Id<T, O> {
		let old_obj = unsafe {
			let obj = self.object_at(index);
			Id::from_ptr(obj as *const _ as *mut T)
		};
		unsafe {
			msg_send![self replaceObjectAtIndex:index withObject:obj];
		}
		old_obj
	}

	fn remove_object_at(&mut self, index: uint) -> Id<T, O> {
		let obj = unsafe {
			let obj = self.object_at(index);
			Id::from_ptr(obj as *const _ as *mut T)
		};
		unsafe {
			msg_send![self removeObjectAtIndex:index];
		}
		obj
	}

	fn remove_last_object(&mut self) -> Id<T, O> {
		let obj = self.last_object().map(|obj| unsafe {
			Id::from_ptr(obj as *const _ as *mut T)
		});
		unsafe {
			msg_send![self removeLastObject];
		}
		// removeLastObject would have failed if the array is empty,
		// so we know this won't be None
		obj.unwrap()
	}

	fn remove_all_objects(&mut self) {
		unsafe {
			msg_send![self removeAllObjects];
		}
	}
}

pub struct NSMutableArray<T, O = Owned> {
	nocopy: NoCopy,
}

object_impl!(NSMutableArray<T, O>)

impl<T: INSObject, O: Ownership> INSArray<T, O> for NSMutableArray<T, O> { }

impl<T: INSObject> INSOwnedArray<T> for NSMutableArray<T, Owned> { }

impl<T: INSObject> INSSharedArray<T> for NSMutableArray<T, Shared> { }

impl<T: INSObject, O: Ownership> INSMutableArray<T, O> for NSMutableArray<T, O> { }

impl<T> INSCopying<NSSharedArray<T>> for NSMutableArray<T, Shared> { }

impl<T> INSMutableCopying<NSMutableSharedArray<T>> for NSMutableArray<T, Shared> { }

impl<T: INSObject, O: Ownership> Collection for NSMutableArray<T, O> {
	fn len(&self) -> uint {
		self.count()
	}
}

impl<T: INSObject, O: Ownership> Mutable for NSMutableArray<T, O> {
	fn clear(&mut self) {
		self.remove_all_objects();
	}
}

impl<T: INSObject, O: Ownership> MutableSeq<Id<T, O>> for NSMutableArray<T, O> {
	fn push(&mut self, value: Id<T, O>) {
		self.add_object(value);
	}

	fn pop(&mut self) -> Option<Id<T, O>> {
		if self.is_empty() {
			None
		} else {
			Some(self.remove_last_object())
		}
	}
}

impl<T: INSObject, O: Ownership> Index<uint, T> for NSMutableArray<T, O> {
	fn index(&self, index: &uint) -> &T {
		self.object_at(*index)
	}
}

pub type NSMutableSharedArray<T> = NSMutableArray<T, Shared>;

#[cfg(test)]
mod tests {
	use {Id};
	use foundation::{INSObject, NSObject};
	use super::{INSArray, INSMutableArray, NSArray, NSMutableArray};

	fn sample_array(len: uint) -> Id<NSArray<NSObject>> {
		let vec: Vec<Id<NSObject>> = Vec::from_fn(len, |_| INSObject::new());
		INSArray::from_vec(vec)
	}

	#[test]
	fn test_count() {
		let empty_array: Id<NSArray<NSObject>> = INSObject::new();
		assert!(empty_array.count() == 0);

		let array = sample_array(4);
		assert!(array.count() == 4);
	}

	#[test]
	fn test_object_at() {
		let array = sample_array(4);
		assert!(array.object_at(0) != array.object_at(3));
		assert!(array.first_object().unwrap() == array.object_at(0));
		assert!(array.last_object().unwrap() == array.object_at(3));

		let empty_array: Id<NSArray<NSObject>> = INSObject::new();
		assert!(empty_array.first_object().is_none());
		assert!(empty_array.last_object().is_none());
	}

	#[test]
	fn test_object_enumerator() {
		let array = sample_array(4);

		assert!(array.object_enumerator().count() == 4);
		assert!(array.object_enumerator()
		             .enumerate()
		             .all(|(i, obj)| obj == array.object_at(i)));
	}

	#[test]
	fn test_objects_in_range() {
		let array = sample_array(4);

		let middle_objs = array.objects_in_range(1, 2);
		assert!(middle_objs.len() == 2);
		assert!(*middle_objs.get(0) == array.object_at(1));
		assert!(*middle_objs.get(1) == array.object_at(2));

		let empty_objs = array.objects_in_range(1, 0);
		assert!(empty_objs.len() == 0);

		let all_objs = array.objects_in_range(0, 4);
		assert!(all_objs.len() == 4);
	}

	#[test]
	fn test_into_vec() {
		let array = sample_array(4);

		let vec = INSArray::into_vec(array);
		assert!(vec.len() == 4);
	}

	#[test]
	fn test_add_object() {
		let mut array: Id<NSMutableArray<NSObject>> = INSObject::new();
		let obj: Id<NSObject> = INSObject::new();
		array.deref_mut().add_object(obj);

		assert!(array.len() == 1);
		assert!(array.object_at(0) == array.object_at(0));

		let obj: Id<NSObject> = INSObject::new();
		array.deref_mut().insert_object_at(0, obj);
		assert!(array.len() == 2);
	}

	#[test]
	fn test_replace_object() {
		let mut array: Id<NSMutableArray<NSObject>> = INSObject::new();
		let obj: Id<NSObject> = INSObject::new();
		array.deref_mut().add_object(obj);

		let obj: Id<NSObject> = INSObject::new();
		let old_obj = array.deref_mut().replace_object_at(0, obj);
		assert!(&*old_obj != array.object_at(0));
	}

	#[test]
	fn test_remove_object() {
		let mut array: Id<NSMutableArray<NSObject>> = INSObject::new();
		for _ in range(0u, 4) {
			let obj: Id<NSObject> = INSObject::new();
			array.deref_mut().add_object(obj);
		}

		array.deref_mut().remove_object_at(1);
		assert!(array.len() == 3);

		array.deref_mut().remove_last_object();
		assert!(array.len() == 2);

		array.deref_mut().remove_all_objects();
		assert!(array.is_empty());
	}
}
