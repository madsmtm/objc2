//! Test underspecified msg_send! errors.
use objc2::msg_send;
use objc2::rc::Retained;
use objc2_foundation::{NSError, NSObject};

fn main() {
    let obj: &NSObject;
    let _: Result<_, Retained<NSError>> = unsafe { msg_send![obj, error: _] };
}
