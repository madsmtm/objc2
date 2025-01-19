//! Test compiler output of msg_send! when result type is not properly specified.
use objc2::msg_send;
use objc2::runtime::NSObject;

fn main() {
    let obj: &NSObject;
    let _: &NSObject = &*unsafe { msg_send![obj, description] };
}
