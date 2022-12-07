//! Test underspecified msg_send! errors.
use objc2::{msg_send, msg_send_id};
use objc2::rc::{Id, Shared};
use icrate::Foundation::NSString;

fn main() {
    let obj: &NSString;
    let _: Result<(), _> = unsafe { msg_send![obj, a: _] };

    let _: Result<_, _> = unsafe { msg_send_id![obj, b: _] };
    let _: Result<Id<NSString, Shared>, _> = unsafe { msg_send_id![obj, c: _] };
    let _: Result<Id<NSString, Shared>, Id<_, Shared>> = unsafe { msg_send_id![obj, d: _] };
}
