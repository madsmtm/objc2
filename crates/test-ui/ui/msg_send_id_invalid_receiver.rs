//! Test compiler output with invalid msg_send_id receivers.
use objc2::msg_send_id;
use objc2::rc::{Allocated, Id};
use objc2::runtime::{AnyClass, NSObject};

fn main() {
    let obj: &NSObject;
    let _: Allocated<NSObject> = unsafe { msg_send_id![obj, alloc] };
    let _: Id<NSObject> = unsafe { msg_send_id![obj, init] };

    let cls: &AnyClass;
    let _: Id<NSObject> = unsafe { msg_send_id![cls, init] };
    let obj: Id<NSObject>;
    let _: Id<NSObject> = unsafe { msg_send_id![obj, init] };
    let obj: Option<Id<NSObject>>;
    let _: Id<NSObject> = unsafe { msg_send_id![obj, init] };

    let obj: Id<NSObject>;
    let _: Id<NSObject> = unsafe { msg_send_id![obj, new] };
    let obj: Id<NSObject>;
    let _: Id<NSObject> = unsafe { msg_send_id![obj, copy] };
}
