//! Test msg_send! with mutable receivers.
use objc2::msg_send;
use objc2::rc::Retained;
use objc2::runtime::NSObject;

fn main() {
    let obj: &mut NSObject;

    let _: () = unsafe { msg_send![&mut *obj, test] };

    let _: Retained<NSObject> = unsafe { msg_send![obj, test] };
}
