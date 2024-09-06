//! Test msg_send! with mutable receivers that are not IsAllowedMutable.
use objc2::rc::Retained;
use objc2::runtime::NSObject;
use objc2::{msg_send, msg_send_id};

fn main() {
    let obj: &mut NSObject;

    let _: () = unsafe { msg_send![&mut *obj, test] };

    let _: Retained<NSObject> = unsafe { msg_send_id![obj, test] };
}
