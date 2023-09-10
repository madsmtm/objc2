//! Test assembly output of `declare_class!`.
#![deny(unsafe_op_in_unsafe_fn)]
#![cfg(feature = "apple")]
use core::ptr::{self};

use icrate::Foundation::{NSCopying, NSObject};
use objc2::declare::{Ivar, IvarDrop, IvarEncode};
use objc2::rc::Id;
use objc2::runtime::{AnyClass, NSZone};
use objc2::{declare_class, msg_send, msg_send_id, mutability, ClassType};

declare_class!(
    #[no_mangle]
    pub struct Custom {
        foo: IvarEncode<u8, "_foo">,
        obj: IvarDrop<Option<Id<NSObject>>, "_obj">,
    }

    mod ivars;

    unsafe impl ClassType for Custom {
        type Super = NSObject;
        type Mutability = mutability::InteriorMutable;
        const NAME: &'static str = "CustomClassName";
    }

    unsafe impl Custom {
        #[no_mangle]
        #[method(init)]
        unsafe fn init(this: *mut Self) -> *mut Self {
            let this: Option<&mut Self> = unsafe { msg_send![super(this), init] };

            this.map(|this| {
                Ivar::write(&mut this.foo, 42);
                Ivar::write(&mut this.obj, None);
                let this: *mut Self = this;
                this
            })
            .unwrap_or_else(ptr::null_mut)
        }

        #[no_mangle]
        #[method(classMethod)]
        fn class_method() {}

        #[no_mangle]
        #[method(method)]
        fn method(&self) {}

        #[no_mangle]
        #[method(methodBool:)]
        fn method_bool(&self, val: bool) -> bool {
            !val
        }

        #[no_mangle]
        #[method_id(methodId)]
        fn method_id(&self) -> Option<Id<NSObject>> {
            self.obj.clone()
        }

        // Test that `objc_autoreleaseReturnValue` is tail-called
        #[no_mangle]
        #[method_id(methodIdWithParam:)]
        fn method_id_with_param(&self, param: bool) -> Option<Id<NSObject>> {
            // Explicitly create outside condition
            let obj = NSObject::new();
            if param {
                self.obj.clone()
            } else {
                Some(obj)
            }
        }
    }

    unsafe impl NSCopying for Custom {
        #[no_mangle]
        #[method_id(copyWithZone:)]
        fn copyWithZone(&self, _zone: *const NSZone) -> Option<Id<Self>> {
            get_obj().map(|new| {
                let hack = Id::as_ptr(&new) as *mut Self;
                let hack = unsafe { &mut *hack };

                Ivar::write(&mut hack.foo, *self.foo);
                Ivar::write(&mut hack.obj, self.obj.clone());
                new
            })
        }
    }
);

#[no_mangle]
#[inline(never)]
pub fn get_class() -> &'static AnyClass {
    Custom::class()
}

#[no_mangle]
#[inline(never)]
pub fn get_obj() -> Option<Id<Custom>> {
    unsafe { msg_send_id![get_class(), new] }
}

#[no_mangle]
#[inline(never)]
pub fn access_ivars() -> (u8, *const NSObject) {
    let obj = unsafe { get_obj().unwrap_unchecked() };
    (
        *obj.foo,
        (*obj.obj)
            .as_ref()
            .map(|obj| Id::as_ptr(&obj))
            .unwrap_or_else(ptr::null),
    )
}
