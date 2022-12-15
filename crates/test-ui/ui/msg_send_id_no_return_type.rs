//! Test that forgetting to annotate the return type fails
use objc2::runtime::NSObject;
use objc2::{msg_send_id, ClassType};

fn main() {
    unsafe { msg_send_id![NSObject::class(), new] };
}
