use std::str::raw::c_str_to_static_slice;

use {class, Id};
use super::INSObject;

pub trait INSCopying<T: INSObject> : INSObject {
	fn copy(&self) -> Id<T> {
		unsafe {
			let obj = msg_send![self copy];
			Id::from_retained_ptr(obj as *Self)
		}
	}
}

pub trait INSString : INSObject {
	fn as_str<'a>(&'a self) -> &'a str {
		unsafe {
			let result = msg_send![self UTF8String];
			c_str_to_static_slice(result as *i8)
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
			Id::from_retained_ptr(obj as *Self)
		}
	}
}

object_struct!(NSString)

impl INSString for NSString { }

impl INSCopying<NSString> for NSString { }

impl Str for NSString {
	fn as_slice<'a>(&'a self) -> &'a str {
		self.as_str()
	}
}
