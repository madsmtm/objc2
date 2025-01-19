//! Test underspecified msg_send! errors.
use objc2::msg_send;
use objc2::runtime::NSObject;

fn main() {
    let obj: &NSObject;
    let _: Result<_, _> = unsafe { msg_send![obj, error: _] };
}
