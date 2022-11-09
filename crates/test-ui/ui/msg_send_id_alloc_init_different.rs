//! Ensure that `init` returns the same type as given from `alloc`.
use objc2::foundation::NSObject;
use objc2::{class, msg_send_id};
use objc2::rc::{Allocated, Id, Shared};
use objc2::runtime::Object;

fn main() {
    let cls = class!(NSObject);
    let obj: Option<Allocated<NSObject>> = unsafe { msg_send_id![cls, alloc] };

    let _: Id<Object, Shared> = unsafe { msg_send_id![obj, init] };
}
