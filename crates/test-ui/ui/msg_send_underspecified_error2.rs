//! Test underspecified msg_send_id! errors.
use objc2::msg_send_id;
use objc2::rc::Retained;
use objc2_foundation::{NSError, NSObject};

fn main() {
    let obj: &NSObject;
    let _: Result<_, Retained<NSError>> = unsafe { msg_send_id![obj, error: _] };
}
