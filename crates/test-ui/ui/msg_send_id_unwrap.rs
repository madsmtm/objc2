//! Test calling something on return from msg_send_id!.
use objc2::rc::Id;
use objc2::runtime::NSObject;
use objc2::{class, msg_send_id};

fn main() {
    let cls = class!(NSObject);
    let _: Id<NSObject> = unsafe { msg_send_id![cls, new].unwrap() };
}
