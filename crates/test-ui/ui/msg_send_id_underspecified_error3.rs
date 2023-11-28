//! Test underspecified msg_send_id! errors.
use icrate::Foundation::NSString;
use objc2::msg_send_id;

fn main() {
    let obj: &NSString;
    let _: Result<_, _> = unsafe { msg_send_id![obj, error: _] };
}
