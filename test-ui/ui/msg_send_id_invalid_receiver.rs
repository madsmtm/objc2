//! Test compiler output with invalid msg_send_id receivers.
use objc2::msg_send_id;
use objc2::rc::{Allocated, Id, Shared};
use objc2::runtime::{Class, Object};

fn main() {
    let obj: &Object;
    let _: Id<Allocated<Object>, Shared> = unsafe { msg_send_id![obj, alloc] };
    let _: Id<Object, Shared> = unsafe { msg_send_id![obj, init] };

    let cls: &Class;
    let _: Id<Object, Shared> = unsafe { msg_send_id![cls, init] };
    let obj: Id<Object, Shared>;
    let _: Id<Object, Shared> = unsafe { msg_send_id![obj, init] };
    let obj: Option<Id<Object, Shared>>;
    let _: Id<Object, Shared> = unsafe { msg_send_id![obj, init] };

    let obj: Id<Object, Shared>;
    let _: Id<Object, Shared> = unsafe { msg_send_id![obj, new] };
    let obj: Id<Object, Shared>;
    let _: Id<Object, Shared> = unsafe { msg_send_id![obj, copy] };
}
