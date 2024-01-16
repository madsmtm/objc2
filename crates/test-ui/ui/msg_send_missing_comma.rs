//! Test msg_send! syntax with missing commas.
#![deny(deprecated)]
use icrate::Foundation::NSString;
use objc2::rc::Id;
use objc2::{msg_send, msg_send_bool, msg_send_id, ClassType};

fn main() {
    let obj: &NSString = &NSString::new();

    let _: () = unsafe { msg_send![super(obj), a:obj b:obj] };
    let _: () = unsafe { msg_send![super(obj, NSString::class()), a:obj b:obj] };
    let _: () = unsafe { msg_send![obj, a:obj b:obj] };

    unsafe { msg_send_bool![obj, c:obj d:obj] };

    let _: Id<NSString> = unsafe { msg_send_id![super(obj), e:obj f:obj] };
    let _: Id<NSString> = unsafe { msg_send_id![super(obj, NSString::class()), e:obj f:obj] };
    let _: Id<NSString> = unsafe { msg_send_id![obj, e:obj f:obj] };
}
