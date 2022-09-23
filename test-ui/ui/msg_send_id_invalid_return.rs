//! Test compiler output with invalid msg_send_id return values.
use objc2::foundation::NSObject;
use objc2::msg_send_id;
use objc2::rc::{Allocated, Id, Shared};
use objc2::runtime::{Class, Object};

fn main() {
    let cls: &Class;
    let _: &Object = unsafe { msg_send_id![cls, new] };
    let _: Id<Class, Shared> = unsafe { msg_send_id![cls, new] };
    let _: Option<Id<Class, Shared>> = unsafe { msg_send_id![cls, new] };

    let _: &Object = unsafe { msg_send_id![cls, alloc] };
    let _: Allocated<Class> = unsafe { msg_send_id![cls, alloc] };
    let _: Id<Object, Shared> = unsafe { msg_send_id![cls, alloc] };
    // Earlier design worked like this
    let _: Id<Allocated<Object>, Shared> = unsafe { msg_send_id![cls, alloc] };

    let obj: Option<Allocated<Object>>;
    let _: &Object = unsafe { msg_send_id![obj, init] };
    let obj: Option<Allocated<Object>>;
    let _: Id<Class, Shared> = unsafe { msg_send_id![obj, init] };
    let obj: Option<Allocated<Object>>;
    let _: Id<NSObject, Shared> = unsafe { msg_send_id![obj, init] };

    let obj: &Object;
    let _: &Object = unsafe { msg_send_id![obj, copy] };

    let _: &Object = unsafe { msg_send_id![obj, description] };
    let _: Option<&Object> = unsafe { msg_send_id![obj, description] };
}
