//! Test invalid msg_send syntax
use objc2::msg_send;
use objc2::runtime::Object;

fn main() {
    let obj: &Object;
    let b = 32i32;
    let d = 32i32;
    let _: () = unsafe { msg_send![obj] };
    let _: () = unsafe { msg_send![obj,] };
    let _: () = unsafe { msg_send![obj, a:] };
    let _: () = unsafe { msg_send![obj, a: b c] };
    let _: () = unsafe { msg_send![obj, a: b: c] };
    let _: () = unsafe { msg_send![obj, a: b, c d] };
    let _: () = unsafe { msg_send![obj, a: b: c] };
}
