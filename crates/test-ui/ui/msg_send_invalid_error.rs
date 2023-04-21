//! Test that msg_send! error handling works correctly.
use icrate::Foundation::NSString;
use objc2::rc::Id;
use objc2::{msg_send, msg_send_id, ClassType};

fn main() {
    let obj: &NSString;

    // Wrong type
    let _: () = unsafe { msg_send![obj, a: _] };
    let _: Result<i32, _> = unsafe { msg_send![obj, b: _] };
    let _: Result<(), i32> = unsafe { msg_send![obj, c: _] };
    let _: Result<(), Id<i32>> = unsafe { msg_send![obj, d: _] };

    // Different calls
    let _: () = unsafe { msg_send![obj, e: obj, f: _] };
    let _: () = unsafe { msg_send![super(obj), g: _] };
    let _: () = unsafe { msg_send![super(obj, NSString::class()), h: _] };
    let _: () = unsafe { msg_send_id![obj, i: _] };
}
