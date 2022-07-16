//! Test msg_send! syntax with commas
use objc2::rc::{Id, Shared};
use objc2::runtime::{Class, Object};
use objc2::{msg_send, msg_send_bool, msg_send_id};

fn main() {
    let obj: &Object;
    let superclass: &Class;

    let _: () = unsafe { msg_send![super(obj, superclass), a:obj b:obj] };
    let _: () = unsafe { msg_send![obj, a:obj b:obj] };

    unsafe { msg_send_bool![obj, c:obj d:obj] };

    let _: Id<Object, Shared> = unsafe { msg_send_id![obj, e:obj f:obj] };
}
