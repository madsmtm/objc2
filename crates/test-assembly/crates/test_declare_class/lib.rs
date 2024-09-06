//! Test assembly output of `declare_class!`.
#![deny(unsafe_op_in_unsafe_fn)]
// Limit to Apple targets only, since we don't particularly care about GNUStep code-size for now.
#![cfg(target_vendor = "apple")]
// Limit to 64-bit since we don't do anything special on other targets, and the assembly files are _huge_.
#![cfg(target_pointer_width = "64")]
use core::ptr;

use objc2::rc::{Allocated, Retained};
use objc2::runtime::AnyClass;
use objc2::{declare_class, msg_send_id, ClassType, DeclaredClass};
use objc2_foundation::{CopyingHelper, NSCopying, NSObject, NSObjectProtocol, NSZone};

declare_class!(
    #[no_mangle]
    pub struct NoIvars;

    unsafe impl ClassType for NoIvars {
        type Super = NSObject;
        const NAME: &'static str = "NoIvars";
    }

    impl DeclaredClass for NoIvars {}

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
        fn method_id(&self) -> Option<Retained<NSObject>> {
            unsafe { msg_send_id![Self::class(), new] }
        }

        // Test that `objc_autoreleaseReturnValue` is tail-called
        #[no_mangle]
        #[method_id(methodIdWithParam:)]
        fn method_id_with_param(&self, param: bool) -> Option<Retained<NSObject>> {
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
        fn copyWithZone(&self, _zone: *const NSZone) -> Option<Retained<Self>> {
            unsafe { msg_send_id![Self::class(), new] }
        }
    }
);

unsafe impl CopyingHelper for NoIvars {
    type Result = Self;
}

pub struct ForgetableIvarsIvars {
    foo: u8,
    bar: u32,
}

declare_class!(
    #[no_mangle]
    pub struct ForgetableIvars;

    unsafe impl ClassType for ForgetableIvars {
        type Super = NSObject;
        const NAME: &'static str = "ForgetableIvars";
    }

    impl DeclaredClass for ForgetableIvars {
        type Ivars = ForgetableIvarsIvars;
    }

    unsafe impl ForgetableIvars {
        #[no_mangle]
        #[method_id(init)]
        fn init_forgetable_ivars(this: Allocated<Self>) -> Option<Retained<Self>> {
            let this = this.set_ivars(ForgetableIvarsIvars { foo: 42, bar: 43 });
            unsafe { msg_send_id![super(this), init] }
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
        (self.ivars().foo, self.ivars().bar)
    }
}

pub struct DropIvarsIvars {
    obj: Retained<NSObject>,
    obj_option: Option<Retained<NSObject>>,
}

declare_class!(
    #[no_mangle]
    pub struct DropIvars;

    unsafe impl ClassType for DropIvars {
        type Super = NSObject;
        const NAME: &'static str = "DropIvars";
    }

    impl DeclaredClass for DropIvars {
        type Ivars = DropIvarsIvars;
    }

    unsafe impl DropIvars {
        #[no_mangle]
        #[method_id(init)]
        fn init_drop_ivars(this: Allocated<Self>) -> Option<Retained<Self>> {
            let this = this.set_ivars(DropIvarsIvars {
                obj: NSObject::new(),
                obj_option: Some(NSObject::new()),
            });
            unsafe { msg_send_id![super(this), init] }
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
    pub fn access_drop_ivars(&self) -> (*const NSObject, *const NSObject) {
        (
            Retained::as_ptr(&self.ivars().obj),
            self.ivars()
                .obj_option
                .as_ref()
                .map(Retained::as_ptr)
                .unwrap_or_else(ptr::null),
        )
    }
}
