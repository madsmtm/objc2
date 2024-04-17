//! Test underspecified msg_send_id! errors.
use objc2::msg_send_id;
use objc2::rc::Id;
use objc2_foundation::NSString;

fn main() {
    let obj: &NSString;
    let _: Result<Id<NSString>, _> = unsafe { msg_send_id![obj, error: _] };
}
