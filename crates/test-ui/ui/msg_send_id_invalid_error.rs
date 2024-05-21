//! Test that msg_send_id! error handling works correctly.
use objc2::rc::{Allocated, Retained};
use objc2::runtime::{AnyObject, NSObject};
use objc2::{msg_send_id, ClassType};

fn main() {
    let obj: &NSObject;

    // Basic bad return type
    let _: () = unsafe { msg_send_id![obj, a: _] };

    // Bad return type from init
    let _: Result<Retained<AnyObject>, Retained<NSObject>> =
        unsafe { msg_send_id![NSObject::alloc(), initWithError: _] };

    // Erroring `alloc` is not supported automatically.
    let _: Result<Allocated<NSObject>, Retained<NSObject>> =
        unsafe { msg_send_id![NSObject::class(), allocWithError: _] };
}
