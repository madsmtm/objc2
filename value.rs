use std::mem;
use std::str::raw::c_str_to_static_slice;

use {class, encode, Encode, Id};
use super::{INSCopying, INSObject};

pub trait INSValue<T: Copy + Encode> : INSObject {
	fn value(&self) -> T {
		unsafe {
			let value = mem::uninitialized::<T>();
			msg_send![self getValue:&value];
			value
		}
	}

	fn encoding(&self) -> &str {
		unsafe {
			let result = msg_send![self objCType] as *const i8;
			c_str_to_static_slice(result)
		}
	}

	fn from_value(value: &T) -> Id<Self> {
		let cls = class::<Self>();
		let encoding = encode::<T>();
		encoding.with_c_str(|encoding| unsafe {
			let obj = msg_send![cls alloc];
			let obj = msg_send![obj initWithBytes:value objCType:encoding];
			Id::from_retained_ptr(obj as *mut Self)
		})
	}
}

object_struct!(NSValue<T>)

impl<T: Copy + Encode> INSValue<T> for NSValue<T> { }

impl<T> INSCopying<NSValue<T>> for NSValue<T> { }

#[cfg(test)]
mod tests {
	use {encode, Id};
	use super::{INSValue, NSValue};

	#[test]
	fn test_value() {
		let val: Id<NSValue<uint>> = INSValue::from_value(&13);
		assert!(val.value() == 13);
		assert!(val.encoding() == encode::<uint>());
	}
}
