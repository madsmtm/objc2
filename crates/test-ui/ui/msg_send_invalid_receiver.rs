//! Test compiler output with invalid msg_send! receivers.
use objc2::msg_send;
use objc2::rc::{Allocated, Retained};
use objc2::runtime::{AnyClass, NSObject};

fn main() {
    let obj: &NSObject;
    let _: Allocated<NSObject> = unsafe { msg_send![obj, alloc] };
    let _: Retained<NSObject> = unsafe { msg_send![obj, init] };

    let cls: &AnyClass;
    let _: Retained<NSObject> = unsafe { msg_send![cls, init] };
    let obj: Retained<NSObject>;
    let _: Retained<NSObject> = unsafe { msg_send![obj, init] };
    let obj: Option<Retained<NSObject>>;
    let _: Retained<NSObject> = unsafe { msg_send![obj, init] };

    let obj: Retained<NSObject>;
    let _: Retained<NSObject> = unsafe { msg_send![obj, new] };
    let obj: Retained<NSObject>;
    let _: Retained<NSObject> = unsafe { msg_send![obj, copy] };
}
