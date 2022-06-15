//! Test invalid msg_send_id methods.
//!
//! The `__msg_send_id_helper!` macro is unfortunately leaked, but I think
//! this is better than having it show up as part of the `msg_send_id!` macro
//! itself!
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
