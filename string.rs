use std::str::raw::c_str_to_static_slice;

use {class, Id, Identifier};
use super::INSObject;

pub trait INSCopying<T: INSObject> : INSObject {
	fn copy(&self) -> Id<T> {
		unsafe {
			let obj = msg_send![self copy];
			Identifier::from_retained_ptr(obj as *mut T)
		}
	}
}

pub trait INSString : INSObject {
	fn as_str(&self) -> &str {
		unsafe {
			let result = msg_send![self UTF8String];
			c_str_to_static_slice(result as *const i8)
		}
	}

	fn from_str(string: &str) -> Id<Self> {
		let cls = class::<Self>();
		let utf8_encoding = 4u;
		unsafe {
			let obj = msg_send![cls alloc];
			let obj = msg_send![obj initWithBytes:string.as_ptr()
			                               length:string.len()
			                             encoding:utf8_encoding];
			Identifier::from_retained_ptr(obj as *mut Self)
		}
	}
}

object_struct!(NSString)

impl INSString for NSString { }

impl INSCopying<NSString> for NSString { }

impl Str for NSString {
	fn as_slice(&self) -> &str {
		self.as_str()
	}
}
