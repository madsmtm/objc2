use std::mem;
use std::str::raw::c_str_to_static_slice;

use runtime::{Messageable, Object, Sel, objc_msgSend};
use id::{Id, FromId};
use super::INSObject;

pub trait INSString : INSObject {
	fn as_str<'a>(&'a self) -> &'a str {
		let utf8_string = Sel::register("UTF8String");
		unsafe {
			let result = objc_msgSend(self.as_ptr(), utf8_string);
			c_str_to_static_slice(mem::transmute(result))
		}
	}
}

#[deriving(Clone)]
pub struct NSString {
	ptr: Id,
}

impl Messageable for NSString {
	unsafe fn as_ptr(&self) -> *Object {
		self.ptr.as_ptr()
	}
}

impl FromId for NSString {
	unsafe fn from_id(id: Id) -> NSString {
		NSString { ptr: id }
	}
}

impl INSObject for NSString { }

impl INSString for NSString { }
