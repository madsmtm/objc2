use std::str::raw::c_str_to_static_slice;

use runtime::{Messageable, Sel, objc_msgSend};
use id::{class, FromId};
use super::INSObject;

pub trait INSCopying<T: FromId> : INSObject {
	fn copy(&self) -> T {
		let copy = Sel::register("copy");
		unsafe {
			let obj = objc_msgSend(self.as_ptr(), copy);
			FromId::from_retained_ptr(obj)
		}
	}
}

pub trait INSString : INSObject {
	fn as_str<'a>(&'a self) -> &'a str {
		let utf8_string = Sel::register("UTF8String");
		unsafe {
			let result = objc_msgSend(self.as_ptr(), utf8_string);
			c_str_to_static_slice(result as *i8)
		}
	}

	fn from_str(string: &str) -> Self {
		let cls = class::<Self>();
		let alloc = Sel::register("alloc");
		let init = Sel::register("initWithBytes:length:encoding:");
		let utf8_encoding = 4u;
		unsafe {
			let obj = objc_msgSend(cls.as_ptr(), alloc);
			let obj = objc_msgSend(obj, init, string.as_ptr(), string.len(),
				utf8_encoding);
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
