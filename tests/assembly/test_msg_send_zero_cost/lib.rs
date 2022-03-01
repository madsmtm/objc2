//! Test that the inner part of msg_send! is inlined into an objc_msgSend

use objc2::runtime::{Object, Sel};
use objc2::MessageReceiver;

#[no_mangle]
pub fn handle(obj: &Object, sel: Sel) -> *mut Object {
    unsafe { MessageReceiver::send_message(&obj, sel, ()).unwrap() }
}
