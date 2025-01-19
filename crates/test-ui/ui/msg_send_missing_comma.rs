//! Test msg_send! syntax with missing commas.
#![deny(deprecated)]
use objc2::{msg_send, ClassType};
use objc2_foundation::NSString;

fn main() {
    let obj: &NSString = &NSString::new();

    let _: () = unsafe { msg_send![super(obj), a:obj b:obj] };
    let _: () = unsafe { msg_send![super(obj, NSString::class()), a:obj b:obj] };
    let _: () = unsafe { msg_send![obj, a:obj b:obj] };
}
