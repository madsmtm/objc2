use std::str::raw::c_str_to_static_slice;

use runtime::{Messageable};
use id::{class, FromId};
use super::INSObject;

pub trait INSCopying<T: FromId> : INSObject {
	fn copy(&self) -> T {
		unsafe {
			let obj = msg_send![self.as_ptr() copy];
			FromId::from_retained_ptr(obj)
		}
	}
}

pub trait INSString : INSObject {
	fn as_str<'a>(&'a self) -> &'a str {
		unsafe {
			let result = msg_send![self.as_ptr() UTF8String];
			c_str_to_static_slice(result as *i8)
		}
	}

	fn from_str(string: &str) -> Self {
		let cls = class::<Self>();
		let utf8_encoding = 4u;
		unsafe {
			let obj = msg_send![cls.as_ptr() alloc];
			let obj = msg_send![obj initWithBytes:string.as_ptr()
			                               length:string.len()
			                             encoding:utf8_encoding];
			FromId::from_retained_ptr(obj)
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
