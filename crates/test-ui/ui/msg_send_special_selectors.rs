//! Test special selectors.
use objc2::msg_send;
use objc2::rc::Retained;
use objc2::runtime::NSObject;

fn main() {
    let object: &NSObject;
    let _: Retained<NSObject> = unsafe { msg_send![object, retain] };
    let _: Retained<NSObject> = unsafe { msg_send![object, release] };
    let _: Retained<NSObject> = unsafe { msg_send![object, autorelease] };
    let _: Retained<NSObject> = unsafe { msg_send![object, dealloc] };
}
