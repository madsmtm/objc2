//! Test msg_send! with mutable receivers that are not IsAllowedMutable.
use icrate::Foundation::NSThread;
use objc2::rc::Id;
use objc2::{msg_send, msg_send_id};

fn main() {
    let obj: &mut NSThread;

    let _: () = unsafe { msg_send![&mut *obj, test] };

    let _: Id<NSThread> = unsafe { msg_send_id![obj, test] };
}
