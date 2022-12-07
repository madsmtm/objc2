//! Test underspecified msg_send! errors.
use objc2::msg_send_id;
use objc2::rc::{Id, Shared};
use icrate::Foundation::{NSError, NSString};

fn main() {
    let obj: &NSString;
    let _: Result<Id<_, Shared>, Id<NSError, Shared>> = unsafe { msg_send_id![obj, a: _] };
}
