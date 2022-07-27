use std::sync::Once;

use objc2::declare::{ClassBuilder, Ivar, IvarType};
use objc2::foundation::NSObject;
use objc2::rc::{Id, Owned};
use objc2::runtime::{Class, Object, Sel};
use objc2::{msg_send, msg_send_id, sel};
use objc2::{Encoding, Message, RefEncode};

struct NumberIvar;
unsafe impl IvarType for NumberIvar {
    type Type = u32;
    const NAME: &'static str = "_number";
}

#[repr(C)]
pub struct MYObject {
    inner: Object,
    number: Ivar<NumberIvar>,
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

    fn class() -> &'static Class {
        MYOBJECT_REGISTER_CLASS.call_once(|| {
            let superclass = NSObject::class();
            let mut builder = ClassBuilder::new("MYObject", superclass).unwrap();
            builder.add_ivar::<<NumberIvar as IvarType>::Type>(<NumberIvar as IvarType>::NAME);

            // Add ObjC methods for getting and setting the number
            extern "C" fn my_object_set_number(this: &mut MYObject, _cmd: Sel, number: u32) {
                *this.number = number;
            }

            extern "C" fn my_object_get_number(this: &MYObject, _cmd: Sel) -> u32 {
                *this.number
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

    *obj.number = 7;
    println!("Number: {}", unsafe {
        let number: u32 = msg_send![&obj, number];
        number
    });

    unsafe {
        let _: () = msg_send![&mut obj, setNumber: 12u32];
    }
    println!("Number: {}", obj.number);
}
