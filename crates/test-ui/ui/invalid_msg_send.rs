//! Test invalid msg_send syntax
use objc2::msg_send;
use objc2::rc::Id;
use objc2::runtime::NSObject;

fn main() {
    let obj: &NSObject;
    let b = 32i32;
    let d = 32i32;
    let _: () = unsafe { msg_send![obj] };
    let _: () = unsafe { msg_send![obj,] };
    let _: () = unsafe { msg_send![obj, a:] };
    let _: () = unsafe { msg_send![obj, a: b c] };
    let _: () = unsafe { msg_send![obj, a: b: c] };
    let _: () = unsafe { msg_send![obj, a: b, c d] };
    let _: () = unsafe { msg_send![obj, a: b: c] };
    let _: () = unsafe { msg_send![obj, a: b c: d,] };

    let _: Result<(), Id<NSObject>> = unsafe { msg_send![obj, a: _, b: _] };
}
