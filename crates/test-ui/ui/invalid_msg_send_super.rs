//! Test invalid msg_send![super(...)] syntax
use objc2::msg_send;
use objc2::runtime::{AnyClass, NSObject};

fn main() {
    let obj: &NSObject;
    let superclass: &AnyClass;

    let _: () = unsafe { msg_send![super, init] };
    let _: () = unsafe { msg_send![super(), init] };
    let _: () = unsafe { msg_send![super(obj,), init] };
    let _: () = unsafe { msg_send![super(obj, superclass,), init] };
}
