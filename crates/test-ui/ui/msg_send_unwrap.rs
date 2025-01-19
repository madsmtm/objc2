//! Test calling something on return from `msg_send!`.
use objc2::rc::Retained;
use objc2::runtime::NSObject;
use objc2::{class, msg_send};

fn main() {
    let cls = class!(NSObject);
    let _: Retained<NSObject> = unsafe { msg_send![cls, new].unwrap() };
}
