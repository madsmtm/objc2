//! Test compiler output of msg_send_id when ownership is not specified.
use objc2::msg_send_id;
use objc2::runtime::NSObject;

fn main() {
    let obj: &NSObject;
    let _: &NSObject = &*unsafe { msg_send_id![obj, description] };
}
