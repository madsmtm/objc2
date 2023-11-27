//! Test assembly output of `declare_class!`.
#![deny(unsafe_op_in_unsafe_fn)]
// Limit to Apple targets only, since we don't particularly care about GNUStep code-size for now.
#![cfg(feature = "apple")]
// Limit to 64-bit since we don't do anything special on other targets, and the assembly files are _huge_.
#![cfg(target_pointer_width = "64")]
use core::ptr;

use icrate::Foundation::{NSCopying, NSObject, NSObjectProtocol, NSZone};
use objc2::declare::{Ivar, IvarDrop, IvarEncode};
use objc2::rc::Id;
use objc2::runtime::AnyClass;
use objc2::{declare_class, msg_send, msg_send_id, mutability, ClassType};

declare_class!(
    #[no_mangle]
    pub struct NoIvars;

    unsafe impl ClassType for NoIvars {
        type Super = NSObject;
        type Mutability = mutability::InteriorMutable;
        const NAME: &'static str = "NoIvars";
    }

    unsafe impl NoIvars {
        #[no_mangle]
        #[method(classMethod)]
        fn get_class() -> &'static AnyClass {
            Self::class()
        }

        #[no_mangle]
        #[method(method)]
        fn method_simple(&self) {}

        #[no_mangle]
        #[method(methodBool:)]
        fn method_bool(&self, val: bool) -> bool {
            !val
        }

        #[no_mangle]
        #[method_id(methodId)]
        fn method_id(&self) -> Option<Id<NSObject>> {
            unsafe { msg_send_id![Self::class(), new] }
        }

        // Test that `objc_autoreleaseReturnValue` is tail-called
        #[no_mangle]
        #[method_id(methodIdWithParam:)]
        fn method_id_with_param(&self, param: bool) -> Option<Id<NSObject>> {
            // Intentionally create this outside condition
            let obj = NSObject::new();
            if param {
                Some(NSObject::new())
            } else {
                Some(obj)
            }
        }
    }

    unsafe impl NSObjectProtocol for NoIvars {}

    unsafe impl NSCopying for NoIvars {
        #[no_mangle]
        #[method_id(copyWithZone:)]
        fn copyWithZone(&self, _zone: *const NSZone) -> Option<Id<Self>> {
            unsafe { msg_send_id![Self::class(), new] }
        }
    }
);

declare_class!(
    #[no_mangle]
    pub struct ForgetableIvars {
        foo: IvarEncode<u8, "_foo">,
        bar: IvarEncode<u32, "_bar">,
    }

    mod forgetable_ivars;

    unsafe impl ClassType for ForgetableIvars {
        type Super = NSObject;
        type Mutability = mutability::InteriorMutable;
        const NAME: &'static str = "ForgetableIvars";
    }

    unsafe impl ForgetableIvars {
        #[no_mangle]
        #[method(init)]
        unsafe fn init_forgetable_ivars(this: *mut Self) -> *mut Self {
            let this: Option<&mut Self> = unsafe { msg_send![super(this), init] };

            this.map(|this| {
                Ivar::write(&mut this.foo, 42);
                Ivar::write(&mut this.bar, 43);
                let this: *mut Self = this;
                this
            })
            .unwrap_or_else(ptr::null_mut)
        }
    }
);

impl ForgetableIvars {
    #[no_mangle]
    pub fn access_forgetable_ivars_class() -> &'static AnyClass {
        Self::class()
    }

    #[no_mangle]
    pub fn access_forgetable_ivars(&self) -> (u8, u32) {
        (*self.foo, *self.bar)
    }
}

declare_class!(
    #[no_mangle]
    pub struct DropIvars {
        obj: IvarDrop<Id<NSObject>, "_obj">,
        obj_option: IvarDrop<Option<Id<NSObject>>, "_obj_option">,
    }

    mod drop_ivars;

    unsafe impl ClassType for DropIvars {
        type Super = NSObject;
        type Mutability = mutability::InteriorMutable;
        const NAME: &'static str = "DropIvars";
    }

    unsafe impl DropIvars {
        #[no_mangle]
        #[method(init)]
        unsafe fn init_drop_ivars(this: *mut Self) -> *mut Self {
            let this: Option<&mut Self> = unsafe { msg_send![super(this), init] };

            this.map(|this| {
                Ivar::write(&mut this.obj, NSObject::new());
                Ivar::write(&mut this.obj_option, Some(NSObject::new()));
                let this: *mut Self = this;
                this
            })
            .unwrap_or_else(ptr::null_mut)
        }
    }
);

impl Drop for DropIvars {
    #[inline(never)]
    fn drop(&mut self) {
        std::hint::black_box(());
    }
}

impl DropIvars {
    #[no_mangle]
    pub fn access_drop_ivars_class() -> &'static AnyClass {
        Self::class()
    }

    #[no_mangle]
    pub fn access_drop_ivars(&self) -> *const NSObject {
        Id::as_ptr(&*self.obj)
    }
}
