#![feature(phase)]

#[phase(plugin, link)]
extern crate objc;
#[phase(plugin, link)]
extern crate objc_foundation;

use objc::{ClassDecl, Id};
use objc::runtime::Object;
use objc_foundation::{class, INSObject, NSObject};

object_struct!(MYObject)

impl MYObject {
	fn number(&self) -> uint {
		let obj = unsafe {
			&*(self as *const _ as *const Object)
		};
		unsafe {
			*obj.get_ivar::<uint>("_number")
		}
	}

	fn set_number(&mut self, number: uint) {
		let obj = unsafe {
			&mut *(self as *mut _ as *mut Object)
		};
		unsafe {
			obj.set_ivar("_number", number);
		}
	}

	fn register() {
		let superclass = class::<NSObject>();
		let mut decl = ClassDecl::new(superclass, "MYObject").unwrap();

		decl.add_ivar::<uint>("_number");
		decl.add_method(method!(
			(&mut MYObject)this, setNumber:(uint)number; {
				this.set_number(number);
			}
		));
		decl.add_method(method!(
			(&MYObject)this, number -> uint {
				this.number()
			}
		));

		decl.register();
	}
}

fn main() {
	MYObject::register();
	let mut obj: Id<MYObject> = INSObject::new();
	println!("{}", obj);

	obj.set_number(7);
	println!("Number: {}", obj.number());

	unsafe {
		msg_send![obj setNumber:12u];
	}
	println!("Number: {}", obj.number());
}
