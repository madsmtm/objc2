//! Ensure that `init` returns the same type as given from `alloc`.
use objc2::rc::{Allocated, Retained};
use objc2::runtime::{AnyObject, NSObject};
use objc2::{class, msg_send_id};

fn main() {
    let cls = class!(NSObject);
    let obj: Allocated<NSObject> = unsafe { msg_send_id![cls, alloc] };

    let _: Retained<AnyObject> = unsafe { msg_send_id![obj, init] };
}
