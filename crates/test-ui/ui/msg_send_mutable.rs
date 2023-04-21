//! Test that `msg_send!` consumes their arguments, including the receiver.
//!
//! Ideally, it shouldn't be so, but that's how it works currently.
use objc2::runtime::Object;
use objc2::{class, msg_send};

fn main() {
    let cls = class!(NSObject);
    let obj: &mut Object = unsafe { msg_send![cls, new] };

    let _: () = unsafe { msg_send![obj, selector] };
    // Could be solved with a reborrow
    let _: () = unsafe { msg_send![obj, selector] };
}
