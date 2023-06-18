//! Note: We can't use the `declare_class!` macro for this, it doesn't support
//! such use-cases (yet). Instead, we'll declare the class manually.
#![deny(unsafe_op_in_unsafe_fn)]
use std::marker::PhantomData;
use std::sync::Once;

use objc2::declare::{ClassBuilder, Ivar, IvarEncode, IvarType};
use objc2::mutability::Mutable;
use objc2::rc::Id;
use objc2::runtime::{AnyClass, NSObject, Sel};
use objc2::{msg_send, msg_send_id, sel};
use objc2::{ClassType, Encoding, Message, RefEncode};

/// Helper type for the instance variable
struct NumberIvar<'a> {
    // Doesn't actually matter what we put here, but we have to use the
    // lifetime parameter somehow
    p: PhantomData<&'a mut u8>,
}

unsafe impl<'a> IvarType for NumberIvar<'a> {
    type Type = IvarEncode<&'a mut u8>;
    const NAME: &'static str = "_number_ptr";
}

/// Struct that represents our custom object.
#[repr(C)]
pub struct MyObject<'a> {
    // Required to give MyObject the proper layout
    superclass: NSObject,
    // SAFETY: The ivar is declared below, and is properly initialized in the
    // designated initializer.
    //
    // Note! Attempting to acess the ivar before it has been initialized is
    // undefined behaviour!
    number: Ivar<NumberIvar<'a>>,
}

unsafe impl RefEncode for MyObject<'_> {
    const ENCODING_REF: Encoding = NSObject::ENCODING_REF;
}

unsafe impl Message for MyObject<'_> {}

impl<'a> MyObject<'a> {
    unsafe extern "C" fn init_with_ptr<'s>(
        &'s mut self,
        _cmd: Sel,
        ptr: Option<&'a mut u8>,
    ) -> Option<&'s mut Self> {
        let this: Option<&mut Self> = unsafe { msg_send![super(self), init] };
        this.map(|this| {
            // Properly initialize the number reference
            Ivar::write(&mut this.number, ptr.expect("got NULL number ptr"));
            this
        })
    }

    pub fn new(number: &'a mut u8) -> Id<Self> {
        // SAFETY: The lifetime of the reference is properly bound to the
        // returned type
        unsafe { msg_send_id![Self::alloc(), initWithPtr: number] }
    }

    pub fn get(&self) -> &u8 {
        &self.number
    }

    pub fn set(&mut self, number: u8) {
        **self.number = number;
    }
}

unsafe impl<'a> ClassType for MyObject<'a> {
    type Super = NSObject;
    type Mutability = Mutable;
    const NAME: &'static str = "MyObject";

    fn class() -> &'static AnyClass {
        // TODO: Use std::lazy::LazyCell
        static REGISTER_CLASS: Once = Once::new();

        REGISTER_CLASS.call_once(|| {
            let superclass = NSObject::class();
            let mut builder = ClassBuilder::new(Self::NAME, superclass).unwrap();

            builder.add_static_ivar::<NumberIvar<'a>>();

            unsafe {
                builder.add_method(
                    sel!(initWithPtr:),
                    Self::init_with_ptr as unsafe extern "C" fn(_, _, _) -> _,
                );
            }

            let _cls = builder.register();
        });

        AnyClass::get("MyObject").unwrap()
    }

    fn as_super(&self) -> &Self::Super {
        &self.superclass
    }

    fn as_super_mut(&mut self) -> &mut Self::Super {
        &mut self.superclass
    }
}

fn main() {
    let mut number = 54;

    let mut obj = MyObject::new(&mut number);
    assert_eq!(*obj.get(), 54);

    // It is not possible to convert to `Id<NSObject>`, since that would loose
    // the lifetime information that `MyObject` stores.
    //
    // let obj = Id::into_super(obj);
    //
    // Neither is it possible to clone or retain the object, since it is
    // marked as `Mutable` in `ClassType::Mutability`.
    //
    // let obj2 = obj.clone();
    //
    // Finally, it is not possible to access `number` any more, since `obj`
    // holds a mutable reference to it.
    //
    // assert_eq!(number, 7);

    // But we can now mutate the referenced `number`
    obj.set(7);
    assert_eq!(*obj.get(), 7);

    drop(obj);
    // And now that we've dropped `obj`, we can access `number` again
    assert_eq!(number, 7);
}
