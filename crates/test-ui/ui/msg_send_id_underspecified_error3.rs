//! Test underspecified msg_send_id! errors.
use objc2::msg_send_id;
use objc2_foundation::NSString;

fn main() {
    let obj: &NSString;
    let _: Result<_, _> = unsafe { msg_send_id![obj, error: _] };
}
