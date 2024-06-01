//! Test underspecified msg_send! errors.
use objc2::msg_send;
use objc2_foundation::NSString;

fn main() {
    let obj: &NSString;
    let _: Result<(), _> = unsafe { msg_send![obj, error: _] };
}
