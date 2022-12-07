//! Invalid receiver to msg_send![super(obj), ...], missing ClassType impl.
use objc2::msg_send;
use objc2::runtime::{Object, NSObject};

fn main() {
    let obj: &Object;
    let _: () = unsafe { msg_send![super(obj), method] };

    let obj: &NSObject; // impls ClassType, but it's superclass does not
    let _: () = unsafe { msg_send![super(obj), method] };
}
