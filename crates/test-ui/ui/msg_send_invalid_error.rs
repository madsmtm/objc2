//! Test that message sending with error handling works correctly.
use objc2::rc::{Allocated, Retained};
use objc2::runtime::{AnyObject, NSObject};
use objc2::{msg_send, AnyThread, ClassType};
use objc2_foundation::NSString;

fn main() {
    let obj: &NSString;

    // Wrong type
    let _: () = unsafe { msg_send![obj, a: _] };
    let _: Result<i32, _> = unsafe { msg_send![obj, b: _] };
    let _: Result<(), i32> = unsafe { msg_send![obj, c: _] };
    let _: Result<(), Retained<i32>> = unsafe { msg_send![obj, d: _] };

    // Different calls
    let _: () = unsafe { msg_send![obj, e: obj, f: _] };
    let _: () = unsafe { msg_send![super(obj), g: _] };
    let _: () = unsafe { msg_send![super(obj, NSString::class()), h: _] };

    // Bad return type from init
    let _: Result<Retained<AnyObject>, Retained<NSObject>> =
        unsafe { msg_send![NSObject::alloc(), initWithError: _] };

    // Erroring `alloc` is not supported automatically.
    let _: Result<Allocated<NSObject>, Retained<NSObject>> =
        unsafe { msg_send![NSObject::class(), allocWithError: _] };
}
