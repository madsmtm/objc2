//! Test underspecified msg_send! errors.
use icrate::Foundation::NSString;
use objc2::msg_send;

fn main() {
    let obj: &NSString;
    let _: Result<(), _> = unsafe { msg_send![obj, error: _] };
}
