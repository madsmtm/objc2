//! Test underspecified msg_send! errors.
use icrate::Foundation::NSString;
use objc2::rc::Id;
use objc2::{msg_send, msg_send_id};

fn main() {
    let obj: &NSString;
    let _: Result<(), _> = unsafe { msg_send![obj, a: _] };

    let _: Result<_, _> = unsafe { msg_send_id![obj, b: _] };
    let _: Result<Id<NSString>, _> = unsafe { msg_send_id![obj, c: _] };
    let _: Result<Id<NSString>, Id<_>> = unsafe { msg_send_id![obj, d: _] };
}
