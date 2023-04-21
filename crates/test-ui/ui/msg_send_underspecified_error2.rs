//! Test underspecified msg_send! errors.
use icrate::Foundation::{NSError, NSString};
use objc2::msg_send_id;
use objc2::rc::Id;

fn main() {
    let obj: &NSString;
    let _: Result<Id<_>, Id<NSError>> = unsafe { msg_send_id![obj, a: _] };
}
