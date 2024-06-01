//! Test underspecified msg_send_id! errors.
use objc2::msg_send_id;
use objc2::rc::Retained;
use objc2_foundation::{NSError, NSString};

fn main() {
    let obj: &NSString;
    let _: Result<_, Retained<NSError>> = unsafe { msg_send_id![obj, error: _] };
}
