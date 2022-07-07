use std::sync::Once;

use objc2::declare::ClassBuilder;
use objc2::rc::{Id, Owned};
use objc2::runtime::{Class, Object, Sel};
use objc2::{msg_send, msg_send_id, sel};
use objc2::{Encoding, Message, RefEncode};
use objc2_foundation::NSObject;

/// In the future this should be an `extern type`, if that gets stabilized,
/// see [RFC-1861](https://rust-lang.github.io/rfcs/1861-extern-types.html).
#[repr(C)]
pub struct MYObject {
    inner: Object,
}

unsafe impl RefEncode for MYObject {
    const ENCODING_REF: Encoding<'static> = Encoding::Object;
}

unsafe impl Message for MYObject {}

static MYOBJECT_REGISTER_CLASS: Once = Once::new();

impl MYObject {
    fn new() -> Id<Self, Owned> {
        let cls = Self::class();
        unsafe { msg_send_id![cls, new].unwrap() }
    }

    fn number(&self) -> u32 {
        unsafe { *self.inner.ivar("_number") }
    }

    fn set_number(&mut self, number: u32) {
        unsafe { self.inner.set_ivar("_number", number) };
    }

    fn class() -> &'static Class {
        MYOBJECT_REGISTER_CLASS.call_once(|| {
            let superclass = NSObject::class();
            let mut builder = ClassBuilder::new("MYObject", superclass).unwrap();
            builder.add_ivar::<u32>("_number");

            // Add ObjC methods for getting and setting the number
            extern "C" fn my_object_set_number(this: &mut MYObject, _cmd: Sel, number: u32) {
                this.set_number(number);
            }

            extern "C" fn my_object_get_number(this: &MYObject, _cmd: Sel) -> u32 {
                this.number()
            }

            unsafe {
                let set_number: extern "C" fn(_, _, _) = my_object_set_number;
                builder.add_method(sel!(setNumber:), set_number);
                let get_number: extern "C" fn(_, _) -> _ = my_object_get_number;
                builder.add_method(sel!(number), get_number);
            }

            builder.register();
        });

        Class::get("MYObject").unwrap()
    }
}

fn main() {
    let mut obj = MYObject::new();

    obj.set_number(7);
    println!("Number: {}", unsafe {
        let number: u32 = msg_send![&obj, number];
        number
    });

    unsafe {
        let _: () = msg_send![&mut obj, setNumber: 12u32];
    }
    println!("Number: {}", obj.number());
}
