#![deny(unsafe_op_in_unsafe_fn)]
use std::marker::PhantomData;
use std::sync::Once;

use objc2::declare::ClassDecl;
use objc2::rc::{Id, Owned, Shared};
use objc2::runtime::{Class, Object, Sel};
use objc2::{msg_send, sel};
use objc2::{Encoding, Message, RefEncode};
use objc2_foundation::NSObject;

#[repr(C)]
pub struct MyObject<'a> {
    inner: Object,
    // `init` defaults ivars to all zeroes, so allow for that here
    // TODO: Verify this claim!
    p: PhantomData<Option<&'a mut u8>>,
}

unsafe impl RefEncode for MyObject<'_> {
    const ENCODING_REF: Encoding<'static> = Object::ENCODING_REF;
}

unsafe impl Message for MyObject<'_> {}

static MYOBJECT_REGISTER_CLASS: Once = Once::new();

impl<'a> MyObject<'a> {
    fn new(number_ptr: &'a mut u8) -> Id<Self, Owned> {
        unsafe {
            let obj: *mut Self = msg_send![Self::class(), alloc];
            let obj: *mut Self = msg_send![obj, initWithPtr: number_ptr];
            Id::new(obj).unwrap()
        }
    }

    fn get(&self) -> Option<&'a u8> {
        unsafe { *self.inner.ivar("_number_ptr") }
    }

    fn write(&mut self, number: u8) {
        let ptr = unsafe { self.inner.ivar_mut::<Option<&'a mut u8>>("_number_ptr") };
        if let Some(ptr) = ptr {
            **ptr = number;
        }
    }

    fn class() -> &'static Class {
        MYOBJECT_REGISTER_CLASS.call_once(|| {
            let superclass = NSObject::class();
            let mut decl = ClassDecl::new("MyObject", superclass).unwrap();
            decl.add_ivar::<Option<&mut u8>>("_number_ptr");

            unsafe extern "C" fn init_with_ptr(
                this: *mut Object,
                _cmd: Sel,
                ptr: *mut u8,
            ) -> *mut Object {
                let this: *mut Object = unsafe { msg_send![super(this, NSObject::class()), init] };
                if let Some(this) = unsafe { this.as_mut() } {
                    unsafe {
                        this.set_ivar("_number_ptr", ptr);
                    }
                }
                this
            }

            unsafe {
                let init_with_ptr: unsafe extern "C" fn(*mut Object, Sel, *mut u8) -> *mut Object =
                    init_with_ptr;
                decl.add_method(sel!(initWithPtr:), init_with_ptr);
            }

            decl.register();
        });

        Class::get("MyObject").unwrap()
    }
}

fn main() {
    let mut number = 54;
    let mut obj = MyObject::new(&mut number);

    println!("Number: {}", obj.get().unwrap());

    obj.write(7);
    // Won't compile, since `obj` holds a mutable reference to number
    // println!("Number: {}", number);
    println!("Number: {}", obj.get().unwrap());

    let obj: Id<_, Shared> = obj.into();
    let obj2 = obj.clone();

    println!("Number: {}", obj.get().unwrap());
    println!("Number: {}", obj2.get().unwrap());

    drop(obj);
    drop(obj2);
    println!("Number: {}", number);
}
