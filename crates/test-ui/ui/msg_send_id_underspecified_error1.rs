//! Test underspecified msg_send_id! errors.
use icrate::Foundation::NSString;
use objc2::msg_send_id;
use objc2::rc::Id;

fn main() {
    let obj: &NSString;
    let _: Result<Id<NSString>, _> = unsafe { msg_send_id![obj, error: _] };
}
