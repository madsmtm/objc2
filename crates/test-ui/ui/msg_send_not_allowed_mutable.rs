//! Test msg_send! with mutable receivers that are not IsAllowedMutable.
use objc2::rc::Retained;
use objc2::{msg_send, msg_send_id};
use objc2_foundation::NSThread;

fn main() {
    let obj: &mut NSThread;

    let _: () = unsafe { msg_send![&mut *obj, test] };

    let _: Retained<NSThread> = unsafe { msg_send_id![obj, test] };
}
