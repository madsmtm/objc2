//! Test underspecified msg_send! errors.
use objc2::msg_send_id;
use objc2::rc::Id;
use icrate::Foundation::{NSError, NSString};

fn main() {
    let obj: &NSString;
    let _: Result<Id<_>, Id<NSError>> = unsafe { msg_send_id![obj, a: _] };
}
