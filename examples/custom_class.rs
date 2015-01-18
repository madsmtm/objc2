#[macro_use]
extern crate objc;
#[macro_use]
extern crate objc_foundation;

use objc::{ClassDecl, Id};
use objc::runtime::Object;
use objc_foundation::{class, INSObject, NSObject};

object_struct!(MYObject);

impl MYObject {
    fn number(&self) -> u32 {
        let obj = unsafe {
            &*(self as *const _ as *const Object)
        };
        unsafe {
            *obj.get_ivar::<u32>("_number")
        }
    }

    fn set_number(&mut self, number: u32) {
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

        decl.add_ivar::<u32>("_number");
        decl.add_method(method!(
            (&mut MYObject)this, setNumber:(u32)number; {
                this.set_number(number);
            }
        ));
        decl.add_method(method!(
            (&MYObject)this, number -> u32, {
                this.number()
            }
        ));

        decl.register();
    }
}

fn main() {
    MYObject::register();
    let mut obj: Id<MYObject> = INSObject::new();
    println!("{:?}", obj);

    obj.set_number(7);
    println!("Number: {}", obj.number());

    unsafe {
        let _: () = msg_send![obj, setNumber:12u32];
    }
    println!("Number: {}", obj.number());
}
