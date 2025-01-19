//! Test that `msg_send!` with a non-NSError type fails.
use objc2::msg_send;
use objc2::rc::Retained;
use objc2::runtime::NSObject;
use objc2_foundation::NSString;

fn main() {
    let obj = NSObject::new();
    let _: Result<Retained<NSObject>, Retained<NSString>> =
        unsafe { msg_send![&obj, doMoreStuffWithError: _] };
}
