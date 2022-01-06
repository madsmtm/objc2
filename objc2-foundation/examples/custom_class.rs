use std::ptr::NonNull;
use std::sync::Once;

use objc2::declare::ClassDecl;
use objc2::encode::{Encoding, RefEncode};
use objc2::rc::{Id, Owned};
use objc2::runtime::{Class, Object, Sel};
use objc2::{msg_send, sel, Message};
use objc2_foundation::{INSObject, NSObject};

/// In the future this should be an `extern type`, if that gets stabilized,
/// see [RFC-1861](https://rust-lang.github.io/rfcs/1861-extern-types.html).
#[repr(C)]
pub struct MYObject {
    /// See the [Nomicon] for details on representing opaque structs.
    ///
    /// [Nomicon]: https://doc.rust-lang.org/nomicon/ffi.html#representing-opaque-structs
    _priv: [u8; 0],
}

unsafe impl RefEncode for MYObject {
    const ENCODING_REF: Encoding<'static> = Encoding::Object;
}

impl MYObject {
    fn new() -> Id<Self, Owned> {
        let cls = Self::class();
        unsafe { Id::new(NonNull::new_unchecked(msg_send![cls, new])) }
    }

    fn number(&self) -> u32 {
        unsafe {
            let obj = &*(self as *const _ as *const Object);
            *obj.ivar("_number")
        }
    }

    fn set_number(&mut self, number: u32) {
        unsafe {
            let obj = &mut *(self as *mut _ as *mut Object);
            obj.set_ivar("_number", number);
        }
    }
}

unsafe impl Message for MYObject {}

static MYOBJECT_REGISTER_CLASS: Once = Once::new();

unsafe impl INSObject for MYObject {
    fn class() -> &'static Class {
        MYOBJECT_REGISTER_CLASS.call_once(|| {
            let superclass = NSObject::class();
            let mut decl = ClassDecl::new("MYObject", superclass).unwrap();
            decl.add_ivar::<u32>("_number");

            // Add ObjC methods for getting and setting the number
            extern "C" fn my_object_set_number(this: &mut Object, _cmd: Sel, number: u32) {
                unsafe {
                    this.set_ivar("_number", number);
                }
            }

            extern "C" fn my_object_get_number(this: &Object, _cmd: Sel) -> u32 {
                unsafe { *this.ivar("_number") }
            }

            unsafe {
                let set_number: extern "C" fn(&mut Object, Sel, u32) = my_object_set_number;
                decl.add_method(sel!(setNumber:), set_number);
                let get_number: extern "C" fn(&Object, Sel) -> u32 = my_object_get_number;
                decl.add_method(sel!(number), get_number);
            }

            decl.register();
        });

        Class::get("MYObject").unwrap()
    }
}

fn main() {
    let mut obj = MYObject::new();

    obj.set_number(7);
    println!("Number: {}", unsafe {
        let number: u32 = msg_send![obj, number];
        number
    });

    unsafe {
        let _: () = msg_send![obj, setNumber: 12u32];
    }
    println!("Number: {}", obj.number());
}
