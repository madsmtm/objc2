use std::mem::MaybeUninit;

use objc2::declare::{ClassBuilder, Ivar, IvarType};
use objc2::rc::{Id, Owned};
use objc2::runtime::{Bool, Class, Object, Sel};
use objc2::{class, msg_send, msg_send_id, sel, Encoding, Message, RefEncode};

// Helper types for the two instance variables

struct CustomIvar1;
unsafe impl IvarType for CustomIvar1 {
    type Type = i32;
    const NAME: &'static str = "ivar1";
}

struct CustomIvar2;
unsafe impl IvarType for CustomIvar2 {
    type Type = Bool;
    const NAME: &'static str = "ivar2";
}

/// Struct that represents the desired object
#[repr(C)]
pub struct CustomObject {
    inner: Object,
    // SAFETY: The instance variables are used within an object, and they are
    // correctly declared in `create_class`.
    ivar1: Ivar<CustomIvar1>,
    ivar2: Ivar<CustomIvar2>,
}

// SAFETY: `Ivar<T>` is zero-sized, so it can be ignored
unsafe impl RefEncode for CustomObject {
    const ENCODING_REF: Encoding<'static> = Encoding::Object;
}
unsafe impl Message for CustomObject {}

pub fn create_class() -> &'static Class {
    let superclass = class!(NSObject);
    let mut builder = ClassBuilder::new("CustomObject", superclass).unwrap();

    builder.add_ivar::<<CustomIvar1 as IvarType>::Type>(CustomIvar1::NAME);
    builder.add_ivar::<<CustomIvar2 as IvarType>::Type>(CustomIvar2::NAME);

    #[repr(C)]
    pub struct PartialInit {
        inner: Object,
        ivar1: Ivar<MaybeUninit<CustomIvar1>>,
        ivar2: Ivar<MaybeUninit<CustomIvar2>>,
    }
    unsafe impl RefEncode for PartialInit {
        const ENCODING_REF: Encoding<'static> = Encoding::Object;
    }
    unsafe impl Message for PartialInit {}

    impl PartialInit {
        extern "C" fn init(&mut self, _cmd: Sel) -> Option<&mut Self> {
            let this: Option<&mut Self> = unsafe { msg_send![super(self, class!(NSObject)), init] };
            this.map(|this| {
                this.ivar1.write(42);
                this.ivar2.write(Bool::from(true));
                this
            })
        }
    }

    unsafe {
        builder.add_method(sel!(init), PartialInit::init as extern "C" fn(_, _) -> _);
    }

    builder.register()
}

fn main() {
    let cls = create_class();
    let mut obj: Id<CustomObject, Owned> = unsafe { msg_send_id![cls, new].unwrap() };

    println!("Ivar1: {:?}", obj.ivar1);
    println!("Ivar2: {:?}", obj.ivar2);

    *obj.ivar1 += 1;
    *obj.ivar2 = Bool::from(false);

    println!("Ivar1: {:?}", obj.ivar1);
    println!("Ivar2: {:?}", obj.ivar2);
}
