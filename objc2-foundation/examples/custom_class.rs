use std::ptr::NonNull;
use std::sync::Once;

use objc2::declare::ClassDecl;
use objc2::rc::{Id, Owned};
use objc2::runtime::{Class, Object, ObjectType, Sel};
use objc2::{msg_send, sel};
use objc2::{Encoding, Message, RefEncode};
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
unsafe impl Message for MYObject {}
unsafe impl ObjectType for MYObject {}

impl MYObject {
    fn new() -> Id<Self, Owned> {
        let cls = <Self as INSObject>::class();
        unsafe { Id::new(NonNull::new_unchecked(msg_send![cls, new])) }
    }

    fn number(&self) -> u32 {
        unsafe { *self.ivar("_number") }
    }

    fn set_number(&mut self, number: u32) {
        unsafe { self.set_ivar("_number", number) }
    }
}

static MYOBJECT_REGISTER_CLASS: Once = Once::new();

unsafe impl INSObject for MYObject {
    fn class() -> &'static Class {
        MYOBJECT_REGISTER_CLASS.call_once(|| {
            let superclass = NSObject::class();
            let mut decl = ClassDecl::new("MYObject", superclass).unwrap();
            decl.add_ivar::<u32>("_number");

            // Add ObjC methods for getting and setting the number
            extern "C" fn my_object_set_number(this: &mut Object, _cmd: Sel, number: u32) {
                unsafe { this.set_ivar("_number", number) }
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
    let number: u32 = unsafe { msg_send![obj, number] };
    println!("Number: {}", number);

    let _: () = unsafe { msg_send![obj, setNumber: 12u32] };
    println!("Number: {}", obj.number());
}
