//! Test compiler output with invalid msg_send! return values.
use objc2::msg_send;
use objc2::rc::{Allocated, Retained};
use objc2::runtime::{AnyClass, AnyObject, NSObject};

fn main() {
    let cls: &AnyClass;

    let _: Allocated<AnyClass> = unsafe { msg_send![cls, alloc] };
    let _: Retained<AnyObject> = unsafe { msg_send![cls, alloc] };
    // Earlier design worked like this
    let _: Option<Allocated<AnyObject>> = unsafe { msg_send![cls, alloc] };
    // And even earlier design like this
    let _: Retained<Allocated<AnyObject>> = unsafe { msg_send![cls, alloc] };

    let obj: Allocated<AnyObject>;
    let _: &AnyObject = unsafe { msg_send![obj, init] };
    let obj: Allocated<AnyObject>;
    let _: Retained<AnyClass> = unsafe { msg_send![obj, init] };
    let obj: Allocated<AnyObject>;
    let _: Retained<NSObject> = unsafe { msg_send![obj, init] };
}
