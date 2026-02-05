//! Test assembly output of `define_class!`.
#![deny(unsafe_op_in_unsafe_fn)]
// Limit to Apple targets only, since we don't particularly care about GNUStep code-size for now.
#![cfg(target_vendor = "apple")]
// Limit to 64-bit since we don't do anything special on other targets, and the assembly files are _huge_.
#![cfg(target_pointer_width = "64")]

use objc2::rc::{Allocated, Retained};
use objc2::runtime::AnyClass;
use objc2::{define_class, msg_send, ClassType, Ivars};
use objc2_foundation::NSObject;

define_class!(
    #[unsafe(super(NSObject))]
    #[name = "ForgetableIvars"]
    pub struct ForgetableIvars {
        foo: u8,
        bar: u32,
    }

    impl ForgetableIvars {
        #[unsafe(method_id(init))]
        fn init(this: Allocated<Self>) -> Option<Retained<Self>> {
            let this = this.set_ivars(Ivars::<Self> { foo: 42, bar: 43 });
            unsafe { msg_send![super(this), init] }
        }
    }
);

impl ForgetableIvars {
    #[export_name = "fn_access_class"]
    pub fn access_class() -> &'static AnyClass {
        Self::class()
    }

    #[export_name = "fn_access_ivars"]
    pub fn access_ivars(&self) -> (u8, u32) {
        (*self.foo(), *self.bar())
    }
}
