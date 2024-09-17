//! Test that forgetting to annotate the return type fails
//! See https://github.com/SSheldon/rust-objc/issues/62
use objc2::{class, msg_send, msg_send_id};

fn main() {
    let cls = class!(NSObject);

    unsafe { msg_send![cls, new] };

    unsafe { msg_send_id![cls, new] };
}
