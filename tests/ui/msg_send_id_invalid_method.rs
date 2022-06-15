//! Test invalid msg_send_id methods.
use objc2::msg_send_id;
use objc2::runtime::Object;

fn main() {
    let object: &Object;
    unsafe { msg_send_id![object, retain] };
    unsafe { msg_send_id![object, release] };
    unsafe { msg_send_id![object, autorelease] };
    unsafe {
        msg_send_id![
            object,
            retain,
        ]
    };
}
