//! Test compiler output with invalid msg_send_id receivers.
use objc2::msg_send_id;
use objc2::runtime::{Class, Object};
use objc2::rc::{Allocated, Id, Shared};

fn main() {
    let obj: &Object;
    let _: Id<Object, Shared> = unsafe { msg_send_id![obj, new].unwrap() };
    let _: Id<Allocated<Object>, Shared> = unsafe { msg_send_id![obj, alloc].unwrap() };
    let _: Id<Object, Shared> = unsafe { msg_send_id![obj, init].unwrap() };

    let cls: &Class;
    let _: Id<Object, Shared> = unsafe { msg_send_id![cls, init].unwrap() };
    let obj: Id<Object, Shared>;
    let _: Id<Object, Shared> = unsafe { msg_send_id![obj, init].unwrap() };
    let obj: Option<Id<Object, Shared>>;
    let _: Id<Object, Shared> = unsafe { msg_send_id![obj, init].unwrap() };

    let obj: Id<Object, Shared>;
    let _: Id<Object, Shared> = unsafe { msg_send_id![obj, copy].unwrap() };
}
