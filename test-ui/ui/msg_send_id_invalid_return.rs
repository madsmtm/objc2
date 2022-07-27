//! Test compiler output with invalid msg_send_id receivers.
use objc2::foundation::NSObject;
use objc2::msg_send_id;
use objc2::rc::{Allocated, Id, Owned, Shared};
use objc2::runtime::{Class, Object};

fn main() {
    let cls: &Class;
    let _: &Object = unsafe { msg_send_id![cls, new].unwrap() };
    let _: Id<Class, Shared> = unsafe { msg_send_id![cls, new].unwrap() };
    let _: &Object = unsafe { msg_send_id![cls, alloc].unwrap() };
    let _: Id<Allocated<Class>, Shared> = unsafe { msg_send_id![cls, alloc].unwrap() };
    let _: Id<Object, Shared> = unsafe { msg_send_id![cls, alloc].unwrap() };

    let obj: Option<Id<Allocated<Object>, Shared>>;
    let _: &Object = unsafe { msg_send_id![obj, init].unwrap() };
    let obj: Option<Id<Allocated<Object>, Shared>>;
    let _: Id<Class, Shared> = unsafe { msg_send_id![obj, init].unwrap() };
    let obj: Option<Id<Allocated<Object>, Shared>>;
    let _: Id<NSObject, Shared> = unsafe { msg_send_id![obj, init].unwrap() };
    let obj: Option<Id<Allocated<Object>, Shared>>;
    let _: Id<Object, Owned> = unsafe { msg_send_id![obj, init].unwrap() };

    let obj: Id<Object, Shared>;
    let _: &Object = unsafe { msg_send_id![&obj, description].unwrap() };
}
