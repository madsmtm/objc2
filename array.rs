use std::mem;

use runtime::{Messageable, Object, Sel, objc_msgSend};
use id::{class, ClassName, Id, FromId};
use super::{INSCopying, INSObject};

pub trait INSEnumerator<T: FromId> : INSObject {
	fn next_object(&mut self) -> Option<T> {
		let next_object = Sel::register("nextObject");
		unsafe {
			let obj = objc_msgSend(self.as_ptr(), next_object);
			FromId::maybe_from_ptr(obj)
		}
	}
}

#[deriving(Clone)]
pub struct NSEnumerator<'a, T> {
	ptr: Id,
}

impl<'a, T> Messageable for NSEnumerator<'a, T> {
	unsafe fn as_ptr(&self) -> *Object {
		self.ptr.as_ptr()
	}
}

impl<'a, T> FromId for NSEnumerator<'a, T> {
	unsafe fn from_id(id: Id) -> NSEnumerator<'a, T> {
		NSEnumerator { ptr: id }
	}
}

impl<'a, T> INSObject for NSEnumerator<'a, T> {
	fn class_name() -> ClassName<NSEnumerator<'a, T>> {
		ClassName::from_str("NSEnumerator")
	}
}

impl<'a, T: FromId> INSEnumerator<T> for NSEnumerator<'a, T> { }

impl<'a, T: FromId + Messageable> Iterator<T> for NSEnumerator<'a, T> {
	fn next(&mut self) -> Option<T> {
		self.next_object()
	}
}

pub trait INSArray<T: INSObject> : INSObject {
	fn count(&self) -> uint {
		let count = Sel::register("count");
		unsafe {
			let result = objc_msgSend(self.as_ptr(), count);
			mem::transmute(result)
		}
	}

	fn object_at(&self, index: uint) -> T {
		let object_at = Sel::register("objectAtIndex:");
		unsafe {
			let result = objc_msgSend(self.as_ptr(), object_at, index);
			FromId::from_ptr(result)
		}
	}

	fn object_enumerator<'a>(&'a self) -> NSEnumerator<'a, T> {
		let object_enumerator = Sel::register("objectEnumerator");
		unsafe {
			let result = objc_msgSend(self.as_ptr(), object_enumerator);
			FromId::from_ptr(result)
		}
	}

	unsafe fn from_ptrs(ptrs: &[*Object]) -> Self {
		let class = class::<Self>();
		let alloc = Sel::register("alloc");
		let init = Sel::register("initWithObjects:count:");

		let obj = objc_msgSend(class.as_ptr(), alloc);
		let obj = objc_msgSend(obj, init, ptrs.as_ptr(), ptrs.len());
		FromId::from_retained_ptr(obj)
	}

	fn from_slice(slice: &[T]) -> Self {
		let mut ptrs: Vec<*Object> = Vec::with_capacity(slice.len());
		for obj in slice.iter() {
			ptrs.push(unsafe { obj.as_ptr() });
		}
		unsafe {
			INSArray::from_ptrs(ptrs.as_slice())
		}
	}
}

#[deriving(Clone)]
pub struct NSArray<T> {
	ptr: Id,
}

impl<T> Messageable for NSArray<T> {
	unsafe fn as_ptr(&self) -> *Object {
		self.ptr.as_ptr()
	}
}

impl<T> FromId for NSArray<T> {
	unsafe fn from_id(id: Id) -> NSArray<T> {
		NSArray { ptr: id }
	}
}

impl<T> INSObject for NSArray<T> {
	fn class_name() -> ClassName<NSArray<T>> {
		ClassName::from_str("NSArray")
	}
}

impl<T: INSObject> INSArray<T> for NSArray<T> { }

impl<T> INSCopying<NSArray<T>> for NSArray<T> { }

impl<T: INSObject> Collection for NSArray<T> {
	fn len(&self) -> uint {
		self.count()
	}
}
