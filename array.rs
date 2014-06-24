use runtime::{Messageable, Object};
use id::{class, ClassName, Id, FromId};
use super::{INSCopying, INSObject};

pub trait INSEnumerator<T: FromId> : INSObject {
	fn next_object(&mut self) -> Option<T> {
		unsafe {
			let obj = msg_send![self.as_ptr() nextObject];
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
		let result = unsafe {
			msg_send![self.as_ptr() count]
		};
		result as uint
	}

	fn object_at(&self, index: uint) -> T {
		unsafe {
			let result = msg_send![self.as_ptr() objectAtIndex:index];
			FromId::from_ptr(result)
		}
	}

	fn object_enumerator<'a>(&'a self) -> NSEnumerator<'a, T> {
		unsafe {
			let result = msg_send![self.as_ptr() objectEnumerator];
			FromId::from_ptr(result)
		}
	}

	unsafe fn from_ptrs(ptrs: &[*Object]) -> Self {
		let cls = class::<Self>();
		let obj = msg_send![cls.as_ptr() alloc];
		let obj = msg_send![obj initWithObjects:ptrs.as_ptr() count:ptrs.len()];
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

object_struct!(NSArray<T>)

impl<T: INSObject> INSArray<T> for NSArray<T> { }

impl<T> INSCopying<NSArray<T>> for NSArray<T> { }

impl<T: INSObject> Collection for NSArray<T> {
	fn len(&self) -> uint {
		self.count()
	}
}
