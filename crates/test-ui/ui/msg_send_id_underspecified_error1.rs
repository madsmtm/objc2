//! Test underspecified msg_send_id! errors.
use objc2::msg_send_id;
use objc2::rc::Retained;
use objc2::runtime::NSObject;

fn main() {
    let obj: &NSObject;
    let _: Result<Retained<NSObject>, _> = unsafe { msg_send_id![obj, error: _] };
}
