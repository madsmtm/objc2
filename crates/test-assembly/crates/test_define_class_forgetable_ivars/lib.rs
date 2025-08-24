//! Test assembly output of `define_class!`.
#![deny(unsafe_op_in_unsafe_fn)]
// Limit to Apple targets only, since we don't particularly care about GNUStep code-size for now.
#![cfg(target_vendor = "apple")]
// Limit to 64-bit since we don't do anything special on other targets, and the assembly files are _huge_.
#![cfg(target_pointer_width = "64")]

use objc2::rc::{Allocated, Retained};
use objc2::runtime::AnyClass;
use objc2::{define_class, msg_send, ClassType, DefinedClass};
use objc2_foundation::NSObject;

pub struct Ivars {
    foo: u8,
    bar: u32,
}

define_class!(
    #[no_mangle]
    #[unsafe(super(NSObject))]
    #[name = "ForgetableIvars"]
    #[ivars = Ivars]
    pub struct ForgetableIvars;

    impl ForgetableIvars {
        #[export_name = "fn1_init"]
        #[unsafe(method_id(init))]
        fn init(this: Allocated<Self>) -> Option<Retained<Self>> {
            let this = this.set_ivars(Ivars { foo: 42, bar: 43 });
            unsafe { msg_send![super(this), init] }
        }
    }
);

impl ForgetableIvars {
    #[export_name = "fn2_access_class"]
    pub fn access_class() -> &'static AnyClass {
        Self::class()
    }

    #[export_name = "fn3_access_ivars"]
    pub fn access_ivars(&self) -> (u8, u32) {
        (self.ivars().foo, self.ivars().bar)
    }
}
