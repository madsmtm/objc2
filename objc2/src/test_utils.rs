use core::ops::{Deref, DerefMut};
use std::os::raw::c_char;
use std::sync::Once;

use crate::declare::{ClassDecl, ProtocolDecl};
use crate::encode::{Encode, Encoding};
use crate::runtime::{Class, Object, Protocol, Sel};
use crate::{ffi, MessageReceiver};

#[derive(Debug)]
pub(crate) struct CustomObject {
    obj: *mut Object,
}

impl CustomObject {
    fn new(class: &Class) -> Self {
        let ptr = class as *const Class as _;
        let obj = unsafe { ffi::class_createInstance(ptr, 0) };
        CustomObject { obj: obj as _ }
    }
}

// TODO: Remove the need for this hack
impl crate::message::private::Sealed for CustomObject {}

unsafe impl MessageReceiver for CustomObject {
    #[inline]
    fn as_raw_receiver(&self) -> *mut Object {
        self.obj
    }
}

impl Deref for CustomObject {
    type Target = Object;

    fn deref(&self) -> &Object {
        unsafe { &*self.obj }
    }
}

impl DerefMut for CustomObject {
    fn deref_mut(&mut self) -> &mut Object {
        unsafe { &mut *self.obj }
    }
}

impl Drop for CustomObject {
    fn drop(&mut self) {
        unsafe {
            #[allow(deprecated)]
            ffi::object_dispose(self.obj as _);
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
#[repr(C)]
pub(crate) struct CustomStruct {
    pub(crate) a: u64,
    pub(crate) b: u64,
    pub(crate) c: u64,
    pub(crate) d: u64,
}

unsafe impl Encode for CustomStruct {
    const ENCODING: Encoding<'static> = Encoding::Struct(
        "CustomStruct",
        &[u64::ENCODING, u64::ENCODING, u64::ENCODING, u64::ENCODING],
    );
}

pub(crate) fn custom_class() -> &'static Class {
    static REGISTER_CUSTOM_CLASS: Once = Once::new();

    REGISTER_CUSTOM_CLASS.call_once(|| {
        // The runtime will call this method, so it has to be implemented
        extern "C" fn custom_obj_class_initialize(_this: &Class, _cmd: Sel) {}

        let mut decl = ClassDecl::root("CustomObject", custom_obj_class_initialize).unwrap();
        let proto = custom_protocol();

        decl.add_protocol(proto);
        decl.add_ivar::<u32>("_foo");

        extern "C" fn custom_obj_set_foo(this: &mut Object, _cmd: Sel, foo: u32) {
            unsafe {
                this.set_ivar::<u32>("_foo", foo);
            }
        }

        extern "C" fn custom_obj_get_foo(this: &Object, _cmd: Sel) -> u32 {
            unsafe { *this.ivar::<u32>("_foo") }
        }

        extern "C" fn custom_obj_get_struct(_this: &Object, _cmd: Sel) -> CustomStruct {
            CustomStruct {
                a: 1,
                b: 2,
                c: 3,
                d: 4,
            }
        }

        extern "C" fn custom_obj_class_method(_this: &Class, _cmd: Sel) -> u32 {
            7
        }

        extern "C" fn custom_obj_set_bar(this: &mut Object, _cmd: Sel, bar: u32) {
            unsafe {
                this.set_ivar::<u32>("_foo", bar);
            }
        }

        extern "C" fn custom_obj_add_number_to_number(
            _this: &Class,
            _cmd: Sel,
            fst: i32,
            snd: i32,
        ) -> i32 {
            fst + snd
        }

        unsafe {
            let set_foo: extern "C" fn(&mut Object, Sel, u32) = custom_obj_set_foo;
            decl.add_method(sel!(setFoo:), set_foo);
            let get_foo: extern "C" fn(&Object, Sel) -> u32 = custom_obj_get_foo;
            decl.add_method(sel!(foo), get_foo);
            let get_struct: extern "C" fn(&Object, Sel) -> CustomStruct = custom_obj_get_struct;
            decl.add_method(sel!(customStruct), get_struct);
            let class_method: extern "C" fn(&Class, Sel) -> u32 = custom_obj_class_method;
            decl.add_class_method(sel!(classFoo), class_method);

            let protocol_instance_method: extern "C" fn(&mut Object, Sel, u32) = custom_obj_set_bar;
            decl.add_method(sel!(setBar:), protocol_instance_method);
            let protocol_class_method: extern "C" fn(&Class, Sel, i32, i32) -> i32 =
                custom_obj_add_number_to_number;
            decl.add_class_method(sel!(addNumber:toNumber:), protocol_class_method);
        }

        decl.register();
    });

    class!(CustomObject)
}

pub(crate) fn custom_protocol() -> &'static Protocol {
    static REGISTER_CUSTOM_PROTOCOL: Once = Once::new();

    REGISTER_CUSTOM_PROTOCOL.call_once(|| {
        let mut decl = ProtocolDecl::new("CustomProtocol").unwrap();

        decl.add_method_description::<(i32,), ()>(sel!(setBar:), true);
        decl.add_method_description::<(), *const c_char>(sel!(getName), false);
        decl.add_class_method_description::<(i32, i32), i32>(sel!(addNumber:toNumber:), true);

        decl.register();
    });

    Protocol::get("CustomProtocol").unwrap()
}

pub(crate) fn custom_subprotocol() -> &'static Protocol {
    static REGISTER_CUSTOM_SUBPROTOCOL: Once = Once::new();

    REGISTER_CUSTOM_SUBPROTOCOL.call_once(|| {
        let super_proto = custom_protocol();
        let mut decl = ProtocolDecl::new("CustomSubProtocol").unwrap();

        decl.add_protocol(super_proto);
        decl.add_method_description::<(u32,), u32>(sel!(calculateFoo:), true);

        decl.register();
    });

    Protocol::get("CustomSubProtocol").unwrap()
}

pub(crate) fn custom_object() -> CustomObject {
    CustomObject::new(custom_class())
}

pub(crate) fn custom_subclass() -> &'static Class {
    static REGISTER_CUSTOM_SUBCLASS: Once = Once::new();

    REGISTER_CUSTOM_SUBCLASS.call_once(|| {
        let superclass = custom_class();
        let mut decl = ClassDecl::new("CustomSubclassObject", superclass).unwrap();

        extern "C" fn custom_subclass_get_foo(this: &Object, _cmd: Sel) -> u32 {
            let foo: u32 = unsafe { msg_send![super(this, custom_class()), foo] };
            foo + 2
        }

        unsafe {
            let get_foo: extern "C" fn(&Object, Sel) -> u32 = custom_subclass_get_foo;
            decl.add_method(sel!(foo), get_foo);
        }

        decl.register();
    });

    class!(CustomSubclassObject)
}

pub(crate) fn custom_subclass_object() -> CustomObject {
    CustomObject::new(custom_subclass())
}
