//! Test compiler output with invalid msg_send_id return values.
use objc2::msg_send_id;
use objc2::rc::{Allocated, Id};
use objc2::runtime::{AnyClass, AnyObject, NSObject};

fn main() {
    let cls: &AnyClass;
    let _: &AnyObject = unsafe { msg_send_id![cls, new] };
    let _: Id<AnyClass> = unsafe { msg_send_id![cls, new] };
    let _: Option<Id<AnyClass>> = unsafe { msg_send_id![cls, new] };

    let _: &AnyObject = unsafe { msg_send_id![cls, alloc] };
    let _: Allocated<AnyClass> = unsafe { msg_send_id![cls, alloc] };
    let _: Id<AnyObject> = unsafe { msg_send_id![cls, alloc] };
    // Earlier design worked like this
    let _: Option<Allocated<AnyObject>> = unsafe { msg_send_id![cls, alloc] };
    // And even earlier design like this
    let _: Id<Allocated<AnyObject>> = unsafe { msg_send_id![cls, alloc] };

    let obj: Allocated<AnyObject>;
    let _: &AnyObject = unsafe { msg_send_id![obj, init] };
    let obj: Allocated<AnyObject>;
    let _: Id<AnyClass> = unsafe { msg_send_id![obj, init] };
    let obj: Allocated<AnyObject>;
    let _: Id<NSObject> = unsafe { msg_send_id![obj, init] };

    let obj: &AnyObject;
    let _: &AnyObject = unsafe { msg_send_id![obj, copy] };

    let _: &AnyObject = unsafe { msg_send_id![obj, description] };
    let _: Option<&AnyObject> = unsafe { msg_send_id![obj, description] };
}
